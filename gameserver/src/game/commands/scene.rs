use super::*;

pub async fn enter(args: &[&str], session: &PlayerSession) -> Result<()> {
    let Some(Ok(entry_id)) = args.get(0).map(|s| s.parse::<u32>()) else {
        return send_text(session, "Usage: /scene enter [entry_id]").await;
    };

    let mut scene_mgr = session.context.scene_mgr.borrow_mut();
    let scene = match scene_mgr.enter_scene(entry_id) {
        Ok(scene_info) => Some(scene_info),
        Err(_) => return send_text(session, &format!("Failed to enter scene {entry_id}.")).await,
    };

    let lineup_mgr = session.context.lineup_mgr.borrow();
    session
        .send(
            CMD_ENTER_SCENE_BY_SERVER_SC_NOTIFY,
            EnterSceneByServerScNotify {
                scene,
                lineup: Some(lineup_mgr.cur_lineup_proto()),
                reason: EnterSceneReason::None.into(),
            },
        )
        .await
}
