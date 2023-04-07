pub mod config;
pub mod login_state;

use std::{net::IpAddr, time::Duration};

use async_trait::async_trait;
use config::LoginConfig;
use data::services::data::account::AccountServiceError;
use data::services::data::character::{CharacterCreateDTO, CharacterID, ItemStarterSet};
use data::services::session::MoopleMigrationKey;
use data::{entities::character, services};
use login_state::LoginState;
use moople_net::service::handler::SessionHandleResult;
use moople_net::service::resp::PongResponse;
use moople_net::{
    maple_router_handler,
    service::{
        handler::{MapleServerSessionHandler, MapleSessionHandler},
        resp::{MigrateResponse, PacketOpcodeExt, ResponsePacket},
    },
    MapleSession,
};
use moople_packet::{
    proto::{list::MapleIndexList8, time::MapleTime, MapleList8},
    HasOpcode, MaplePacket, MaplePacketReader, MaplePacketWriter,
};

use proto95::shared::char::AvatarEquips;
use proto95::shared::{ExceptionLogReq, PongReq};
use proto95::{
    id::{FaceId, HairId, ItemId, Skin},
    login::{
        account::{
            BlockedIp, CheckPasswordReq, CheckPasswordResp, ConfirmEULAReq, ConfirmEULAResp,
            LoginAccountData, LoginInfo, SetGenderReq, SetGenderResp, SuccessResult,
        },
        char::{
            CharRankInfo, CheckDuplicateIDReq, CheckDuplicateIDResp, CheckDuplicateIDResult,
            CreateCharReq, CreateCharResp, DeleteCharReq, DeleteCharResp, DeleteCharResult,
            MigrateStageInfo, SelectCharReq, SelectCharResp, SelectCharResult, SelectWorldCharList,
            SelectWorldResp, ViewChar, ViewCharWithRank,
        },
        pin::{CheckPinReq, CheckPinResp, UpdatePinReq, UpdatePinResp},
        world::{
            ChannelId, LogoutWorldReq, SelectWorldReq, WorldCheckUserLimitReq,
            WorldCheckUserLimitResp, WorldId, WorldInfoReq, WorldInfoResp, WorldReq,
        },
        CreateSecurityHandleReq, LoginOpt, LoginResultHeader,
    },
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{
        char::{AvatarData, CharStat, PetIds},
        UpdateScreenSettingReq,
    },
};
use tokio::net::TcpStream;

pub type LoginResponse<T> = ResponsePacket<SendOpcodes, T>;
pub type LoginResult<T> = Result<LoginResponse<T>, anyhow::Error>;

pub struct LoginHandler {
    services: services::SharedServices,
    state: LoginState,
    addr: IpAddr,
    cfg: &'static LoginConfig,
}

impl LoginHandler {
    pub fn new(
        services: services::SharedServices,
        cfg: &'static LoginConfig,
        addr: IpAddr,
    ) -> Self {
        Self {
            services,
            state: LoginState::default(),
            cfg,
            addr,
        }
    }
}

#[async_trait]
impl MapleSessionHandler for LoginHandler {
    type Transport = TcpStream;
    type Error = anyhow::Error;

    async fn handle_packet(
        &mut self,
        packet: MaplePacket,
        session: &mut moople_net::MapleSession<Self::Transport>,
    ) -> Result<SessionHandleResult, Self::Error> {
        maple_router_handler!(
            handler,
            LoginHandler,
            MapleSession<TcpStream>,
            anyhow::Error,
            LoginHandler::handle_default,
            PongReq => LoginHandler::handle_pong,
            CreateSecurityHandleReq => LoginHandler::handle_create_security_handle,
            UpdateScreenSettingReq => LoginHandler::handle_update_screen_setting,
            CheckPasswordReq => LoginHandler::handle_check_password,
            SetGenderReq => LoginHandler::handle_set_gender,
            CheckPinReq => LoginHandler::handle_check_pin,
            UpdatePinReq => LoginHandler::handle_register_pin,
            ConfirmEULAReq => LoginHandler::handle_accept_tos,
            WorldInfoReq => LoginHandler::handle_world_information,
            LogoutWorldReq => LoginHandler::handle_world_logout,
            WorldReq => LoginHandler::handle_world_request,
            WorldCheckUserLimitReq => LoginHandler::handle_world_check_user_limit,
            SelectWorldReq => LoginHandler::handle_select_world,
            CheckDuplicateIDReq => LoginHandler::handle_check_duplicate_id,
            CreateCharReq => LoginHandler::handle_create_char,
            DeleteCharReq => LoginHandler::handle_delete_character,
            SelectCharReq => LoginHandler::handle_select_char,
            ExceptionLogReq => LoginHandler::handle_exception_log
        );

        handler(self, session, packet.into_reader()).await
    }
}

