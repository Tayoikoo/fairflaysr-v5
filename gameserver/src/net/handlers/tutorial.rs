use super::*;

pub async fn on_get_tutorial_cs_req(
    session: &PlayerSession,
    _body: &GetTutorialCsReq,
) -> Result<()> {
    let tutorial_mgr = session.context.tutorial_mgr.borrow();

    session
        .send(
            CMD_GET_TUTORIAL_SC_RSP,
            GetTutorialScRsp {
                retcode: 0,
                tutorial_list: tutorial_mgr.tutorial_list_proto(),
            },
        )
        .await
}

pub async fn on_get_tutorial_guide_cs_req(
    session: &PlayerSession,
    _body: &GetTutorialGuideCsReq,
) -> Result<()> {
    let tutorial_mgr = session.context.tutorial_mgr.borrow();

    session
        .send(
            CMD_GET_TUTORIAL_GUIDE_SC_RSP,
            GetTutorialGuideScRsp {
                retcode: 0,
                tutorial_guide_list: tutorial_mgr.tutorial_guide_list_proto(),
            },
        )
        .await
}

pub async fn on_unlock_tutorial_guide_cs_req(
    session: &PlayerSession,
    body: &UnlockTutorialGuideCsReq,
) -> Result<()> {
    let tutorial_mgr = session.context.tutorial_mgr.borrow();

    match tutorial_mgr.unlock_tutorial_guide(body.group_id) {
        Ok(tutorial_guide) => {
            session
                .send(
                    CMD_UNLOCK_TUTORIAL_GUIDE_SC_RSP,
                    UnlockTutorialGuideScRsp {
                        retcode: 0,
                        tutorial_guide: Some(tutorial_guide),
                    },
                )
                .await
        }
        Err(retcode) => {
            session
                .send(
                    CMD_UNLOCK_TUTORIAL_GUIDE_SC_RSP,
                    UnlockTutorialGuideScRsp {
                        retcode: retcode as u32,
                        ..Default::default()
                    },
                )
                .await
        }
    }
}
