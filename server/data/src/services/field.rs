use std::{ops::Deref, sync::Arc};

use dashmap::DashMap;
use proto95::{
    game::{drop::DropId, mob::MobLeaveType},
    id::MapId,
    shared::{FootholdId, Range2, Vec2},
};

use super::{
    character::CharacterID,
    helper::pool::{drop::DropLeaveParam, reactor::Reactor, Drop, Mob, Npc, Pool},
    meta::meta_service::{FieldMeta, MetaService},
    session::session_set::{SessionSet, SharedSessionDataRef},
};

#[derive(Debug)]
pub struct FieldData {
    meta: FieldMeta,
    drop_pool: Pool<Drop>,
    mob_pool: Pool<Mob>,
    npc_pool: Pool<Npc>,
    reactor_pool: Pool<Reactor>,
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
    pub fn new(meta: FieldMeta) -> Self {
        let npcs = meta
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

        let mobs = meta
            .life
            .values()
            .filter(|life| life._type == "m" && life.hide != Some(1))
            .map(|mob| Mob {
                tmpl_id: mob.id.parse().unwrap(),
                pos: Vec2::from((mob.x as i16, mob.y as i16)),
                fh: mob.fh as FootholdId,
                origin_fh: Some(mob.fh as FootholdId),
            });

        let reactors = meta.reactor.values().map(|r| Reactor {
            pos: Vec2::from((r.x as i16, r.y as i16)),
            tmpl_id: r.id.parse().unwrap(),
            state: 0,
        });

        Self {
            meta,
            drop_pool: Pool::new(),
            sessions: SessionSet::new(),
            mob_pool: Pool::from_elems(mobs),
            npc_pool: Pool::from_elems(npcs),
            reactor_pool: Pool::from_elems(reactors),
        }
    }

    pub async fn enter_field(
        &self,
        char_id: CharacterID,
        session: SharedSessionDataRef,
    ) -> anyhow::Result<()> {
        self.sessions.add(char_id, session.clone());
        self.drop_pool.on_enter(session.clone()).await?;
        self.npc_pool.on_enter(session.clone()).await?;
        self.mob_pool.on_enter(session.clone()).await?;
        self.reactor_pool.on_enter(session.clone()).await?;

        Ok(())
    }

    pub fn leave_field(&self, id: CharacterID) {
        self.sessions.remove(id);

        //TODO: broadcast the message without async
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

    pub fn get_meta(&self) -> FieldMeta {
        self.meta
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

        Ok(Arc::new(FieldData::new(field_meta)))
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
        field_id: MapId,
    ) -> anyhow::Result<FieldJoinHandle> {
        let field = self.get_field(field_id)?;
        field.enter_field(char_id, session).await?;

        Ok(FieldJoinHandle {
            field_data: field.clone(),
            id: char_id,
        })
    }
}