impl MapleServerSessionHandler for LoginHandler {
    fn get_ping_interval() -> std::time::Duration {
        Duration::from_secs(30)
    }

    fn get_ping_packet(&mut self) -> Result<MaplePacket, Self::Error> {
        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(SendOpcodes::AliveReq);
        Ok(pw.into_packet())
    }
}

impl LoginHandler {
    pub async fn handle_default(
        &mut self,
        _op: RecvOpcodes,
        pr: MaplePacketReader<'_>,
    ) -> anyhow::Result<SessionHandleResult> {
        log::info!("Unhandled packet: {:?}", pr.into_inner());
        Ok(SessionHandleResult::Ok)
    }

    async fn handle_pong(&mut self, _req: PongReq) -> anyhow::Result<PongResponse> {
        Ok(PongResponse)
    }

    async fn handle_exception_log(&mut self, _req: ExceptionLogReq) -> anyhow::Result<()> {
        dbg!(&_req);
        Ok(())
    }

    async fn handle_create_security_handle(
        &mut self,
        _req: CreateSecurityHandleReq,
    ) -> anyhow::Result<()> {
        dbg!(&_req);
        Ok(())
    }

    async fn handle_update_screen_setting(
        &mut self,
        req: UpdateScreenSettingReq,
    ) -> anyhow::Result<()> {
        dbg!(&req);
        Ok(())
    }

    async fn handle_accept_tos(&mut self, req: ConfirmEULAReq) -> LoginResult<ConfirmEULAResp> {
        self.state.get_accept_tos()?;

        if !req.accepted {
            anyhow::bail!("Should accept the TOS");
        }

        self.state
            .update_account(|acc| self.services.data.account.accept_tos(acc))
            .await?;
        self.state.reset();

        Ok(ConfirmEULAResp { success: true }.into())
    }

    async fn handle_check_pin(&mut self, req: CheckPinReq) -> LoginResult<CheckPinResp> {
        let acc = self.state.get_pin()?;

        Ok(if self.cfg.enable_pin {
            match req.pin.opt {
                Some(pin) => {
                    if self.services.data.account.check_pin(acc, &pin.pin)? {
                        CheckPinResp::Accepted
                    } else {
                        CheckPinResp::InvalidPin
                    }
                }
                _ => CheckPinResp::EnterPin,
            }
        } else {
            CheckPinResp::Accepted
        }
        .into())
    }

    async fn handle_register_pin(&mut self, req: UpdatePinReq) -> LoginResult<UpdatePinResp> {
        self.state.get_pin()?;

        let Some(pin) = req.pin.opt else {
            //TODO handle a login reset here not a dc
            anyhow::bail!("Pin registration cancelled");
        };

        self.state
            .update_account(|acc| self.services.data.account.set_pin(acc, pin))
            .await?;

        Ok(UpdatePinResp { success: true }.into())
    }

    async fn handle_set_gender(&mut self, req: SetGenderReq) -> LoginResult<SetGenderResp> {
        let _ = self.state.get_set_gender()?;

        let gender = req
            .gender
            .opt
            .ok_or_else(|| anyhow::format_err!("Gender not set"))?;

        self.state
            .update_account(|acc| self.services.data.account.set_gender(acc, gender.into()))
            .await?;

        self.state.transition_login().unwrap();

        //TODO this doesn't set the client key, maybe make it dc?
        Ok(SetGenderResp {
            gender,
            success: true,
        }
        .into())
    }

