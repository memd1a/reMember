pub mod repl;
pub mod state;

use std::sync::Arc;

use std::{net::IpAddr, time::Duration};

use async_trait::async_trait;

use data::services::field::FieldJoinHandle;
use data::services::helper::pool::drop::{DropLeaveParam, DropTypeValue};
use data::services::session::session_data::MoopleSessionHolder;
use data::services::session::session_set::{SharedSessionData, SharedSessionDataRef};
use data::services::session::{ClientKey, MoopleMigrationKey};
use data::services::SharedServices;
use moople_net::service::handler::BroadcastSender;
use moople_net::service::packet_buffer::PacketBuffer;
use moople_net::service::resp::PacketOpcodeExt;
use moople_net::SessionTransport;
use moople_net::{
    maple_router_handler,
    service::{
        handler::{
            MakeServerSessionHandler, MapleServerSessionHandler, MapleSessionHandler, SessionError,
        },
        resp::{MigrateResponse, ResponsePacket},
    },
    MapleSession,
};

use moople_packet::EncodePacket;

use moople_packet::proto::list::MapleIndexListZ;
use moople_packet::proto::time::MapleExpiration;
use moople_packet::{
    proto::{
        list::{MapleIndexListZ16, MapleIndexListZ8},
        time::MapleTime,
        MapleList16,
    },
    DecodePacket, HasOpcode, MaplePacket, MaplePacketReader, MaplePacketWriter,
};

use data::services::helper::pool::Drop;

use proto95::game::mob::{MobMoveCtrlAckResp, MobMoveReq};
use proto95::game::user::{
    ChangeSkillRecordResp, UpdatedSkillRecord, UserDropMoneyReq, UserDropPickUpReq,
    UserMeleeAttackReq, UserSkillUpReq,
};

use proto95::shared::char::{SkillInfo, TeleportRockInfo};
use proto95::shared::movement::Movement;
use proto95::shared::{FootholdId, Vec2};
use proto95::{
    game::{
        chat::{ChatMsgReq, UserChatMsgResp},
        field::{
            CrcSeed, LogoutGiftConfig, NotificationList, SetFieldCharData, SetFieldResp,
            SetFieldResult,
        },
        friend::{FriendList, FriendResultResp},
        keymaps::FuncKeyMapInitResp,
        user::{UserMoveReq, UserPortalScriptReq, UserTransferFieldReq},
        BroadcastMessageResp, ClaimSvrStatusChangedResp, CtxSetGenderResp, MigrateCommandResp,
        MigrateInGameReq, TransferChannelReq,
    },
    id::MapId,
    login::world::{ChannelId, WorldId},
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{
        char::{
            CharDataAll, CharDataEquipped, CharDataFlagsAll, CharDataHeader, CharDataStat,
            CharStatChangedResp, CharStatPartial,
        },
        item::Item,
        UpdateScreenSettingReq,
    },
    stats::PartialFlag,
};
use repl::GameRepl;
use tokio::net::TcpStream;

pub type GameResponse<T> = ResponsePacket<SendOpcodes, T>;
pub type GameResult<T> = Result<GameResponse<T>, anyhow::Error>;

#[derive(Debug, Clone)]
pub struct MakeGameHandler {
    services: SharedServices,
    channel_id: ChannelId,
    world_id: WorldId,
}

impl MakeGameHandler {
    pub fn new(services: SharedServices, channel_id: ChannelId, world_id: WorldId) -> Self {
        Self {
            services,
            channel_id,
            world_id,
        }
    }
}

#[async_trait::async_trait]
impl MakeServerSessionHandler for MakeGameHandler {
    type Transport = TcpStream;

    type Error = anyhow::Error;

    type Handler = GameHandler;

    async fn make_handler(
        &mut self,
        sess: &mut moople_net::MapleSession<Self::Transport>,
        broadcast_tx: BroadcastSender,
    ) -> Result<Self::Handler, Self::Error> {
        let mut handler = GameHandler::from_session(
            sess,
            self.services.clone(),
            self.channel_id,
            self.world_id,
            broadcast_tx,
        )
        .await?;
        sess.send_packet(handler.set_field()).await?;
        handler.init_char(sess).await?;

        Ok(handler)
    }
}

pub struct GameHandler {
    session: MoopleSessionHolder,
    channel_id: ChannelId,
    world_id: WorldId,
    services: SharedServices,
    addr: IpAddr,
    client_key: ClientKey,
    handle: SharedSessionDataRef,
    pos: Vec2,
    fh: FootholdId,
    field: FieldJoinHandle,
    repl: GameRepl,
    packet_buf: PacketBuffer,
}

