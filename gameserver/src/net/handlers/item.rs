use super::*;

pub async fn on_get_bag_cs_req(session: &PlayerSession, _body: &GetBagCsReq) -> Result<()> {
    let item_mgr = session.context.item_mgr.borrow();

    session
        .send(
            CMD_GET_BAG_SC_RSP,
            GetBagScRsp {
                equipment_list: item_mgr.equipment_list_proto(),
                // material_list: item_mgr.material_list_proto(),
                relic_list: item_mgr.relic_list_proto(),
                ..Default::default()
            },
        )
        .await
}
