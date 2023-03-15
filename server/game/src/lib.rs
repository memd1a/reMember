use std::ops::Add;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use std::{net::IpAddr, time::Duration};

use async_trait::async_trait;

use data::entities::character;
use data::services::field::FieldJoinHandle;
use data::services::helper::pool::drop::{DropTypeValue, DropLeaveParam};
use data::services::session::session_data::MoopleSessionHolder;
use data::services::session::session_set::{SharedSessionData, SharedSessionDataRef};
use data::services::session::{ClientKey, MoopleMigrationKey};
use data::services::SharedServices;
use moople_net::service::handler::BroadcastSender;
use moople_net::service::resp::PacketOpcodeExt;
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
use moople_packet::proto::list::MapleIndexList8;

use moople_packet::{
    proto::{
        list::{MapleIndexListZ16, MapleIndexListZ8},
        time::MapleTime,
        MapleList16,
    },
    DecodePacket, HasOpcode, MaplePacket, MaplePacketReader, MaplePacketWriter,
};

use data::services::helper::pool::{Drop, Mob, Npc};

use proto95::game::user::{UserDropMoneyReq, UserDropPickUpReq};

use proto95::id::{FaceId, HairId, ItemId, Skin};
use proto95::shared::char::{AvatarData, PetIds};
use proto95::shared::movement::Movement;
use proto95::shared::{FootholdId, Range2, Vec2};
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
            CharStatChangedResp, CharStatPartial, TeleportRockInfo,
        },
        item::Item,
        UpdateScreenSettingReq,
    },
    stats::PartialFlag,
};
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
    broadcast_tx: BroadcastSender,
    handle: SharedSessionDataRef,
    pos: Vec2,
    fh: FootholdId,
    field: FieldJoinHandle,
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

        let join_field = services
            .field
            .join_field(
                session.char.id,
                handle.clone(),
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
            broadcast_tx,
            pos: Vec2::default(),
            fh: 0,
            handle,
            field: join_field,
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
        );

        handler(self, session, packet.into_reader()).await?;

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

        let char_equipped = CharDataEquipped {
            equipped,
            ..Default::default()
        };

        let char_data = CharDataAll {
            stat: CharDataStat {
                stat: char.into(),
                friend_max: 30,
                linked_character: None.into(),
            },
            money: char.mesos as u32,
            inv_size: [20; 5],
            equip_ext_slot_expire: MapleTime(0),
            equipped: char_equipped,
            use_inv: MapleIndexListZ8::default(),
            setup_inv: MapleIndexListZ8::default(),
            etc_inv: etc,
            cash_inv: MapleIndexListZ8::default(),
            skill_records: MapleList16::default(),
            skill_cooltime: MapleList16::default(),
            quests: MapleList16::default(),
            quests_completed: MapleList16::default(),
            mini_game_records: MapleList16::default(),
            social_records: MapleList16::default(),
            teleport_rock_info: TeleportRockInfo {
                maps: [MapId(0); 5],
                vip_maps: [MapId(10000); 10],
            },
            new_year_cards: MapleList16::default(),
            quest_records_expired: MapleList16::default(),
            /*wildhunterinfo: WildHunterInfo {
                riding_ty_id: 0,
                captured_mobs: [0; 5]
            },*/
            quest_complete_old: MapleList16::default(),
            visitor_quest_log_info: MapleList16::default(),
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
                owner: self.session.char.id as u32,
                pos: self.pos,
                start_pos: self.pos,
                value: DropTypeValue::Mesos(req.money),
            })
            .await?;
        Ok(self.enable_char().into())
    }

    async fn handle_chat_msg(&mut self, req: ChatMsgReq) -> GameResult<UserChatMsgResp> {
        static INITIAL: AtomicI32 = AtomicI32::new(1337);

        let next_id = INITIAL.fetch_add(1, Ordering::SeqCst);

        /*let secondary_stats =  RemoteCharSecondaryStatPartial {
            //shadowpartner: Some(4111002).into(),
            darksight: Some(()).into(),
            curse: Some(1000).into(),
            ..Default::default()
        };



        let secondary_stat = PartialFlag { hdr: (), data: secondary_stats };

        let mut secondary_pw = MaplePacketWriter::default();
        secondary_stat.encode_packet(&mut secondary_pw)?;
        dbg!(&secondary_pw.into_packet().data[..]);

        let avatar = map_char_to_avatar(&self.session.char);



        // Spawn a new user with the name fo the msg
        let pkt = UserEnterFieldResp {
            char_id: next_id as u32,
            user_init_data: UserRemoteInitData{
                level: 30,
                name: req.msg.clone(),
                guild_name: "Eden".to_string(),
                guild_mark: GuildMarkData::default(),
                secondary_stat,
                avatar,
                driver_id: 0,
                passenger_id: 0,
                choco_count: 0,
                active_effect_item: ItemId(0),
                completed_set_item_id: ItemId(0),
                portable_chair: ItemId(0),
                pos: self.pos,
                fh: self.fh,
                show_admin_effects: false,
                pet_infos: MapleIndexListZ::default(),
                taming_mob: TamingMobData::default(),
                mini_room: None.into(),
                ad_board: None.into(),
                couple: None.into(),
                friendship: None.into(),
                marriage: None.into(),
                load_flags: 0,
                new_year_cards: None.into(),
                phase: 0,
                defense_att: 0,
                defense_state: 0,
                job: JobId::Bandit,
                move_action: 0,
            }
        };
        let op = UserEnterFieldResp::OPCODE;*/
        let char_pos = self.pos.add((0, -20).into());
        let char_fh = self.fh;
        let new_mob_id = next_id as u32;

        let msg = req.msg.as_str();

        if msg.starts_with("aggro") {
            self.field
                .assign_mob_controller(SharedSessionDataRef::new(SharedSessionData {
                    broadcast_tx: self.broadcast_tx.clone(),
                }))
                .await?;
        }

        if msg.starts_with("mob") {
            self.field
                .add_mob(Mob {
                    tmpl_id: 1110100,
                    pos: char_pos,
                    fh: char_fh,
                    origin_fh: None,
                })
                .await?;
        } else if msg.starts_with("npc") {
            if let Some(npc) = self
                .field
                .get_meta()
                .life
                .values()
                .find(|life| life._type == "n")
            {
                dbg!(npc);
                let tmpl_id: u32 = npc.id.parse().unwrap();
                self.field
                    .add_npc(Npc {
                        tmpl_id,
                        pos: Vec2::from((npc.x as i16, npc.y as i16)),
                        fh: npc.fh as FootholdId,
                        move_action: 0,
                        range_horz: Range2 {
                            low: npc.rx_0 as i16,
                            high: npc.rx_1 as i16,
                        },
                        enabled: true,
                    })
                    .await?;
            }
        } else {
            let drop_type = if new_mob_id % 2 == 0 {
                DropTypeValue::Item(ItemId::ADVANCED_MONSTER_CRYSTAL_1)
            } else {
                DropTypeValue::Mesos(1_000)
            };

            self.field
                .add_drop(Drop {
                    owner: 0,
                    pos: self.pos,
                    start_pos: self.pos,
                    value: drop_type,
                })
                .await?;
        }

        Ok(UserChatMsgResp {
            char: self.session.char.id as u32,
            is_admin: true,
            msg: format!("MSG: {}", req.msg),
            only_balloon: req.only_balloon,
        }
        .into())
    }

    async fn handle_portal_script(
        &mut self,
        req: UserPortalScriptReq,
    ) -> GameResult<CharStatChangedResp> {
        dbg!(&req);
        Ok(self.enable_char().into())
    }

    async fn handle_field_transfer(
        &mut self,
        req: UserTransferFieldReq,
    ) -> GameResult<SetFieldResp> {
        dbg!(&req);
        let portal = self
            .field
            .get_meta()
            .portal
            .values()
            .find(|p| p.pn == req.portal)
            .ok_or_else(|| anyhow::format_err!("Invalid portal"))?;
        dbg!(portal);

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

pub fn map_char_to_avatar(char: &character::Model) -> AvatarData {
    AvatarData {
        gender: (&char.gender).into(),
        skin: Skin::try_from(char.skin as u8).unwrap(),
        mega: true,
        face: FaceId(char.face as u32),
        hair: HairId(char.hair as u32),
        equips: MapleIndexList8::from(vec![
            (5, ItemId(1040006)),
            (6, ItemId(1060006)),
            (7, ItemId(1072005)),
            (11, ItemId(1322005)),
        ]),
        masked_equips: MapleIndexList8::from(vec![]),
        weapon_sticker_id: ItemId(0),
        pets: PetIds::default(),
    }
}
