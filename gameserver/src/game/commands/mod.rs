use anyhow::Result;
use proto::*;

use crate::{net::PlayerSession, util};

mod set;
mod lua;
mod give;
mod scene;
mod battle;
mod refillsp;

macro_rules! commands {
    ($($category:ident $action:ident;)*) => {
        pub async fn execute_command(command: &str, session: &PlayerSession) -> Result<()> {
            let input = command[1..].split(" ").collect::<Vec<&str>>();

            if input.is_empty() {
                return send_text(session, "Available Commands: lua, avatar, give, scene, set, battle").await;
            }

            match input[0] {
                "lua" => {
                    return lua::windy_command(&input[1..], session).await;
                }
                "give" => {
                    return give::give_command(&input[1..], session).await;
                }
                "set" => {
                    return set::set_command(&input[1..], session).await;
                }
                "refill" => {
                    return refillsp::refill_sp(&input[1..], session).await;
                }                    
                "battle" => {
                    return battle::battle_command(&input[1..], session).await;
                }                
                _ => {}
            }

            let (Some(category), Some(action)) = (input.get(0), input.get(1)) else {
                return send_text(session, "Available Commands: lua, give, scene, set, battle").await;
            };

            let args = &input[2..];
            if let Err(_) = match (*category, *action) {
                $(
                    (stringify!($category), stringify!($action)) => {
                        $category::$action(args, session).await
                    }
                )*,
                _ => send_text(session, "Unknown command").await,
            } {
                return send_text(
                    session,
                    "Command execution failed. Re-check your input and try again.",
                )
                .await;
            }

            Ok(())
        }
    };
}

commands! {
    scene enter;
}

async fn send_text(session: &PlayerSession, content: &str) -> Result<()> {

    session
        .send(
            CMD_GET_PRIVATE_CHAT_HISTORY_SC_RSP,
            GetPrivateChatHistoryScRsp {
                contact_id: 13371337,
                chat_message_list: vec![ChatMessageData {
                    sender_id: 13371337,
                    message_type: MsgType::CustomText.into(),
                    timestamp: util::cur_timestamp_seconds(),
                    content: content.to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}