    async fn handle_world_logout(&mut self, _req: LogoutWorldReq) -> anyhow::Result<()> {
        self.state.get_char_select()?;
        self.state.transition_server_select()?;

        Ok(())
    }

    async fn handle_world_check_user_limit(
        &mut self,
        _req: WorldCheckUserLimitReq,
    ) -> LoginResult<WorldCheckUserLimitResp> {
        let _acc = self.state.get_server_selection()?;

        Ok(WorldCheckUserLimitResp {
            over_user_limit: false,
            populate_level: 0,
        }
        .into())
    }

    fn get_world_info(&self) -> Vec<LoginResponse<WorldInfoResp>> {
        self.services
            .server_info
            .get_world_info_packets()
            .into_iter()
            .map(|p| p.into())
            .collect()
    }

    async fn handle_world_information(
        &mut self,
        _req: WorldInfoReq,
    ) -> anyhow::Result<Vec<LoginResponse<WorldInfoResp>>> {
        Ok(self.get_world_info())
    }

    async fn handle_world_request(
        &mut self,
        _req: WorldReq,
    ) -> anyhow::Result<Vec<LoginResponse<WorldInfoResp>>> {
        Ok(self.get_world_info())
    }

    pub async fn handle_check_password(
        &mut self,
        req: CheckPasswordReq,
    ) -> LoginResult<CheckPasswordResp> {
        let login_result = self.services.data.account.try_login(&req.id, &req.pw).await;
        let hdr = LoginResultHeader::default();

        let res = match login_result {
            Err(AccountServiceError::UsernameNotFound) => CheckPasswordResp::InvalidUserName(hdr),
            Err(AccountServiceError::PasswordMismatch) => CheckPasswordResp::InvalidPassword(hdr),
            Err(AccountServiceError::AccountIsBanned) => CheckPasswordResp::BlockedIp(BlockedIp {
                hdr,
                reason: 0,
                ban_time: MapleTime::maple_default(),
            }),
            Ok(acc) => {
                let account_info = (&acc).into();
                self.state.transition_login_with_acc(acc)?;
                let client_key = self
                    .state
                    .get_client_key()
                    .expect("Must have client key after login");

                let login_info = (!self.state.is_set_gender_stage())
                    .then_some(LoginInfo {
                        skip_pin: false,
                        login_opt: proto95::login::LoginOpt::EnableSecondPassword,
                        client_key,
                    })
                    .into();

                if self.state.is_accept_tos_stage() {
                    CheckPasswordResp::TOS(hdr)
                } else {
                    CheckPasswordResp::Success(SuccessResult {
                        hdr,
                        account: LoginAccountData {
                            account_info,
                            login_info,
                        },
                    })
                }
            }
            _ => todo!("Unhandled Account Service Login Result: {:?}", login_result),
        };

        Ok(res.into())
    }

    async fn handle_select_world(&mut self, req: SelectWorldReq) -> LoginResult<SelectWorldResp> {
        let acc = self.state.get_server_selection()?;
        let char_list = self
            .services
            .data
            .char
            .get_characters_for_account(acc.id)
            .await?;
        let characters: MapleList8<_> = char_list.iter().map(map_char_with_rank).collect();

        let char_list = SelectWorldCharList {
            characters,
            //TODO pic handling
            login_opt: LoginOpt::NoSecondPassword1,
            slot_count: acc.character_slots as u32,
            //TODO get buy count
            buy_char_count: 3,
        };
        self.state
            .transition_char_select(req.world_id as WorldId, req.channel_id as ChannelId)?;

        Ok(SelectWorldResp::Success(char_list).into())
    }