impl GameHandler {
    pub async fn from_session(
        net_session: &mut MapleSession<TcpStream>,
        services: SharedServices,
        channel_id: ChannelId,
        world_id: WorldId,
        broadcast_tx: BroadcastSender,
    ) -> anyhow::Result<Self> {
        let addr = net_session.peer_addr()?;
        log::info!("Game sess: {} - waiting abit for session to be free", addr);

        let pkt = net_session.read_packet().await?;
        log::info!("Migration: {:?}", pkt);
        let mut pr = pkt.into_reader();

        let op = pr.read_opcode::<RecvOpcodes>()?;
        log::info!("New client with opcode: {:?}", op);
        if op != MigrateInGameReq::OPCODE {
            anyhow::bail!("Wrong client hello packet: {op:?}")
        }

        let req = MigrateInGameReq::decode_packet(&mut pr)?;
        let addr = net_session.peer_addr()?.ip();

        dbg!(MoopleMigrationKey::new(req.client_key, addr));

        let session = services
            .session_manager
            .claim_migration_session(MoopleMigrationKey::new(req.client_key, addr))
            .await?;

        log::info!(
            "Session for acc: {} - char: {}",
            session.acc.username,
            session.char.name
        );

        let handle = Arc::new(SharedSessionData {
            broadcast_tx: broadcast_tx.clone(),
        });

        let mut packet_buf = PacketBuffer::new();

        let join_field = services
            .field
            .join_field(
                session.char.id,
                handle.clone(),
                &mut packet_buf,
                MapId(session.char.map_id as u32),
            )
            .await?;

        Ok(Self {
            session,
            services,
            channel_id,
            world_id,
            addr,
            client_key: req.client_key,
            pos: Vec2::default(),
            fh: 0,
            handle,
            field: join_field,
            packet_buf,
            repl: GameRepl::new(),
        })
    }
}

#[async_trait]
impl MapleSessionHandler for GameHandler {
    type Transport = TcpStream;
    type Error = anyhow::Error;

    async fn handle_packet(
        &mut self,
        packet: MaplePacket,
        session: &mut moople_net::MapleSession<Self::Transport>,
    ) -> Result<(), SessionError<Self::Error>> {
        maple_router_handler!(
            handler,
            GameHandler,
            MapleSession<TcpStream>,
            SessionError<anyhow::Error>,
            GameHandler::handle_default,
            UpdateScreenSettingReq => GameHandler::handle_update_screen_setting,
            ChatMsgReq => GameHandler::handle_chat_msg,
            UserMoveReq => GameHandler::handle_movement,
            UserPortalScriptReq => GameHandler::handle_portal_script,
            UserTransferFieldReq => GameHandler::handle_field_transfer,
            TransferChannelReq => GameHandler::handle_channel_transfer,
            UserDropPickUpReq => GameHandler::handle_drop_pick_up,
            UserDropMoneyReq => GameHandler::handle_drop_money,
            MobMoveReq => GameHandler::handle_mob_move,
            UserMeleeAttackReq => GameHandler::handle_melee_attack,
            UserSkillUpReq => GameHandler::handle_skill_up
        );

        handler(self, session, packet.into_reader()).await?;
        self.flush_packet_buf(session)
            .await
            .map_err(SessionError::Session)?;

        Ok(())
    }

    async fn finish(self, is_migrating: bool) -> Result<(), SessionError<Self::Error>> {
        log::info!("Finishing game session...");
        if is_migrating {
            self.services
                .session_manager
                .migrate_session(
                    MoopleMigrationKey::new(self.client_key, self.addr),
                    self.session,
                )
                .map_err(SessionError::Session)?;
        }
        Ok(())
    }
}

impl MapleServerSessionHandler for GameHandler {
    fn get_ping_interval() -> std::time::Duration {
        Duration::from_secs(30)
    }

    fn get_ping_packet(&mut self) -> Result<MaplePacket, Self::Error> {
        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(SendOpcodes::AliveReq);
        Ok(pw.into_packet())
    }
}

impl GameHandler {
    async fn handle_skill_up(&mut self, req: UserSkillUpReq) -> GameResult<ChangeSkillRecordResp> {
        dbg!(&req);
        Ok(ChangeSkillRecordResp {
            reset_excl: true,
            skill_records: vec![UpdatedSkillRecord {
                id: req.skill_id,
                level: 1,
                master_level: 0,
                expiration: MapleExpiration::never(),
            }]
            .into(),
            updated_secondary_stat: false,
        }
        .into())
    }

    async fn flush_packet_buf<Trans: SessionTransport + Unpin>(
        &mut self,
        sess: &mut MapleSession<Trans>,
    ) -> anyhow::Result<()> {
        sess.send_packet_buffer(&self.packet_buf).await?;
        self.packet_buf.clear();

        Ok(())
    }

