use anyhow::Result;
use paste::paste;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::Instrument;

use proto::*;

use super::handlers::*;
use super::PlayerSession;

const HEAD_MAGIC: u32 = 0x9D74C714;
const TAIL_MAGIC: u32 = 0xD7A152C8;

pub struct NetPacket {
    pub cmd_type: u16,
    pub head: Vec<u8>,
    pub body: Vec<u8>,
}

impl From<NetPacket> for Vec<u8> {
    fn from(value: NetPacket) -> Self {
        let mut out = Self::new();

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
    pub async fn read(stream: &mut TcpStream) -> std::io::Result<Self> {
        assert_eq!(stream.read_u32().await?, HEAD_MAGIC);
        let cmd_type = stream.read_u16().await?;

        let head_length = stream.read_u16().await? as usize;
        let body_length = stream.read_u32().await? as usize;

        let mut head = vec![0; head_length];
        stream.read_exact(&mut head).await?;

        let mut body = vec![0; body_length];
        stream.read_exact(&mut body).await?;

        assert_eq!(stream.read_u32().await?, TAIL_MAGIC);

        Ok(Self {
            cmd_type,
            head,
            body,
        })
    }
}

macro_rules! trait_handler {
    ($($name:ident $cmd_type:expr;)*) => {
        pub trait NetCommandHandler {
            $(
                paste! {
                    async fn [<on_$name:snake>](session: &PlayerSession, body: &$name) -> Result<()> {
                        [<on_$name:snake>](session, body).await
                    }
                }
            )*

            async fn on_message(session: &PlayerSession, cmd_type: u16, payload: Vec<u8>) -> Result<()> {
                use ::prost::Message;
                if PlayerSession::should_send_dummy_rsp(cmd_type) {
                    session.send_dummy_response(cmd_type).await?;
                    return Ok(());
                }
                match cmd_type {
                    $(
                        $cmd_type => {
                            let body = $name::decode(&mut &payload[..])?;
                            paste! {
                                Self::[<on_$name:snake>](session, &body)
                                    .instrument(tracing::info_span!(stringify!([<on_$name:snake>]), cmd_type = cmd_type))
                                    .await
                            }
                        }
                    )*
                    _ => {
                        tracing::warn!("Unknown command type: {cmd_type}");
                        Ok(())
                    },
                }
            }
        }
    };
}

trait_handler! {
    GetAvatarDataCsReq 398;
    TakeOffEquipmentCsReq 378;
    TakeOffRelicCsReq 320;
    DressAvatarCsReq 328;
    DressRelicAvatarCsReq 305;
    PveBattleResultCsReq 198;
    GetBagCsReq 598;
    GetAllLineupDataCsReq 731;
    JoinLineupCsReq 779;
    ChangeLineupLeaderCsReq 722;
    ReplaceLineupCsReq 752;
    QuitLineupCsReq 733;
    GetCurLineupDataCsReq 783;
    GetCurBattleInfoCsReq 179;
    GetMissionStatusCsReq 1231;
    PlayerGetTokenCsReq 79;
    PlayerLoginCsReq 98;
    PlayerHeartBeatCsReq 39;
    GetBasicInfoCsReq 53;
    GetCurSceneInfoCsReq 1433;
    SceneEntityMoveCsReq 1498;
    StartCocoonStageCsReq 1455; // last saved
    EnterSectionCsReq 1462;
    GetSceneMapInfoCsReq 1426;
    EnterSceneCsReq 1473;
    GetTutorialGuideCsReq 1683;
    UnlockTutorialGuideCsReq 1633;
    GetTutorialCsReq 1698;
    GetFriendListInfoCsReq 2998;
    SendMsgCsReq 3998;
    ContentPackageGetDataCsReq 7529;
    PlayerLoginFinishCsReq 90;
    GetMultiPathAvatarInfoCsReq 68;
    SetClientPausedCsReq 1457;
    // PlayerLogoutCsReq 11;
}