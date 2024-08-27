use crate::{game::commands, util};

use super::*;

pub async fn on_send_msg_cs_req(session: &PlayerSession, body: &SendMsgCsReq) -> Result<()> {
    if body.message_text.starts_with("/") {
        commands::execute_command(&body.message_text, session).await?;
    } else {
        session
        .send(
            CMD_GET_PRIVATE_CHAT_HISTORY_SC_RSP,
            GetPrivateChatHistoryScRsp {
                contact_id: 13371337,
                chat_message_list: vec![ChatMessageData {
                    sender_id: session.context.get_uid(),
                    message_type: MsgType::CustomText.into(),
                    timestamp: util::cur_timestamp_seconds(),
                    content: body.message_text.to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await?;
    }

    println!(
        "Message Text: {}, UID: {}, to_uid: {}",
        body.message_text.to_string(), session.context.get_uid(), body.extra_id
    );

    let notify = RevcMsgScNotify {
        message_type: MsgType::CustomText.into(),
        chat_type: ChatType::Private.into(),
        message_text: body.message_text.to_string(),
        to_uid: 0,
        sender_uid: 0,
        ..Default::default()
    };
    
    // Print the notification for debugging
    println!(
        "RevcMsgScNotify: message_type={:?}, chat_type={:?}, message_text={}, to_uid={}, sender_uid={}",
        notify.message_type,
        notify.chat_type,
        notify.message_text,
        notify.to_uid,
        notify.sender_uid
    );
    
    // Send the notification
    session.send(CMD_REVC_MSG_SC_NOTIFY, notify).await?;

    session.send(
        CMD_SEND_MSG_SC_RSP,
        SendMsgScRsp {
            end_time: util::cur_timestamp_seconds(),
            retcode: 0,
        },
    )
    .await?;

    Ok(())
}
