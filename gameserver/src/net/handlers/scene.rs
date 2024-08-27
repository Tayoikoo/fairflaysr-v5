use super::*;

pub async fn on_enter_scene_cs_req(session: &PlayerSession, body: &EnterSceneCsReq) -> Result<()> {
    let mut scene_mgr = session.context.scene_mgr.borrow_mut();
    match scene_mgr.enter_scene(body.entry_id) {
        Ok(scene_info) => {
            session
                .send(CMD_ENTER_SCENE_SC_RSP, EnterSceneScRsp::default())
                .await?;

            let lineup_mgr = session.context.lineup_mgr.borrow();

            let enter_scene_by_server = EnterSceneByServerScNotify {
                reason: EnterSceneReason::None.into(),
                lineup: Some(lineup_mgr.cur_lineup_proto()),
                scene: Some(scene_info),
            };

            session
                .send(CMD_ENTER_SCENE_BY_SERVER_SC_NOTIFY, enter_scene_by_server)
                .await
        }
        Err(retcode) => {
            session
                .send(
                    CMD_ENTER_SCENE_SC_RSP,
                    EnterSceneScRsp {
                        retcode: retcode as u32,
                        ..Default::default()
                    },
                )
                .await
        }
    }
}

pub async fn on_get_cur_scene_info_cs_req(
    session: &PlayerSession,
    _body: &GetCurSceneInfoCsReq,
) -> Result<()> {
    let scene_mgr = session.context.scene_mgr.borrow();

    session
        .send(
            CMD_GET_CUR_SCENE_INFO_SC_RSP,
            GetCurSceneInfoScRsp {
                retcode: 0,
                scene: Some(scene_mgr.cur_scene_info_proto()),
            },
        )
        .await
}

pub async fn on_scene_entity_move_cs_req(
    session: &PlayerSession,
    body: &SceneEntityMoveCsReq,
) -> Result<()> {
    let mut scene_mgr = session.context.scene_mgr.borrow_mut();
    body.entity_motion_list
        .iter()
        .filter(|e| e.motion.is_some())
        .for_each(|e| scene_mgr.entity_move(e));

    session
        .send(
            CMD_SCENE_ENTITY_MOVE_SC_RSP,
            SceneEntityMoveScRsp::default(),
        )
        .await
}

pub async fn on_enter_section_cs_req(
    session: &PlayerSession,
    body: &EnterSectionCsReq,
) -> Result<()> {
    let scene_mgr = session.context.scene_mgr.borrow();
    scene_mgr.enter_section(body.section_id);

    session
        .send(CMD_ENTER_SECTION_SC_RSP, EnterSectionScRsp::default())
        .await
}
