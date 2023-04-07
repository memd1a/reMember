use std::{ops::Deref, sync::Arc};

use dashmap::DashMap;
use moople_net::service::{packet_buffer::PacketBuffer, session_svc::SharedSessionHandle};
use proto95::{
    game::{
        chat::UserChatMsgResp,
        drop::DropId,
        mob::{MobLeaveType, MobMoveReq},
        user::UserMoveReq,
        ObjectId,
    },
    id::MapId,
    shared::{char::AvatarData, FootholdId, Range2, Vec2},
};
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};

use super::{
    data::character::CharacterID,
    helper::pool::{drop::DropLeaveParam, reactor::Reactor, user::User, Drop, Mob, Npc, Pool},
    meta::{
        fh_tree::FhTree,
        meta_service::{FieldMeta, MetaService},
    },
    session::MoopleSessionSet,
};

#[derive(Debug)]
pub struct FieldData {
    _meta: &'static MetaService,
    field_meta: FieldMeta,
    field_fh: &'static FhTree,
    drop_pool: Pool<Drop>,
    mob_pool: Pool<Mob>,
    npc_pool: Pool<Npc>,
    reactor_pool: Pool<Reactor>,
    user_pool: Pool<User>,
    sessions: MoopleSessionSet,
}

pub struct FieldJoinHandle {
    field_data: Arc<FieldData>,
    char_id: CharacterID,
}

impl Deref for FieldJoinHandle {
    type Target = FieldData;

    fn deref(&self) -> &Self::Target {
        &self.field_data
    }
}

impl std::ops::Drop for FieldJoinHandle {
    fn drop(&mut self) {
        self.field_data.leave_field(self.char_id)
    }
}

impl FieldData {
    pub fn new(
        meta: &'static MetaService,
        field_meta: FieldMeta,
        fh_meta: &'static FhTree,
    ) -> Self {
        let npcs = field_meta
            .life
            .values()
            .filter(|life| life._type == "n")
            .map(|npc| Npc {
                tmpl_id: npc.id.parse().unwrap(),
                pos: Vec2::from((npc.x as i16, npc.y as i16)),
                fh: npc.fh as FootholdId,
                move_action: 0,
                range_horz: Range2 {
                    low: npc.rx_0 as i16,
                    high: npc.rx_1 as i16,
                },
                enabled: true,
            });

        let mobs = field_meta
            .life
            .values()
            .filter(|life| life._type == "m" && life.hide != Some(1))
            .map(|mob| {
                let tmpl_id = mob.id.parse().unwrap();
                let meta = meta.get_mob_data(tmpl_id).unwrap();
                Mob {
                    meta,
                    tmpl_id,
                    pos: Vec2::from((mob.x as i16, mob.y as i16)),
                    fh: mob.fh as FootholdId,
                    origin_fh: Some(mob.fh as FootholdId),
                    hp: meta.max_hp,
                    perc: 100,
                }
            });

        let reactors = field_meta.reactor.values().map(|r| Reactor {
            pos: Vec2::from((r.x as i16, r.y as i16)),
            tmpl_id: r.id.parse().unwrap(),
            state: 0,
        });

        Self {
            _meta: meta,
            field_meta,
            field_fh: fh_meta,
            drop_pool: Pool::new(meta),
            sessions: MoopleSessionSet::new(),
            mob_pool: Pool::from_elems(meta, mobs),
            npc_pool: Pool::from_elems(meta, npcs),
            reactor_pool: Pool::from_elems(meta, reactors),
            user_pool: Pool::new(meta),
        }
    }

    pub async fn enter_field(
        &self,
        char_id: CharacterID,
        mut session: SharedSessionHandle,
        avatar_data: AvatarData,
    ) -> anyhow::Result<()> {
        self.sessions.add(char_id, session.clone());
        self.user_pool.add(
            User {
                char_id: char_id as u32,
                pos: Vec2::from((0, 0)),
                fh: 1,
                avatar_data,
            },
            &self.sessions,
        )?;
        let mut buf = PacketBuffer::new();
        self.user_pool.on_enter(&mut buf)?;
        self.drop_pool.on_enter(&mut buf)?;
        self.npc_pool.on_enter(&mut buf)?;
        self.mob_pool.on_enter(&mut buf)?;
        self.reactor_pool.on_enter(&mut buf)?;

        session.try_send_buf(&buf)?;

        Ok(())
    }

    pub fn leave_field(&self, id: CharacterID) {
        self.sessions.remove(id);
        self.user_pool
            .remove(id as u32, (), &self.sessions)
            .expect("Must remove user");
    }

    pub fn add_user(&self, user: User) -> anyhow::Result<()> {
        self.user_pool.add(user, &self.sessions)?;
        Ok(())
    }

    pub fn remove_user(&self, id: CharacterID) -> anyhow::Result<()> {
        self.user_pool.remove(id as u32, (), &self.sessions)?;
        Ok(())
    }

    pub fn add_npc(&self, npc: Npc) -> anyhow::Result<()> {
        self.npc_pool.add(npc, &self.sessions)?;
        Ok(())
    }

    pub fn remove_npc(&self, id: u32, param: ()) -> anyhow::Result<()> {
        self.npc_pool.remove(id, param, &self.sessions)?;
        Ok(())
    }

