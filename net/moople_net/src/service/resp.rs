use async_trait::async_trait;
use moople_packet::{
    opcode::{HasOpcode, NetOpcode},
    EncodePacket, NetResult,
};

use crate::{MapleSession, SessionTransport};

//TODO get rid of async_trait for performance reasons here

#[async_trait]
pub trait Response {
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        session: &mut MapleSession<Trans>,
    ) -> NetResult<()>;
}

#[async_trait]
impl Response for () {
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        _session: &mut MapleSession<Trans>,
    ) -> NetResult<()> {
        Ok(())
    }
}

#[async_trait]
impl<Resp: Response + Send> Response for Option<Resp> {
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        session: &mut MapleSession<Trans>,
    ) -> NetResult<()> {
        match self {
            Some(resp) => resp.send(session).await,
            None => Ok(()),
        }
    }
}

#[async_trait]
impl<Resp: Response + Send> Response for Vec<Resp> {
    async fn send<Trans: SessionTransport + Send + Unpin>(
        self,
        session: &mut MapleSession<Trans>,
    ) -> NetResult<()> {
        for resp in self.into_iter() {
            resp.send(session).await?;
        }
        Ok(())
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
    ) -> NetResult<()> {
        session.encode_packet(self.op, self.data).await?;
        Ok(())
    }
}

pub trait PacketOpcodeExt: Sized + EncodePacket {
    fn as_response<Op: NetOpcode>(self, opcode: Op) -> ResponsePacket<Op, Self> {
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

impl<Op, T> IntoResponse for ResponsePacket<Op, T>
where
    Op: NetOpcode + Send,
    T: EncodePacket + Send,
{
    type Resp = ResponsePacket<Op, T>;

    fn into_response(self) -> Self::Resp {
        self
    }
}

impl IntoResponse for () {
    type Resp = ();

    fn into_response(self) -> Self::Resp {}
}

impl<Resp> IntoResponse for Option<Resp>
where
    Resp: IntoResponse,
{
    type Resp = Option<Resp::Resp>;

    fn into_response(self) -> Self::Resp {
        self.map(|r| r.into_response())
    }
}

impl<Resp> IntoResponse for Vec<Resp>
where
    Resp: IntoResponse,
{
    type Resp = Vec<Resp::Resp>;

    fn into_response(self) -> Self::Resp {
        self.into_iter().map(|r| r.into_response()).collect()
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
