use anyhow::Result;
use atomic_refcell::{AtomicRef, AtomicRefCell};
use prost::Message;
use std::sync::Arc;
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::{Mutex, MutexGuard},
};

use crate::game::{GameContext, PlayerInfo};

use super::{packet::NetCommandHandler, NetPacket};

#[derive(Clone)]
pub struct PlayerSession {
    client_socket: Arc<Mutex<TcpStream>>,
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
    pub context: GameContext,
}

impl PlayerSession {
    pub fn new(client_socket: TcpStream) -> Self {
        let player_info = Arc::new(AtomicRefCell::new(PlayerInfo::new()));

        Self {
            client_socket: Arc::new(Mutex::new(client_socket)),
            player_info: player_info.clone(),
            context: GameContext::new(player_info),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let net_packet = match NetPacket::read(&mut *self.client_socket().await).await {
                Ok(packet) => packet,
                Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                    return self.context.on_player_logout().await
                }
                Err(err) => {
                    self.context.on_player_logout().await?;
                    return Err(err.into());
                }
            };

            Self::on_message(self, net_packet.cmd_type, net_packet.body).await?;
        }
    }

    pub async fn send(&self, cmd_type: u16, body: impl Message) -> Result<()> {
        self.client_socket()
            .await
            .write_all(&Vec::from(NetPacket {
                cmd_type,
                head: Vec::new(),
                body: body.encode_to_vec(),
            }))
            .await?;

        Ok(())
    }

    pub async fn send_dummy(&self, cmd_type: u16) -> Result<()> {
        self.client_socket()
            .await
            .write_all(&Vec::from(NetPacket {
                cmd_type,
                head: Vec::new(),
                body: Vec::new(),
            }))
            .await?;

        Ok(())
    }

    pub async fn client_socket(&self) -> MutexGuard<'_, TcpStream> {
        self.client_socket.lock().await
    }

    pub fn player_info(&self) -> AtomicRef<PlayerInfo> {
        self.player_info.borrow()
    }
}

// Auto implemented
impl NetCommandHandler for PlayerSession {}
