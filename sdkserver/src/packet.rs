use anyhow::Result;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::io;
use prost::Message; // Ensure `Message` is imported from your proto module

const HEAD_MAGIC: u32 = 0x9D74C714;
const TAIL_MAGIC: u32 = 0xD7A152C8;

pub struct NetPacket {
    pub cmd_type: u16,
    pub head: Vec<u8>,
    pub body: Vec<u8>,
}

pub struct PlayerSession {
    socket: TcpStream,
}

impl From<NetPacket> for Vec<u8> {
    fn from(value: NetPacket) -> Self {
        let mut out = Vec::new();
        out.extend(HEAD_MAGIC.to_be_bytes());
        out.extend(value.cmd_type.to_be_bytes());
        out.extend((value.head.len() as u16).to_be_bytes());
        out.extend((value.body.len() as u32).to_be_bytes());
        out.extend(value.head);
        out.extend(value.body);
        out.extend(TAIL_MAGIC.to_be_bytes());
        out
    }
}

impl NetPacket {
    pub async fn read(stream: &mut TcpStream) -> io::Result<Self> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf).await?;
        let head_magic = u32::from_be_bytes(buf);

        if head_magic != HEAD_MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid head magic number"));
        }

        let cmd_type = stream.read_u16().await?;
        let head_length = stream.read_u16().await? as usize;
        let body_length = stream.read_u32().await? as usize;

        let mut head = vec![0; head_length];
        stream.read_exact(&mut head).await?;

        let mut body = vec![0; body_length];
        stream.read_exact(&mut body).await?;

        stream.read_exact(&mut buf).await?;
        let tail_magic = u32::from_be_bytes(buf);

        if tail_magic != TAIL_MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid tail magic number"));
        }

        Ok(Self {
            cmd_type,
            head,
            body,
        })
    }
}

impl PlayerSession {
    pub fn client_socket(&self) -> &TcpStream {
        &self.socket
    }

    pub async fn send(&self, cmd_type: u16, body: impl Message) -> Result<()> {
        let packet = NetPacket {
            cmd_type,
            head: Vec::new(),
            body: body.encode_to_vec(),
        };

        let packet_bytes: Vec<u8> = packet.into();
        self.client_socket()
            .write_all(&packet_bytes)
            .await?;

        Ok(())
    }
}
