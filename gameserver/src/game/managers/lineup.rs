use proto::*;

use crate::net::PlayerSession;

use super::*;

pub struct LineupManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
}

impl LineupManager {
    const MP_LIMIT: u32 = 5;
    const AVATAR_SLOT_COUNT: u32 = 4;
    const DEFAULT_LINEUP_AVATARS: [u32; 4] = [1310, 0, 0, 0]; // firefly

    pub fn new(player_info: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self { player_info }
    }

    pub fn init_defaults(&self) {
        let mut player_info = self.player_info.borrow_mut();
        let avatar_comp = player_info.data.avatar_bin.as_ref().unwrap();

        player_info.data.lineup_bin = Some(PlayerLineupCompBin {
            lineup_list: vec![LineupBin {
                avatar_list: Self::DEFAULT_LINEUP_AVATARS
                    .iter()
                    .enumerate()
                    .filter(|(_, id)| **id != 0)
                    .map(|(i, id)| {
                        let avatar_bin = avatar_comp
                            .avatar_list
                            .iter()
                            .find(|a| a.avatar_id == *id)
                            .expect("Default lineup avatar is not present in PlayerAvatarCompBin");

                        LineupAvatarBin {
                            avatar_id: avatar_bin.avatar_id,
                            avatar_type: avatar_bin.avatar_type,
                            slot: i as u32,
                            hp: 10000,
                            sp: 10000,
                        }
                    })
                    .collect(),
                ..Default::default()
            }],
            cur_lineup_index: 0,
            mp: Self::MP_LIMIT,
            mp_max: Self::MP_LIMIT,
        });
    }

    pub fn set_cur_lineup_leader(&self, leader_slot: u32) -> Result<(), Retcode> {
        if !(0..Self::AVATAR_SLOT_COUNT).contains(&leader_slot) {
            return Err(Retcode::RetLineupNotValidLeader);
        }

        let mut player_info = self.player_info.borrow_mut();
        let lineup_comp = player_info.data.lineup_bin.as_mut().unwrap();

        let Some(lineup) = lineup_comp
            .lineup_list
            .iter_mut()
            .find(|l| l.index == lineup_comp.cur_lineup_index)
        else {
            return Err(Retcode::RetLineupIsEmpty);
        };

        lineup.leader_slot = leader_slot;
        Ok(())
    }

    pub fn join_lineup(&self, index: u32, slot: u32, base_avatar_id: u32) -> Result<(), Retcode> {
        if !(0..Self::AVATAR_SLOT_COUNT).contains(&slot) {
            return Err(Retcode::RetLineupInvalidMemberPos);
        }

        let mut player_info = self.player_info.borrow_mut();
        let lineup_comp = player_info.data.lineup_bin.as_mut().unwrap();
        let lineup_bin = lineup_comp
            .lineup_list
            .iter_mut()
            .find(|l| l.index == index)
            .ok_or(Retcode::RetLineupNotExist)?;

        if lineup_bin
            .avatar_list
            .iter()
            .any(|avatar| avatar.slot == slot)
        {
            return Err(Retcode::RetLineupAvatarAlreadyInit);
        }

        lineup_bin
            .avatar_list
            .push(Self::lineup_avatar_bin(base_avatar_id, slot));

        Ok(())
    }

    pub fn quit_lineup(&self, index: u32, base_avatar_id: u32) -> Result<(), Retcode> {
        let mut player_info = self.player_info.borrow_mut();
        let lineup_comp = player_info.data.lineup_bin.as_mut().unwrap();

        let lineup_bin = lineup_comp
            .lineup_list
            .iter_mut()
            .find(|l| l.index == index)
            .ok_or(Retcode::RetLineupNotExist)?;

        lineup_bin
            .avatar_list
            .retain(|a| a.avatar_id != base_avatar_id);

        Ok(())
    }

    pub fn replace_lineup(
        &self,
        index: u32,
        leader_slot: u32,
        replace_slot_list: &[LineupSlotInfo],
    ) -> Result<(), Retcode> {
        if !(0..Self::AVATAR_SLOT_COUNT).contains(&leader_slot) {
            return Err(Retcode::RetLineupInvalidMemberPos);
        }

        let mut player_info = self.player_info.borrow_mut();
        let lineup_comp = player_info.data.lineup_bin.as_mut().unwrap();
        let lineup_bin = lineup_comp
            .lineup_list
            .iter_mut()
            .find(|l| l.index == index)
            .ok_or(Retcode::RetLineupNotExist)?;

        lineup_bin.avatar_list.clear();
        for slot_info in replace_slot_list {
            if !(0..Self::AVATAR_SLOT_COUNT).contains(&slot_info.slot) {
                return Err(Retcode::RetLineupInvalidMemberPos);
            }

            lineup_bin
                .avatar_list
                .push(Self::lineup_avatar_bin(slot_info.id, slot_info.slot));
        }

        lineup_bin.leader_slot = leader_slot;
        Ok(())
    }

    pub async fn sync_cur_lineup(&self, session: &PlayerSession) -> anyhow::Result<()> {
        session
            .send(
                CMD_SYNC_LINEUP_NOTIFY,
                SyncLineupNotify {
                    lineup: Some(self.cur_lineup_proto()),
                    ..Default::default()
                },
            )
            .await
    }

    pub fn cur_lineup_proto(&self) -> LineupInfo {
        let player_info = self.player_info.borrow();
        let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

        self.get_lineup_proto(lineup_comp.cur_lineup_index).unwrap()
    }

    pub fn get_lineup_proto(&self, index: u32) -> Option<LineupInfo> {
        let player_info = self.player_info.borrow();
        let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

        lineup_comp
            .lineup_list
            .iter()
            .find(|l| l.index == index)
            .map(|l| LineupInfo {
                avatar_list: l
                    .avatar_list
                    .iter()
                    .map(|a| LineupAvatar {
                        id: a.avatar_id,
                        avatar_type: a.avatar_type,
                        slot: a.slot,
                        hp: a.hp,
                        sp: Some(AmountInfo {
                            cur_amount: a.sp,
                            max_amount: 10000,
                        }),
                        ..Default::default()
                    })
                    .collect(),
                index: l.index,
                name: l.name.clone(),
                is_virtual: l.is_virtual,
                plane_id: l.plane_id,
                mp: lineup_comp.mp,
                mp_max: lineup_comp.mp_max,
                leader_slot: l.leader_slot,
                extra_lineup_type: l.extra_lineup_type.into(),
                ..Default::default()
            })
    }

    pub fn get_all_lineup_proto(&self) -> Vec<LineupInfo> {
        let player_info = self.player_info.borrow();
        let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

        lineup_comp
            .lineup_list
            .iter()
            .enumerate()
            .map(|(i, _)| self.get_lineup_proto(i as u32))
            .flatten()
            .collect()
    }

    pub fn refill_skillpoint(&self) -> LineupInfo {
        {
            let mut player_info = self.player_info.borrow_mut();
            let lineup_bin = player_info.data.lineup_bin.as_mut().unwrap();

            // Refill mp to the maximum limit
            lineup_bin.mp = Self::MP_LIMIT;
        } // Mutable borrow ends here

        // Return the current lineup information to ensure it includes all characters
        self.cur_lineup_proto()
    }

    #[must_use]
    const fn lineup_avatar_bin(id: u32, slot: u32) -> LineupAvatarBin {
        LineupAvatarBin {
            avatar_id: id,
            slot,
            hp: 10000,
            sp: 10000,
            avatar_type: 3,
        }
    }
}
