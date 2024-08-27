use super::*;

pub async fn on_get_all_lineup_data_cs_req(
    session: &PlayerSession,
    _body: &GetAllLineupDataCsReq,
) -> Result<()> {
    let lineup_mgr = session.context.lineup_mgr.borrow();

    let player_info = session.player_info();
    let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

    session
        .send(
            CMD_GET_ALL_LINEUP_DATA_SC_RSP,
            GetAllLineupDataScRsp {
                retcode: 0,
                cur_index: lineup_comp.cur_lineup_index,
                lineup_list: lineup_mgr.get_all_lineup_proto(),
            },
        )
        .await
}

pub async fn on_get_cur_lineup_data_cs_req(
    session: &PlayerSession,
    _body: &GetCurLineupDataCsReq,
) -> Result<()> {
    let lineup_mgr = session.context.lineup_mgr.borrow();

    session
        .send(
            CMD_GET_CUR_LINEUP_DATA_SC_RSP,
            GetCurLineupDataScRsp {
                retcode: 0,
                lineup: Some(lineup_mgr.cur_lineup_proto()),
            },
        )
        .await
}

pub async fn on_change_lineup_leader_cs_req(
    session: &PlayerSession,
    body: &ChangeLineupLeaderCsReq,
) -> Result<()> {
    if let Err(retcode) = session
        .context
        .lineup_mgr
        .borrow()
        .set_cur_lineup_leader(body.slot)
    {
        return session
            .send(
                CMD_CHANGE_LINEUP_LEADER_SC_RSP,
                ChangeLineupLeaderScRsp {
                    retcode: retcode as u32,
                    ..Default::default()
                },
            )
            .await;
    }

    session
        .send(
            CMD_CHANGE_LINEUP_LEADER_SC_RSP,
            ChangeLineupLeaderScRsp {
                slot: body.slot,
                retcode: 0,
            },
        )
        .await
}

pub async fn on_join_lineup_cs_req(session: &PlayerSession, body: &JoinLineupCsReq) -> Result<()> {
    let lineup_mgr = session.context.lineup_mgr.borrow();

    if let Err(retcode) = lineup_mgr.join_lineup(body.index, body.slot, body.base_avatar_id) {
        return session
            .send(
                CMD_JOIN_LINEUP_SC_RSP,
                JoinLineupScRsp {
                    retcode: retcode as u32,
                },
            )
            .await;
    }

    lineup_mgr.sync_cur_lineup(session).await?;

    let mut scene_mgr = session.context.scene_mgr.borrow_mut();
    if let Some(refresh_info) = scene_mgr.refresh_actor_group() {
        session
            .send(
                CMD_SCENE_GROUP_REFRESH_SC_NOTIFY,
                SceneGroupRefreshScNotify {
                    refresh_group_list: vec![refresh_info],
                },
            )
            .await?;
    }

    session
        .send(CMD_JOIN_LINEUP_SC_RSP, JoinLineupScRsp::default())
        .await
}

pub async fn on_replace_lineup_cs_req(
    session: &PlayerSession,
    body: &ReplaceLineupCsReq,
) -> Result<()> {
    let lineup_mgr = session.context.lineup_mgr.borrow();

    if let Err(retcode) =
        lineup_mgr.replace_lineup(body.index, body.leader_slot, &body.replace_slot_list)
    {
        return session
            .send(
                CMD_REPLACE_LINEUP_SC_RSP,
                ReplaceLineupScRsp {
                    retcode: retcode as u32,
                },
            )
            .await;
    }

    let mut scene_mgr = session.context.scene_mgr.borrow_mut();
    if let Some(refresh_info) = scene_mgr.refresh_actor_group() {
        session
            .send(
                CMD_SCENE_GROUP_REFRESH_SC_NOTIFY,
                SceneGroupRefreshScNotify {
                    refresh_group_list: vec![refresh_info],
                },
            )
            .await?;
    }

    lineup_mgr.sync_cur_lineup(session).await?;
    session
        .send(CMD_REPLACE_LINEUP_SC_RSP, ReplaceLineupScRsp::default())
        .await
}

pub async fn on_quit_lineup_cs_req(session: &PlayerSession, body: &QuitLineupCsReq) -> Result<()> {
    let lineup_mgr = session.context.lineup_mgr.borrow();

    if let Err(retcode) = lineup_mgr.quit_lineup(body.index, body.base_avatar_id) {
        return session
            .send(
                CMD_QUIT_LINEUP_SC_RSP,
                QuitLineupScRsp {
                    retcode: retcode as u32,
                    ..Default::default()
                },
            )
            .await;
    }

    session
        .send(
            CMD_QUIT_LINEUP_SC_RSP,
            QuitLineupScRsp {
                plane_id: body.plane_id,
                is_mainline: !body.is_virtual,
                is_virtual: body.is_virtual,
                base_avatar_id: body.base_avatar_id,
                retcode: 0,
            },
        )
        .await
}