    pub async fn add_mob(&self, drop: Mob) -> anyhow::Result<()> {
        self.mob_pool.add(drop, &self.sessions)?;
        Ok(())
    }

    pub fn remove_mob(&self, id: u32, param: MobLeaveType) -> anyhow::Result<()> {
        self.mob_pool.remove(id, param, &self.sessions)?;
        Ok(())
    }

    pub fn update_user_pos(&self, movement: UserMoveReq, id: CharacterID) -> anyhow::Result<()> {
        let last_pos_fh = movement.move_path.get_last_pos_fh();

        if let Some((pos, fh)) = last_pos_fh {
            //TODO post mob state to msg state here
            self.user_pool.update(id as u32, |usr| {
                usr.pos = pos;
                usr.fh = fh.unwrap_or(usr.fh);
            });
        }

        self.user_pool.user_move(id, movement, &self.sessions)?;

        Ok(())
    }

    pub fn update_mob_pos(
        &self,
        movement: MobMoveReq,
        controller: CharacterID,
    ) -> anyhow::Result<()> {
        let id = movement.id;
        let last_pos_fh = movement.move_path.path.get_last_pos_fh();

        if let Some((pos, fh)) = last_pos_fh {
            //TODO post mob state to msg state here
            self.mob_pool.update(id, |mob| {
                mob.pos = pos;
                mob.fh = fh.unwrap_or(mob.fh);
            });
        }

        self.mob_pool
            .mob_move(movement.id, movement, controller, &self.sessions)?;

        Ok(())
    }

    pub fn add_drop(&self, drop: Drop) -> anyhow::Result<()> {
        self.drop_pool.add(drop, &self.sessions)?;
        Ok(())
    }

    pub fn remove_drop(&self, id: DropId, param: DropLeaveParam) -> anyhow::Result<()> {
        self.drop_pool.remove(id, param, &self.sessions)?;
        Ok(())
    }

    pub fn assign_mob_controller(&self, session: SharedSessionHandle) -> anyhow::Result<()> {
        self.mob_pool.assign_controller(session)?;
        Ok(())
    }

    pub fn add_chat(&self, chat: UserChatMsgResp) -> anyhow::Result<()> {
        self.sessions.broadcast_pkt(chat, -1)?;
        Ok(())
    }

    pub async fn attack_mob(
        &self,
        id: ObjectId,
        dmg: u32,
        attacker: CharacterID,
        session: &mut SharedSessionHandle,
    ) -> anyhow::Result<()> {
        let mut buf = PacketBuffer::new();
        let killed = self
            .mob_pool
            .attack_mob(attacker, id, dmg, &mut buf, &self.sessions)?;
        session.try_send_buf(&buf)?;

        if killed {
            let mob = self
                .mob_pool
                .remove(id, MobLeaveType::Etc(()), &self.sessions)?;

            let fh = self
                .field_fh
                .get_foothold_below((mob.pos.x as f32, mob.pos.y as f32 - 20.).into());

            self.drop_pool
                .add_mob_drops(mob.tmpl_id, mob.pos, fh, attacker, &self.sessions)?;
        }

        Ok(())
    }

    pub fn get_meta(&self) -> FieldMeta {
        self.field_meta
    }
}

pub enum FieldMessage {
    UserEnter(User, RpcReplyPort<FieldJoinHandle>),
    UserLeave(CharacterID),
}

pub struct FieldActor;

#[async_trait::async_trait]
impl Actor for FieldActor {
    type Msg = FieldMessage;
    type State = FieldData;
    type Arguments = FieldData;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self>,
        field_data: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(field_data)
    }

    // This is our main message handler
    async fn handle(
        &self,
        _myself: ActorRef<Self>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            FieldMessage::UserEnter(user, reply) => {
                state.add_user(user);
            }
            FieldMessage::UserLeave(id) => state.remove_user(id)?,
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct FieldService {
    fields: DashMap<MapId, Arc<FieldData>>,
    meta: &'static MetaService,
}

impl FieldService {
    pub fn new(meta: &'static MetaService) -> Self {
        Self {
            fields: DashMap::new(),
            meta,
        }
    }

    fn create_field(&self, field_id: MapId) -> anyhow::Result<Arc<FieldData>> {
        let field_meta = self
            .meta
            .get_field_data(field_id)
            .ok_or_else(|| anyhow::format_err!("Invalid field id: {field_id:?}"))?;

        let field_fh = self.meta.get_field_fh_data(field_id).unwrap();

        Ok(Arc::new(FieldData::new(self.meta, field_meta, field_fh)))
    }

    pub fn get_field(&self, field_id: MapId) -> anyhow::Result<Arc<FieldData>> {
        Ok(self
            .fields
            .entry(field_id)
            .or_try_insert_with(|| self.create_field(field_id))?
            .clone())
    }

    pub async fn join_field(
        &self,
        char_id: CharacterID,
        avatar_data: AvatarData,
        session: SharedSessionHandle,
        field_id: MapId,
    ) -> anyhow::Result<FieldJoinHandle> {
        let field = self.get_field(field_id)?;
        field.enter_field(char_id, session, avatar_data).await?;

        Ok(FieldJoinHandle {
            field_data: field.clone(),
            char_id,
        })
    }
}
