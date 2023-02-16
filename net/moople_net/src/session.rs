use std::{io, net::SocketAddr};

use crate::codec::{
    handshake::Handshake,
    maple_codec::{MapleCodec, MapleFramedCodec},
};
use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use moople_packet::{opcode::NetOpcode, EncodePacket, MaplePacket, MaplePacketWriter, NetResult};
use tokio::{
    io::{AsyncRead, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
};
use tokio_util::codec::Framed;

pub trait SessionTransport: AsyncWrite + AsyncRead {}
impl<T> SessionTransport for T where T: AsyncWrite + AsyncRead {}

pub struct MapleSession<T> {
    pub codec: MapleFramedCodec<T>,
    //TODO use the codec write buffer later
    //TODO: how to handle panic/unwind if buffer capacity is too low
    encode_buf: BytesMut,
}

impl<T> MapleSession<T>
where
    T: SessionTransport + Unpin,
{
    pub fn new(codec: MapleFramedCodec<T>) -> Self {
        Self {
            codec,
            encode_buf: BytesMut::with_capacity(4096),
        }
    }

    pub async fn initialize_server_session(mut io: T, handshake: &Handshake) -> NetResult<Self> {
        handshake.write_handshake_async(&mut io).await?;
        Ok(Self::from_server_handshake(io, handshake))
    }

    pub async fn initialize_client_session(mut io: T) -> NetResult<(Self, Handshake)> {
        let handshake = Handshake::read_handshake_async(&mut io).await?;
        let sess = Self::from_client_handshake(io, &handshake);

        Ok((sess, handshake))
    }

    pub fn from_server_handshake(io: T, handshake: &Handshake) -> Self {
        let codec = MapleCodec::server_from_handshake(handshake);
        let framed = Framed::new(io, codec);
        Self::new(framed)
    }

    pub fn from_client_handshake(io: T, handshake: &Handshake) -> Self {
        let codec = MapleCodec::client_from_handshake(handshake);
        let framed = Framed::new(io, codec);
        Self::new(framed)
    }

    pub async fn read_packet(&mut self) -> NetResult<MaplePacket> {
        match self.codec.next().await {
            Some(p) => Ok(p?),
            None => Err(io::Error::from(io::ErrorKind::UnexpectedEof).into()),
        }
    }

    pub async fn send_packet(&mut self, pkt: MaplePacket) -> NetResult<()> {
        self.codec.send(pkt).await?;
        Ok(())
    }

    pub async fn encode_packet<P: EncodePacket>(
        &mut self,
        opcode: impl NetOpcode,
        data: P,
    ) -> NetResult<()> {
        self.encode_buf.clear();
        let mut pw = MaplePacketWriter::new(self.encode_buf.clone());
        pw.write_opcode(opcode);
        data.encode_packet(&mut pw)?;

        self.send_packet(MaplePacket::from_writer(pw)).await?;
        Ok(())
    }

    pub async fn shutdown(&mut self) -> NetResult<()> {
        self.get_mut().shutdown().await?;
        Ok(())
    }

    pub fn get_ref(&self) -> &T {
        self.codec.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.codec.get_mut()
    }
}

impl MapleSession<TcpStream> {
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.get_ref().peer_addr()
    }
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.get_ref().local_addr()
    }

    pub async fn connect(addr: &SocketAddr) -> NetResult<(Self, Handshake)> {
        let socket = TcpStream::connect(addr).await?;

        Self::initialize_client_session(socket).await
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use bytes::Bytes;
    use moople_packet::MaplePacket;
    use turmoil::net::{TcpListener, TcpStream};

    use crate::{codec::handshake::Handshake, crypto::RoundKey, MapleSession};

    const PORT: u16 = 1738;

    async fn bind() -> std::result::Result<TcpListener, std::io::Error> {
        TcpListener::bind((IpAddr::from(Ipv4Addr::UNSPECIFIED), PORT)).await
    }

    #[test]
    fn echo() -> anyhow::Result<()> {
        let mut sim = turmoil::Builder::new().build();
        const ECHO_DATA: [&'static [u8]; 4] = [&[0xFF; 4096], &[1, 2], &[], &[0x0; 1024]];
        const V: u16 = 83;

        sim.host("server", || async move {
            let handshake = Handshake {
                version: V,
                subversion: "1".to_string(),
                iv_enc: RoundKey::zero(),
                iv_dec: RoundKey::zero(),
                locale: 1,
            };

            let listener = bind().await?;

            loop {
                let socket = listener.accept().await?.0;
                let mut sess = MapleSession::initialize_server_session(socket, &handshake).await?;

                // Echo
                loop {
                    match sess.read_packet().await {
                        Ok(pkt) => {
                            sess.send_packet(pkt).await?;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
        });

        sim.client("client", async move {
            let socket = TcpStream::connect(("server", PORT)).await?;
            let (mut sess, handshake) = MapleSession::initialize_client_session(socket).await?;
            assert_eq!(handshake.version, V);

            for data in ECHO_DATA.iter() {
                sess.send_packet(MaplePacket::from_data(Bytes::from_static(data)))
                    .await?;
                let pkt = sess.read_packet().await?;
                assert_eq!(pkt.data.as_ref(), *data);
            }

            Ok(())
        });

        sim.run().unwrap();

        Ok(())
    }
}
