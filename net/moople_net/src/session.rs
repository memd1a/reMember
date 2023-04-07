use std::{io, net::SocketAddr};

use crate::{
    codec::{handshake::Handshake, maple_codec::PacketCodec},
    service::packet_buffer::PacketBuffer,
};
use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use moople_packet::{
    opcode::NetOpcode, EncodePacket, HasOpcode, MaplePacket, MaplePacketWriter, NetResult,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};
use tokio_util::codec::Framed;

pub trait SessionTransport: AsyncWrite + AsyncRead {}
impl<T> SessionTransport for T where T: AsyncWrite + AsyncRead {}

pub struct MapleSession<T> {
    codec: Framed<T, PacketCodec>,
    encode_buffer: BytesMut,
}

impl<T> MapleSession<T>
where
    T: SessionTransport + Unpin,
{
    pub fn new(io: T, codec: PacketCodec) -> Self {
        Self {
            codec: Framed::new(io, codec),
            encode_buffer: BytesMut::new(),
        }
    }

    /// Initialize a server session, by sending out the given handshake
    pub async fn initialize_server_session(mut io: T, handshake: Handshake) -> NetResult<Self> {
        handshake.write_handshake_async(&mut io).await?;
        Ok(Self::from_server_handshake(io, handshake))
    }

    pub async fn initialize_client_session(mut io: T) -> NetResult<(Self, Handshake)> {
        let handshake = Handshake::read_handshake_async(&mut io).await?;
        let sess = Self::from_client_handshake(io, handshake.clone());

        Ok((sess, handshake))
    }

    pub fn from_server_handshake(io: T, handshake: Handshake) -> Self {
        let codec = PacketCodec::server_from_handshake(handshake);
        Self::new(io, codec)
    }

    pub fn from_client_handshake(io: T, handshake: Handshake) -> Self {
        let codec = PacketCodec::client_from_handshake(handshake);
        Self::new(io, codec)
    }

    pub async fn read_packet(&mut self) -> NetResult<MaplePacket> {
        match self.codec.next().await {
            Some(p) => Ok(p?),
            None => Err(io::Error::from(io::ErrorKind::UnexpectedEof).into()),
        }
    }

    pub async fn send_packet_buffer(&mut self, buf: &PacketBuffer) -> NetResult<()> {
        // It is required to send the packets one-by-one, because the client doesn't support
        // other ways
        for pkt in buf.packets() {
            self.send_raw_packet(pkt).await?;
        }

        Ok(())
    }

    pub async fn send_raw_packet(&mut self, data: &[u8]) -> NetResult<()> {
        self.codec.send(data).await?;
        Ok(())
    }

    pub async fn send_packet_with_opcode<P: EncodePacket>(
        &mut self,
        opcode: impl NetOpcode,
        data: P,
    ) -> NetResult<()> {
        self.encode_buffer.clear();
        let mut pw = MaplePacketWriter::new(&mut self.encode_buffer);
        pw.write_opcode(opcode);
        data.encode_packet(&mut pw)?;

        self.codec.send(&self.encode_buffer).await?;
        Ok(())
    }

    pub async fn send_packet<P: EncodePacket + HasOpcode>(&mut self, data: P) -> NetResult<()> {
        self.send_packet_with_opcode(P::OPCODE, data).await
    }

    pub async fn close(&mut self) -> NetResult<()> {
        self.codec.close().await?;
        Ok(())
    }

    pub async fn flush(&mut self) -> NetResult<()> {
        self.codec.flush().await?;
        Ok(())
    }
}

impl MapleSession<TcpStream> {
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.codec.get_ref().peer_addr()
    }
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.codec.get_ref().local_addr()
    }

    pub async fn connect(addr: SocketAddr) -> NetResult<(Self, Handshake)> {
        let socket = TcpStream::connect(addr).await?;
        Self::initialize_client_session(socket).await
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use arrayvec::ArrayString;
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
                subversion: ArrayString::try_from("1").unwrap(),
                iv_enc: RoundKey::zero(),
                iv_dec: RoundKey::zero(),
                locale: 1,
            };

            let listener = bind().await?;

            loop {
                let socket = listener.accept().await?.0;
                let mut sess =
                    MapleSession::initialize_server_session(socket, handshake.clone()).await?;

                // Echo
                loop {
                    match sess.read_packet().await {
                        Ok(pkt) => {
                            sess.send_raw_packet(&pkt.data).await?;
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
                sess.send_raw_packet(data).await?;
                let pkt = sess.read_packet().await?;
                assert_eq!(pkt.data.as_ref(), *data);
            }

            Ok(())
        });

        sim.run().unwrap();

        Ok(())
    }
}
