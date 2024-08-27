use super::*;

pub async fn on_get_friend_list_info_cs_req(
    session: &PlayerSession,
    _body: &GetFriendListInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_FRIEND_LIST_INFO_SC_RSP,
            GetFriendListInfoScRsp {
                retcode: 0,
                friend_info_list: vec![FriendInfo {
                    player_simple_info: Some(PlayerSimpleInfo {
                        nickname: String::from("Mommy Kafka"),
                        level: 70,
                        uid: 13371337,
                        head_icon: 201005,
                        assist_simple_info_list: vec![AssistSimpleInfo {
                            avatar_id: 1005,
                            level: 80,
                            dressed_skin_id: 0,
                            pos: 0,
                        }],
                        platform: PlatformType::Pc.into(),
                        online_status: FriendOnlineStatus::Online.into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await
}
