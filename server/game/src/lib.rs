use std::{net::IpAddr, time::Duration};

use async_trait::async_trait;
use data::entities::{account, character};
use moople_net::{
    maple_router_handler,
    service::{
        handler::{MakeServerSessionHandler, MapleServerSessionHandler, MapleSessionHandler},
        resp::{ResponsePacket, Response},
    },
    MapleSession,
};
use moople_packet::{
    proto::{list::MapleIndexListZ8, time::MapleTime, MapleList16},
    DecodePacket, HasOpcode, MaplePacket, MaplePacketReader, MaplePacketWriter,
};
use proto95::{
    game::{
        field::{
            CrcSeed, LogoutGiftConfig, NotificationList, SetFieldCharData, SetFieldResp,
            SetFieldResult,
        },
        MigrateInGameReq,
    },
    id::MapId,
    login::world::{ChannelId, WorldId},
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{
        char::{CharDataAll, CharDataEquipped, CharDataStat, TeleportRockInfo, CharDataHeader, CharDataFlagsAll},
        UpdateScreenSettingReq,
    },
};
use services::{migration::IpIdKey, SharedServices};
use tokio::net::TcpStream;

pub type GameResponse<T> = ResponsePacket<SendOpcodes, T>;
pub type GameResult<T> = Result<GameResponse<T>, anyhow::Error>;

#[derive(Debug, Clone)]
pub struct MakeGameHandler {
    services: SharedServices,
}

impl MakeGameHandler {
    pub fn new(services: SharedServices) -> Self {
        Self { services }
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
    ) -> Result<Self::Handler, Self::Error> {
        let mut handler = GameHandler::from_session(sess, self.services.clone()).await?;
        let pkt = handler.set_field().await?;
        pkt.send(sess).await?;
        Ok(handler)
    }
}

pub struct GameState {
    acc: account::Model,
    char: character::Model,
}

pub struct GameHandler {
    services: services::SharedServices,
    state: GameState,
    addr: IpAddr,
    world_id: WorldId,
    channel_id: ChannelId,
}

impl GameHandler {
    pub async fn from_session(
        session: &mut MapleSession<TcpStream>,
        services: SharedServices,
    ) -> anyhow::Result<Self> {
        let addr = session.peer_addr()?;
        log::info!("Game sess: {}", addr);

        let pkt = session.read_packet().await?;
        log::info!("Migration: {:?}", pkt);
        let mut pr = pkt.into_reader();

        let op = pr.read_opcode::<RecvOpcodes>()?;
        log::info!("New client with opcode: {:?}", op);
        if op != MigrateInGameReq::OPCODE {
            anyhow::bail!("Wrong client hello packet: {op:?}")
        }

        let req = MigrateInGameReq::decode_packet(&mut pr)?;
        dbg!(&req);

        let key = IpIdKey {
            ip: session.peer_addr()?.ip(),
            id: req.char_id,
        };

        dbg!(&key);

        log::info!("Loading ctx");

        let ctx = services
            .migration
            .take(&key)
            .ok_or_else(|| anyhow::format_err!("Unable to find migration context"))?;

        log::info!("Loading acc");
        let acc = services
            .account
            .get(ctx.acc_id as i32)
            .await?
            .ok_or_else(|| anyhow::format_err!("No Account"))?; //TODO claim the account

        let char = services.character.must_get(ctx.char_id as i32).await?;

        log::info!("Session for acc: {} - char: {}", acc.username, char.name);

        Ok(Self {
            services,
            state: GameState { acc, char },
            addr: addr.ip(),
            world_id: 0,   //TODO
            channel_id: 0, //TODO
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
    ) -> Result<(), Self::Error> {
        maple_router_handler!(
            handler,
            GameHandler,
            MapleSession<TcpStream>,
            anyhow::Error,
            GameHandler::handle_default,
            UpdateScreenSettingReq => GameHandler::handle_update_screen_setting
        );

        handler(self, session, packet.into_reader()).await?;

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
    pub async fn handle_default(&mut self, pr: MaplePacketReader<'_>) -> anyhow::Result<()> {
        log::info!("Unhandled packet: {:?}", pr.into_inner());
        Ok(())
    }

    async fn set_field(&mut self) -> GameResult<SetFieldResp> {
        let char = &self.state.char;
        
        let char_data = CharDataAll {
            stat: CharDataStat {
                stat: char.into(),
                friend_max: 30,
                linked_character: None.into(),
            },
            money: char.mesos as u32,
            inv_size: [20; 5],
            equip_ext_slot_expire: MapleTime(0),
            equipped: CharDataEquipped::default(),
            use_inv: MapleIndexListZ8::default(),
            setup_inv: MapleIndexListZ8::default(),
            etc_inv: MapleIndexListZ8::default(),
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

        Ok(SetFieldResp {
            client_option: MapleList16::default(),
            channel_id: self.channel_id as u32,
            old_driver_id: 0,
            unknown_flag_1: 0,
            set_field_result: SetFieldResult::CharData(char_data),
            timestamp: MapleTime::utc_now(),
            extra: 0,
        }
        .into())
    }

    async fn handle_update_screen_setting(
        &mut self,
        req: UpdateScreenSettingReq,
    ) -> anyhow::Result<()> {
        dbg!(&req);
        Ok(())
    }
}
