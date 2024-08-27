use super::*;

pub async fn on_get_scene_map_info_cs_req(
    session: &PlayerSession,
    body: &GetSceneMapInfoCsReq,
) -> Result<()> {
    let mut map_infos = Vec::new();

    for entry_id in &body.entry_id_list {
        let map_info = SceneMapInfo {
            entry_id: *entry_id,
            ..Default::default()
        };
        map_infos.push(map_info);
    }

    session
        .send(
            CMD_GET_SCENE_MAP_INFO_SC_RSP,
            GetSceneMapInfoScRsp {
                map_info_list: map_infos,
                retcode: 0,
                ..Default::default()
            },
        )
        .await
}
