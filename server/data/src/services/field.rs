use std::{ops::Deref, sync::Arc};

use dashmap::DashMap;
use moople_net::service::packet_buffer::PacketBuffer;
use proto95::{
    game::{
        chat::UserChatMsgResp,
        drop::DropId,
        mob::{MobLeaveType, MobMoveReq},
        ObjectId,
    },
    id::MapId,
    shared::{char::CharacterId, FootholdId, Range2, Vec2, movement::Movement},
};

use super::{
    character::CharacterID,
    helper::pool::{drop::DropLeaveParam, reactor::Reactor, user::User, Drop, Mob, Npc, Pool},
    meta::meta_service::{FieldMeta, MetaService},
    session::session_set::{SessionSet, SharedSessionDataRef},
};

#[derive(Debug)]
pub struct FieldData {
    meta: &'static MetaService,
    field_meta: FieldMeta,
    drop_pool: Pool<Drop>,
    mob_pool: Pool<Mob>,
    npc_pool: Pool<Npc>,
    reactor_pool: Pool<Reactor>,
    user_pool: Pool<User>,
    sessions: SessionSet,
}

pub struct FieldJoinHandle {
    field_data: Arc<FieldData>,
    id: CharacterID,
}

impl Deref for FieldJoinHandle {
    type Target = FieldData;

    fn deref(&self) -> &Self::Target {
        &self.field_data
    }
}

impl std::ops::Drop for FieldJoinHandle {
    fn drop(&mut self) {
        self.field_data.leave_field(self.id)
    }
}

impl FieldData {
    pub fn new(meta: &'static MetaService, field_meta: FieldMeta) -> Self {
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
            meta,
            field_meta,
            drop_pool: Pool::new(meta),
            sessions: SessionSet::new(),
            mob_pool: Pool::from_elems(meta, mobs),
            npc_pool: Pool::from_elems(meta, npcs),
            reactor_pool: Pool::from_elems(meta, reactors),
            user_pool: Pool::new(meta),
        }
    }

    pub async fn enter_field(
        &self,
        char_id: CharacterID,
        session: SharedSessionDataRef,
        buf: &mut PacketBuffer,
    ) -> anyhow::Result<()> {
        self.sessions.add(char_id, session);
        /*self.user_pool
        .add(
            User {
                char_id: char_id as u32,
                pos: Vec2::from((0, 0)),
                fh: 1,
            },
            &self.sessions,
        )
        .await?;*/
        self.user_pool.on_enter(buf).await?;
        self.drop_pool.on_enter(buf).await?;
        self.npc_pool.on_enter(buf).await?;
        self.mob_pool.on_enter(buf).await?;
        self.reactor_pool.on_enter(buf).await?;

        Ok(())
    }

    pub fn leave_field(&self, id: CharacterID) {
        self.sessions.remove(id);
        //TODO: self.user_pool.remove(id as u32, (), &self.sessions);
        //TODO: broadcast the message without async
    }

    pub async fn add_user(&self, user: User) -> anyhow::Result<()> {
        self.user_pool.add(user, &self.sessions).await?;
        Ok(())
    }

    pub async fn remove_user(&self, id: CharacterId) -> anyhow::Result<()> {
        self.user_pool.remove(id, (), &self.sessions).await?;
        Ok(())
    }

    pub async fn add_npc(&self, npc: Npc) -> anyhow::Result<()> {
        self.npc_pool.add(npc, &self.sessions).await?;
        Ok(())
    }

    pub async fn remove_npc(&self, id: u32, param: ()) -> anyhow::Result<()> {
        self.npc_pool.remove(id, param, &self.sessions).await?;
        Ok(())
    }

    pub async fn add_mob(&self, drop: Mob) -> anyhow::Result<()> {
        self.mob_pool.add(drop, &self.sessions).await?;
        Ok(())
    }

    pub async fn remove_mob(&self, id: u32, param: MobLeaveType) -> anyhow::Result<()> {
        self.mob_pool.remove(id, param, &self.sessions).await?;
        Ok(())
    }

    pub async fn update_mob_pos(&self, movement: &MobMoveReq) -> anyhow::Result<()> {
        let id = movement.id;
        let last_movement = movement
            .move_path
            .path
            .moves
            .items
            .iter()
            .filter_map(|movement| match movement {
                Movement::Normal(mv) => Some(mv),
                _ => None,
            })
            .last();

        if let Some(mv) = last_movement {
            //TODO post mob state to msg state here
            self.mob_pool.update(id, |mob| {
                mob.pos = mv.p;
                mob.fh = mv.foothold;
            }).await;
        }

        Ok(())
    }

    pub async fn add_drop(&self, drop: Drop) -> anyhow::Result<()> {
        self.drop_pool.add(drop, &self.sessions).await?;
        Ok(())
    }

    pub async fn remove_drop(&self, id: DropId, param: DropLeaveParam) -> anyhow::Result<()> {
        self.drop_pool.remove(id, param, &self.sessions).await?;
        Ok(())
    }

    pub async fn assign_mob_controller(&self, session: SharedSessionDataRef) -> anyhow::Result<()> {
        self.mob_pool.assign_controller(session).await?;
        Ok(())
    }

    pub async fn add_chat(&self, chat: UserChatMsgResp) -> anyhow::Result<()> {
        self.sessions.broadcast_pkt(chat, -1).await?;
        Ok(())
    }

    pub async fn attack_mob(
        &self,
        id: ObjectId,
        dmg: u32,
        attacker: CharacterId,
        buf: &mut PacketBuffer,
    ) -> anyhow::Result<()> {
        let killed = self.mob_pool.attack_mob(id, dmg, buf).await?;

        if killed {
            let mob = self
                .mob_pool
                .remove(id, MobLeaveType::Etc(()), &self.sessions)
                .await?;
            self.drop_pool
                .add_mob_drops(mob.tmpl_id, mob.pos, attacker, &self.sessions)
                .await?;
        }

        Ok(())
    }

    pub fn get_meta(&self) -> FieldMeta {
        self.field_meta
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

        Ok(Arc::new(FieldData::new(&self.meta, field_meta)))
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
        session: SharedSessionDataRef,
        buf: &mut PacketBuffer,
        field_id: MapId,
    ) -> anyhow::Result<FieldJoinHandle> {
        let field = self.get_field(field_id)?;
        field.enter_field(char_id, session, buf).await?;

        Ok(FieldJoinHandle {
            field_data: field.clone(),
            id: char_id,
        })
    }
}
