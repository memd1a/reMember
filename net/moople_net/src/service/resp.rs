use async_trait::async_trait;
use moople_packet::{
    opcode::{HasOpcode, NetOpcode},
    EncodePacket, NetResult,
};

use crate::{MapleSession, SessionTransport};

use super::handler::SessionHandleResult;

//TODO get rid of async_trait for performance reasons here

#[async_trait]
pub trait Response {
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        session: &mut MapleSession<Trans>,
    ) -> NetResult<SessionHandleResult>;
}

#[async_trait]
impl Response for () {
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        _session: &mut MapleSession<Trans>,
    ) -> NetResult<SessionHandleResult> {
        Ok(SessionHandleResult::Ok)
    }
}

#[async_trait]
impl<Resp: Response + Send> Response for Option<Resp> {
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        session: &mut MapleSession<Trans>,
    ) -> NetResult<SessionHandleResult> {
        match self {
            Some(resp) => resp.send(session).await,
            None => Ok(SessionHandleResult::Ok),
        }
    }
}

#[async_trait]
impl<Resp: Response + Send> Response for Vec<Resp> {
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        session: &mut MapleSession<Trans>,
    ) -> NetResult<SessionHandleResult> {
        for resp in self.into_iter() {
            resp.send(session).await?;
        }
        Ok(SessionHandleResult::Ok)
    }
}

pub struct ResponsePacket<Op, T> {
    pub op: Op,
    pub data: T,
}

impl<Op, T> ResponsePacket<Op, T> {
    pub fn new(op: Op, data: T) -> Self {
        Self { op, data }
    }
}

#[async_trait]
impl<Op, T> Response for ResponsePacket<Op, T>
where
    Op: NetOpcode + Send,
    T: EncodePacket + Send,
{
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        session: &mut MapleSession<Trans>,
    ) -> NetResult<SessionHandleResult> {
        session.send_packet_with_opcode(self.op, self.data).await?;
        Ok(SessionHandleResult::Ok)
    }
}

pub trait PacketOpcodeExt: EncodePacket {
    fn into_response<Op: NetOpcode>(self, opcode: Op) -> ResponsePacket<Op, Self> {
        ResponsePacket::new(opcode, self)
    }
}

impl<T: EncodePacket> PacketOpcodeExt for T {}

pub trait IntoResponse {
    type Resp: Response + Send;

    fn into_response(self) -> Self::Resp;
}

impl<T: EncodePacket + HasOpcode> From<T> for ResponsePacket<T::OP, T> {
    fn from(value: T) -> Self {
        ResponsePacket::new(T::OPCODE, value)
    }
}

/// Response which sends the packet `T` and then
/// migrates the session
pub struct MigrateResponse<T>(pub T);

#[async_trait]
impl<T> Response for MigrateResponse<T>
where
    T: Response + Send,
{
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        session: &mut MapleSession<Trans>,
    ) -> NetResult<SessionHandleResult> {
        self.0.send(session).await?;
        return Ok(SessionHandleResult::Migrate);
    }
}

pub struct PongResponse;

#[async_trait]
impl Response for PongResponse
{
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        _session: &mut MapleSession<Trans>,
    ) -> NetResult<SessionHandleResult> {
        return Ok(SessionHandleResult::Pong);
    }
}

impl<T> IntoResponse for T
where
    T: Response + Send,
{
    type Resp = T;

    fn into_response(self) -> Self::Resp {
        self
    }
}

#[cfg(test)]
mod tests {

    use super::{IntoResponse, ResponsePacket};

    fn check_is_into_response<T>() -> bool
    where
        T: IntoResponse,
    {
        true
    }

    #[test]
    fn name() {
        check_is_into_response::<()>();
        check_is_into_response::<Option<()>>();
        check_is_into_response::<ResponsePacket<u16, ()>>();
        check_is_into_response::<Vec<ResponsePacket<u16, ()>>>();
    }
}
