use super::*;

pub async fn on_get_avatar_data_cs_req(
    session: &PlayerSession,
    body: &GetAvatarDataCsReq,
) -> Result<()> {
    let avatar_mgr = session.context.avatar_mgr.borrow();

    session
        .send(
            CMD_GET_AVATAR_DATA_SC_RSP,
            GetAvatarDataScRsp {
                is_all: body.is_get_all,
                avatar_list: avatar_mgr.avatar_list_proto(),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_dress_avatar_cs_req(
    session: &PlayerSession,
    body: &DressAvatarCsReq,
) -> Result<()> {
    let avatar_mgr = session.context.avatar_mgr.borrow();

    if let Err(retcode) = avatar_mgr.dress_equipment(body.equip_avatar_id, body.equipment_unique_id)
    {
        return session
            .send(
                CMD_DRESS_AVATAR_SC_RSP,
                DressAvatarScRsp {
                    retcode: retcode as u32,
                },
            )
            .await;
    }

    let item_mgr = session.context.item_mgr.borrow();
    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            PlayerSyncScNotify {
                avatar_sync: Some(AvatarSync {
                    avatar_list: avatar_mgr.avatar_list_proto(),
                }),
                equipment_list: item_mgr.equipment_list_proto(),
                ..Default::default()
            },
        )
        .await?;
    session
        .send(CMD_DRESS_AVATAR_SC_RSP, DressAvatarScRsp::default())
        .await
}

pub async fn on_take_off_equipment_cs_req(
    session: &PlayerSession,
    body: &TakeOffEquipmentCsReq,
) -> Result<()> {
    let avatar_mgr = session.context.avatar_mgr.borrow();
    if let Err(retcode) = avatar_mgr.take_off_equipment(body.equip_avatar_id) {
        return session
            .send(
                CMD_TAKE_OFF_EQUIPMENT_SC_RSP,
                TakeOffEquipmentScRsp {
                    retcode: retcode as u32,
                },
            )
            .await;
    };

    let item_mgr = session.context.item_mgr.borrow();
    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            PlayerSyncScNotify {
                avatar_sync: Some(AvatarSync {
                    avatar_list: avatar_mgr.avatar_list_proto(),
                }),
                equipment_list: item_mgr.equipment_list_proto(),
                ..Default::default()
            },
        )
        .await?;
    session
        .send(
            CMD_TAKE_OFF_EQUIPMENT_SC_RSP,
            TakeOffEquipmentScRsp::default(),
        )
        .await
}

pub async fn on_dress_relic_avatar_cs_req(
    session: &PlayerSession,
    body: &DressRelicAvatarCsReq,
) -> Result<()> {
    let avatar_mgr = session.context.avatar_mgr.borrow();
    for param in &body.param_list {
        if let Err(retcode) = avatar_mgr.dress_relic(
            body.equip_avatar_id,
            param.relic_unique_id,
            param.relic_slot,
        ) {
            return session
                .send(
                    CMD_DRESS_RELIC_AVATAR_SC_RSP,
                    DressRelicAvatarScRsp {
                        retcode: retcode as u32,
                    },
                )
                .await;
        }
    }

    let item_mgr = session.context.item_mgr.borrow();
    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            PlayerSyncScNotify {
                avatar_sync: Some(AvatarSync {
                    avatar_list: avatar_mgr.avatar_list_proto(),
                }),
                relic_list: item_mgr.relic_list_proto(),
                ..Default::default()
            },
        )
        .await?;

    session
        .send(
            CMD_DRESS_RELIC_AVATAR_SC_RSP,
            DressRelicAvatarScRsp::default(),
        )
        .await
}

pub async fn on_take_off_relic_cs_req(
    session: &PlayerSession,
    body: &TakeOffRelicCsReq,
) -> Result<()> {
    let avatar_mgr = session.context.avatar_mgr.borrow();
    for relic_type in &body.relic_type_list {
        if let Err(retcode) = avatar_mgr.take_off_relic(body.equip_avatar_id, *relic_type) {
            return session
                .send(
                    CMD_TAKE_OFF_RELIC_SC_RSP,
                    TakeOffRelicScRsp {
                        retcode: retcode as u32,
                    },
                )
                .await;
        }
    }

    let item_mgr = session.context.item_mgr.borrow();
    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            PlayerSyncScNotify {
                avatar_sync: Some(AvatarSync {
                    avatar_list: avatar_mgr.avatar_list_proto(),
                }),
                relic_list: item_mgr.relic_list_proto(),
                ..Default::default()
            },
        )
        .await?;

    session
        .send(CMD_TAKE_OFF_RELIC_SC_RSP, TakeOffRelicScRsp::default())
        .await
}