    pub fn enable_char(&mut self) -> CharStatChangedResp {
        CharStatChangedResp {
            excl: true,
            stats: PartialFlag {
                hdr: (),
                data: CharStatPartial {
                    level: Some(50).into(),
                    ..CharStatPartial::default()
                },
            },
            secondary_stat: false,
            battle_recovery: false,
        }
    }

    pub async fn handle_default(
        &mut self,
        _op: RecvOpcodes,
        pr: MaplePacketReader<'_>,
    ) -> anyhow::Result<()> {
        log::info!("Unhandled packet: {:?}", pr.into_inner());
        Ok(())
    }

    async fn init_char(&mut self, sess: &mut MapleSession<TcpStream>) -> anyhow::Result<()> {
        sess.send_packet(FriendResultResp::Reset3(FriendList::empty()))
            .await?;
        sess.send_packet(FuncKeyMapInitResp::default_map()).await?;
        sess.send_packet(ClaimSvrStatusChangedResp { connected: true })
            .await?;
        sess.send_packet(CtxSetGenderResp {
            gender: (&self.session.char.gender).into(),
        })
        .await?;

        sess.send_packet(BroadcastMessageResp::PinkMessage("Hello".to_string()))
            .await?;

        sess.send_packet(self.enable_char()).await?;
        self.flush_packet_buf(sess).await?;

        Ok(())
    }

