use anyhow::Result;
use proto::*;

use crate::{database, net::PlayerSession, util};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::fs::File;
use std::io::Read;

pub async fn on_player_get_token_cs_req(
    session: &PlayerSession,
    body: &PlayerGetTokenCsReq,
) -> Result<()> {
    println!("Player Token: {}", &body.token);
    println!("Player UID: {}", &body.account_uid);
    if !database::verify_combo_token(&body.account_uid, &body.token).await? {
        return session
            .send(
                CMD_PLAYER_GET_TOKEN_SC_RSP,
                PlayerGetTokenScRsp {
                    retcode: Retcode::RetAccountVerifyError as u32,
                    msg: String::from("Account token is invalid. Please relogin and try again."),
                    ..Default::default()
                },
            )
            .await;
    }

    let (uid, player_bin) = database::get_player_bin_by_account_uid(&body.account_uid).await?;
    session.context.on_player_get_token_succ(uid, player_bin);
    session
        .send(
            CMD_PLAYER_GET_TOKEN_SC_RSP,
            PlayerGetTokenScRsp {
                uid,
                ..Default::default()
            },
        )
        .await
}

pub async fn on_player_login_cs_req(
    session: &PlayerSession,
    body: &PlayerLoginCsReq,
) -> Result<()> {
    if session.context.is_new_player() {
        session.context.init_default_player();
    }

    session.context.on_player_logged_in().await?;
    session
        .send(
            CMD_PLAYER_LOGIN_SC_RSP,
            PlayerLoginScRsp {
                login_random: body.login_random,
                server_timestamp_ms: util::cur_timestamp_ms(),
                stamina: 240,
                basic_info: Some(session.context.player_basic_info_proto()),
                ..Default::default()
            },
        )
        .await?;


    Ok(())
}

pub async fn on_player_login_finish_cs_req(
    session: &PlayerSession,
    body: &PlayerLoginFinishCsReq,
) -> Result<()> {

    tokio::spawn(read_user_input(session.clone()));

    // Sending the login finish response
    session.send(CMD_PLAYER_LOGIN_FINISH_SC_RSP, ()).await?;

    let content = [200001, 200002, 200003, 150017, 150015, 150021, 150018];
    let content_package_list: Vec<ContentPackageInfo> = content.iter().map(|&id_| {
        ContentPackageInfo {
            content_id: id_,
            status: ContentPackageStatus::Finished as i32,
        }
    }).collect();

    let data = ContentPackageData {
        cur_content_id: 0,
        content_package_list,
    };

    let content_data = ContentPackageSyncDataScNotify {
        data: Some(data),
    };
    session.send(CMD_CONTENT_PACKAGE_SYNC_DATA_SC_NOTIFY, content_data).await?;

    Ok(())
}

pub async fn on_content_package_get_data_cs_req(
    session: &PlayerSession,
    body: &ContentPackageGetDataCsReq,
) -> Result<()> {
    // Sending the contentpackage finish response
    session.send(CMD_CONTENT_PACKAGE_GET_DATA_SC_RSP, ()).await
}

pub async fn on_set_client_paused_cs_req(
    session: &PlayerSession,
    body: &SetClientPausedCsReq,
) -> Result<()> {
    session.send(
        CMD_SET_CLIENT_PAUSED_SC_RSP, SetClientPausedScRsp{
            retcode: 0,
            is_paused: body.is_paused,
        }).await
}

async fn read_user_input(
    session: PlayerSession,
) {
    let mut reader = BufReader::new(tokio::io::stdin());
    loop {
        let mut input = String::new();
        if let Ok(n) = reader.read_line(&mut input).await {
            if n == 0 {
                break; // EOF reached
            }
            let input = input.trim();
            if let Err(err) = handle_input(session.clone(), input).await {
                println!("Error: {}", err);
            }
        } else {
            println!("Failed to read input");
        }
    }
}

async fn handle_input(session: PlayerSession, input: &str) -> Result<()> {
    let file_path = format!("lua/{}.lua", input);
    let mut file = File::open(file_path)?;
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents)?;

    let windseed = ClientDownloadDataScNotify {
        download_data: Some(ClientDownloadData {
            version: 51,
            time: util::cur_timestamp_ms() as i64,
            data: file_contents,
        }),
    };
    session.send(CMD_CLIENT_DOWNLOAD_DATA_SC_NOTIFY, windseed).await?;
    Ok(())
}