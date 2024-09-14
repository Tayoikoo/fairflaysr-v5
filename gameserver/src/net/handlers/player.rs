use crate::util;
use super::*;
use common::data::EXCEL_COLLECTION;
use std::collections::HashMap;

pub async fn on_get_basic_info_cs_req(
    session: &PlayerSession,
    _body: &GetBasicInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_BASIC_INFO_SC_RSP,
            GetBasicInfoScRsp {
                retcode: Retcode::RetSucc as u32,
                player_setting_info: Some(PlayerSettingInfo::default()),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_multi_path_avatar_info_cs_req(
    session: &PlayerSession, 
    _body: &GetMultiPathAvatarInfoCsReq
) -> Result<()> {
    let player_info = session.player_info();
    let multipath_basic_type_bin = player_info.data.multipath_bin.as_ref().unwrap();
    tracing::info!("march_type: {0}001", multipath_basic_type_bin.cur_march_path);

    session.send(
        CMD_GET_MULTI_PATH_AVATAR_INFO_SC_RSP, 
        GetMultiPathAvatarInfoScRsp {
            retcode: Retcode::RetSucc as u32,
            cur_multi_path_avatar_type_map: HashMap::from([
                (8001, multipath_basic_type_bin.cur_mc_path.into()),
                (1001, multipath_basic_type_bin.cur_march_path.into()),
            ]),
            multipath_type_info_list: multipath_basic_type_bin
            .multipath_type_list
            .iter()
            .flat_map(|tb| {
                let mut avatars = Vec::new();

                if tb.basic_type == 8001 {
                    avatars.push(MultiPathAvatar {
                        basic_type: multipath_basic_type_bin.cur_mc_path,
                        rank: tb.rank,
                        equip_relic_list: Vec::new(),
                        skill_tree_list: EXCEL_COLLECTION
                            .avatar_skill_tree_configs
                            .iter()
                            .filter(|c| c.avatar_id == multipath_basic_type_bin.cur_mc_path as u32)
                            .map(|c| AvatarSkillTree {
                                point_id: c.point_id,
                                level: c.max_level,
                            })
                            .collect(),
                        ..Default::default()
                    });
                }

                if tb.basic_type == 1001 {
                    let cur_march_path_str = multipath_basic_type_bin.cur_march_path;

                    avatars.push(MultiPathAvatar {
                        basic_type: multipath_basic_type_bin.cur_march_path,
                        rank: tb.rank,
                        equip_relic_list: Vec::new(),
                        skill_tree_list: EXCEL_COLLECTION
                            .avatar_skill_tree_configs
                            .iter()
                            .filter(|c| c.avatar_id == multipath_basic_type_bin.cur_march_path as u32)
                            .map(|c| {
                                let point_id_str = format!("{}{}", cur_march_path_str, &c.point_id.to_string()[4..]);
                                let point_ids = point_id_str.parse::<u32>().ok()?;
                
                                Some(AvatarSkillTree {
                                    point_id: point_ids,
                                    level: c.max_level,
                                })
                            })
                            .filter_map(|item| item)
                            .collect(),
                        ..Default::default()
                    });
                }

                avatars
            })
            .collect(),
            ..Default::default()
        }
    ).await?;

    Ok(())
}

// pub async fn on_get_hero_basic_type_info_cs_req(
//     session: &PlayerSession,
//     _body: &GetHeroBasicTypeInfoCsReq,
// ) -> Result<()> {
//     let player_info = session.player_info();
//     let hero_basic_type_bin = player_info.data.hero_bin.as_ref().unwrap();

//     session
//         .send(
//             CMD_GET_HERO_BASIC_TYPE_INFO_SC_RSP,
//             GetHeroBasicTypeInfoScRsp {
//                 retcode: Retcode::RetSucc as u32,
//                 gender: hero_basic_type_bin.gender.into(),
//                 cur_basic_type: hero_basic_type_bin.cur_basic_type.into(),
//                 basic_type_info_list: hero_basic_type_bin
//                     .basic_type_list
//                     .iter()
//                     .map(|b| HeroBasicTypeInfo {
//                         basic_type: b.basic_type.into(),
//                         rank: b.rank,
//                         skill_tree_list: EXCEL_COLLECTION
//                             .avatar_skill_tree_configs
//                             .iter()
//                             .filter(|c| c.avatar_id == hero_basic_type_bin.cur_basic_type as u32)
//                             .map(|c| AvatarSkillTree {
//                                 point_id: c.point_id,
//                                 level: c.max_level,
//                             })
//                             .collect(),
//                         ..Default::default()
//                     })
//                     .collect(),
//                 ..Default::default()
//             },
//         )
//         .await
// }

pub async fn on_player_heart_beat_cs_req(
    session: &PlayerSession,
    body: &PlayerHeartBeatCsReq,
) -> Result<()> {
    session
        .context
        .on_player_heartbeat(body.client_time_ms)
        .await?;

    let info = os_info::get();
    let os_info = info.os_type();
    let uid = session.context.get_uid();
    let uid_script = format!(
        "
        local version = CS.UnityEngine.Application.version
        CS.UnityEngine.GameObject.Find(\"UIRoot/AboveDialog/BetaHintDialog(Clone)\"):GetComponentInChildren(typeof(CS.RPG.Client.LocalizedText)).text = \"<size=25><color=#f7ff8a>Game Version: CNBETA{}\"..version..\" | UID: {} </color></size>\"
        CS.UnityEngine.GameObject.Find(\"VersionText\"):GetComponentInChildren(typeof(CS.RPG.Client.LocalizedText)).text = \"<size=15><color=#FFC0CB>FireflySR is a free software made by xeon. discord: reversedrooms</color></size>\"
        ",
        os_info,
        uid
    );

    // Encode the script as base64
    let base64_string = rbase64::encode(uid_script.as_bytes());

    // Decode the base64 string back into bytes
    let decoded_bytes = match rbase64::decode(&base64_string) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to decode base64 string: {}", e);
            return Ok(());
        }
    };

    let download_data_value = ClientDownloadData {
        version: 51,
        time: util::cur_timestamp_ms() as i64,
        data: decoded_bytes,
    };    

    session
        .send(
            CMD_PLAYER_HEART_BEAT_SC_RSP,
            PlayerHeartBeatScRsp {
                retcode: 0,
                client_time_ms: body.client_time_ms,
                server_time_ms: util::cur_timestamp_ms(),
                download_data: Some(download_data_value),
            },
        )
        .await
}