    fn set_field(&mut self) -> SetFieldResp {
        let char = &self.session.char;

        let equipped: MapleIndexListZ16<Item> = self
            .session
            .inv
            .equipped
            .iter()
            .map(|(slot, item)| (slot as u16, Item::Equip(item.item.as_ref().into())))
            .collect();

        let etc: MapleIndexListZ8<Item> = self
            .session
            .inv
            .etc
            .iter()
            .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.item.as_ref().into())))
            .collect();

        // TODO cash slots
        let invsize = [
            char.equip_slots as u8,
            char.use_slots as u8,
            char.setup_slots as u8,
            char.etc_slots as u8,
            char.equip_slots as u8,
        ];

        let char_equipped = CharDataEquipped {
            equipped,
            ..Default::default()
        };

        let skill_records: MapleList16<SkillInfo> = self
            .session
            .skills
            .iter()
            .map(|(id, skill)| SkillInfo {
                id: *id,
                level: skill.skill_level as u32,
                expiration: skill.expires_at.into(),
                master_level: skill.master_level as u32,
            })
            .collect();

        let char_data = CharDataAll {
            stat: CharDataStat {
                stat: char.into(),
                friend_max: 30,
                linked_character: None.into(),
            },
            money: char.mesos as u32,
            invsize,
            equipextslotexpiration: MapleExpiration::never(),
            equipped: char_equipped,
            useinv: MapleIndexListZ::default(),
            setupinv: MapleIndexListZ::default(),
            etcinv: etc,
            cashinv: MapleIndexListZ::default(),
            skillrecords: skill_records,
            skllcooltime: MapleList16::default(),
            quests: MapleList16::default(),
            questscompleted: MapleList16::default(),
            minigamerecords: MapleList16::default(),
            socialrecords: MapleList16::default(),
            teleportrockinfo: TeleportRockInfo::default(),
            newyearcards: MapleList16::default(),
            questrecordsexpired: MapleList16::default(),
            questcompleteold: MapleList16::default(),
            visitorquestloginfo: MapleList16::default(),
        };

        let char_data = SetFieldCharData {
            notifications: NotificationList::default(),
            seed: CrcSeed {
                s1: 1,
                s2: 2,
                s3: 3,
            },
            logout_gift_config: LogoutGiftConfig {
                predict_quit: 0,
                gift_commodity_id: [0; 3],
            },
            char_data_hdr: CharDataHeader {
                combat_orders: 0,
                extra_data: None.into(),
            },
            char_data,
            char_data_flags: CharDataFlagsAll,
        };

        SetFieldResp {
            client_option: MapleList16::default(),
            channel_id: self.channel_id as u32,
            old_driver_id: 0,
            unknown_flag_1: 0,
            set_field_result: SetFieldResult::CharData(char_data),
            timestamp: MapleTime::utc_now(),
            extra: 0,
        }
    }

    async fn handle_update_screen_setting(
        &mut self,
        req: UpdateScreenSettingReq,
    ) -> anyhow::Result<()> {
        dbg!(&req);
        Ok(())
    }

    async fn handle_melee_attack(&mut self, req: UserMeleeAttackReq) -> anyhow::Result<()> {
        dbg!(&req);
        for target in req.targets {
            let dmg = target.hits.iter().sum::<u32>();
            self.field
                .attack_mob(
                    target.mob_id,
                    dmg,
                    self.session.char.id as u32,
                    &mut self.packet_buf,
                )
                .await?;
        }

        Ok(())
    }

    async fn handle_drop_pick_up(
        &mut self,
        req: UserDropPickUpReq,
    ) -> GameResult<CharStatChangedResp> {
        dbg!(&req);
        self.field
            .remove_drop(
                req.drop_id,
                DropLeaveParam::UserPickup(self.session.char.id as u32),
            )
            .await?;
        Ok(self.enable_char().into())
    }

    async fn handle_drop_money(
        &mut self,
        req: UserDropMoneyReq,
    ) -> GameResult<CharStatChangedResp> {
        self.field
            .add_drop(Drop {
                owner: proto95::game::drop::DropOwner::User(self.session.char.id as u32),
                pos: self.pos,
                start_pos: self.pos,
                value: DropTypeValue::Mesos(req.money),
                quantity: 1,
            })
            .await?;
        Ok(self.enable_char().into())
    }

    async fn handle_chat_msg(&mut self, req: ChatMsgReq) -> anyhow::Result<()> {
        let admin = false;
        if let Some(s) = req.msg.strip_prefix('@') {
            let repl_resp = self.handle_repl(s).await?;
            let Some(msg) = repl_resp else {
                return Ok(())
            };
            let resp = UserChatMsgResp {
                char: self.session.char.id as u32,
                is_admin: admin,
                msg,
                only_balloon: false,
            };
            let mut pw = MaplePacketWriter::default();
            pw.write_opcode(UserChatMsgResp::OPCODE);
            resp.encode_packet(&mut pw)?;

            self.handle.broadcast_tx.send(pw.into_packet()).await?;
        } else {
            self.field
                .add_chat(UserChatMsgResp {
                    char: self.session.char.id as u32,
                    is_admin: admin,
                    msg: req.msg,
                    only_balloon: req.only_balloon,
                })
                .await?;
        };
        Ok(())
    }

    async fn handle_mob_move(&mut self, req: MobMoveReq) -> GameResult<MobMoveCtrlAckResp> {
        self.field.update_mob_pos(&req).await?;

        Ok(MobMoveCtrlAckResp {
            id: req.id,
            ctrl_sn: req.ctrl_sn,
            next_atk_possible: false,
            mp: 0,
            skill_id: 0,
            slv: 0,
        }
        .into())
    }

    async fn handle_portal_script(
        &mut self,
        _req: UserPortalScriptReq,
    ) -> GameResult<CharStatChangedResp> {
        Ok(self.enable_char().into())
    }

    async fn handle_field_transfer(
        &mut self,
        req: UserTransferFieldReq,
    ) -> GameResult<SetFieldResp> {
        let portal = self
            .field
            .get_meta()
            .portal
            .values()
            .find(|p| p.pn == req.portal)
            .ok_or_else(|| anyhow::format_err!("Invalid portal"))?;

        // TODO(!) tm should be an option as mapid 999999 is invalid
        let map_id = MapId(portal.tm as u32);
        self.session.char.map_id = map_id.0 as i32;
        self.session.char.spawn_point = self
            .services
            .meta
            .get_field_data(map_id)
            .unwrap()
            .portal
            .iter()
            .find(|(_, p)| p.pn == portal.tn)
            .map(|(id, _)| *id as u8)
            .unwrap_or(0) as i32;

        self.field = self
            .services
            .field
            .join_field(
                self.session.char.id,
                self.handle.clone(),
                &mut self.packet_buf,
                MapId(self.session.char.map_id as u32),
            )
            .await?;

        let transfer_field = self.set_field();
        Ok(transfer_field.into())
    }

    async fn handle_movement(&mut self, req: UserMoveReq) -> anyhow::Result<()> {
        self.pos = req.move_path.pos;
        let last_fh = req
            .move_path
            .moves
            .items
            .iter()
            .rev()
            .find_map(|mv| match mv {
                Movement::Normal(d) => Some(d.foothold),
                _ => None,
            });

        if let Some(fh) = last_fh {
            self.fh = fh;
        }
        log::info!("User move req @ {:?}", req.move_path.pos);
        Ok(())
    }

    async fn handle_channel_transfer(
        &mut self,
        req: TransferChannelReq,
    ) -> anyhow::Result<MigrateResponse<ResponsePacket<SendOpcodes, MigrateCommandResp>>> {
        log::info!("Transfer channel: {:?}", req);
        let addr = self
            .services
            .server_info
            .get_channel_addr(self.world_id, req.channel_id as ChannelId)?;

        let pkt: ResponsePacket<_, _> = MigrateCommandResp {
            unknown: true,
            addr: addr.try_into()?,
        }
        .into_response(MigrateCommandResp::OPCODE);

        Ok(MigrateResponse(pkt))
    }
}