    async fn handle_check_duplicate_id(
        &mut self,
        req: CheckDuplicateIDReq,
    ) -> anyhow::Result<LoginResponse<CheckDuplicateIDResp>> {
        let _ = self.state.get_char_select()?;
        let name_used = !self.services.data.char.check_name(&req.name).await?;

        let resp = if name_used {
            CheckDuplicateIDResp {
                name: "".to_string(),
                result: CheckDuplicateIDResult::Error1,
            }
        } else {
            CheckDuplicateIDResp {
                name: req.name,
                result: CheckDuplicateIDResult::Success,
            }
        };

        Ok(resp.into())
    }

    async fn handle_create_char(&mut self, req: CreateCharReq) -> LoginResult<CreateCharResp> {
        let (acc, _, _) = self.state.get_char_select()?;

        let starter_set = ItemStarterSet {
            shoes: req.starter_set.shoes,
            bottom: req.starter_set.bottom,
            weapon: req.starter_set.weapon,
            top: req.starter_set.top,
            guide: req.job.get_guide_item(),
        };

        let char_id = self
            .services
            .data
            .char
            .create_character(
                acc.id,
                CharacterCreateDTO {
                    name: req.name,
                    job_group: req.job,
                    face: req.starter_set.face,
                    skin: (req.starter_set.skin_color as u8).try_into()?,
                    hair: req.starter_set.hair,
                    //TODO hair color
                    starter_set,
                    gender: req.gender,
                },
                &self.services.data.item,
            )
            .await?;

        let char = self.services.data.char.get(char_id).await?.unwrap();
        Ok(CreateCharResp::Success(map_char(&char)).into())
    }

    async fn handle_delete_character(&mut self, req: DeleteCharReq) -> LoginResult<DeleteCharResp> {
        let (acc, _, _) = self.state.get_char_select()?;
        let status = self
            .services
            .data
            .char
            .delete_character(acc, req.char_id as i32, &req.pic)
            .await?;

        let result = match status {
            DeleteCharResult::Success => DeleteCharResult::Success,
            //TODO add more
            _ => DeleteCharResult::UnknownErr,
        };

        Ok(DeleteCharResp {
            char_id: req.char_id,
            result,
        }
        .into())
    }

    async fn handle_select_char(
        &mut self,
        req: SelectCharReq,
    ) -> anyhow::Result<MigrateResponse<ResponsePacket<SendOpcodes, SelectCharResp>>> {
        let (_, world, channel) = self.state.get_char_select()?;

        let acc = self.state.claim_account()?;
        let client_key = self.state.get_client_key()?;

        self.services
            .session_manager
            .create_migration_session(
                MoopleMigrationKey::new(client_key, self.addr),
                (acc, req.char_id as CharacterID),
            )
            .await?;

        let addr = self.services.server_info.get_channel_addr(world, channel)?;
        let migrate = MigrateStageInfo {
            socket_addr: addr.try_into()?,
            char_id: req.char_id,
            premium: false,
            premium_arg: 0,
        };

        let pkt: ResponsePacket<_, _> = SelectCharResp {
            error_code: 0,
            result: SelectCharResult::Success(migrate),
        }
        .into_response(SelectCharResp::OPCODE);

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
        equips: AvatarEquips {
            equips: MapleIndexList8::from(vec![
                (5, ItemId(1040006)),
                (6, ItemId(1060006)),
                (7, ItemId(1072005)),
                (11, ItemId(1322005)),
            ]),
            masked_equips: MapleIndexList8::from(vec![]),
            weapon_sticker_id: ItemId(0),
        },
        pets: PetIds::default(),
    }
}

pub fn map_rank_info(_char: &character::Model) -> CharRankInfo {
    CharRankInfo {
        world_rank: 0,
        rank_move: 0,
        job_rank: 0,
        job_rank_mode: 0,
    }
}

pub fn map_char(char: &character::Model) -> ViewChar {
    let stats: CharStat = char.into();
    let avatar_data = map_char_to_avatar(char);

    ViewChar { stats, avatar_data }
}

fn map_char_with_rank(char: &character::Model) -> ViewCharWithRank {
    ViewCharWithRank {
        view_char: map_char(char),
        u1: 0,
        rank_info: Some(map_rank_info(char)).into(),
    }
}
