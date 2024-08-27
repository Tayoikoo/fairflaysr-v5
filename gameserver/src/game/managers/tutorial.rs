use common::data::EXCEL_COLLECTION;
use proto::{
    PlayerTutorialCompBin, Retcode, Tutorial, TutorialBin, TutorialGuide, TutorialGuideBin,
    TutorialStatus,
};

use super::*;

pub struct TutorialManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
}

impl TutorialManager {
    pub fn new(player_info: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self { player_info }
    }

    pub fn init_defaults(&self) {
        let mut player_info = self.player_info.borrow_mut();

        player_info.data.tutorial_bin = Some(PlayerTutorialCompBin {
            tutorial_list: EXCEL_COLLECTION
                .tutorial_configs
                .iter()
                .map(|c| TutorialBin {
                    id: c.tutorial_id,
                    status: TutorialStatus::TutorialFinish.into(),
                })
                .collect(),
            tutorial_guide_list: Vec::new(),
        });
    }

    pub fn unlock_tutorial_guide(&self, group_id: u32) -> Result<TutorialGuide, Retcode> {
        if !EXCEL_COLLECTION
            .tutorial_guide_group_configs
            .iter()
            .any(|c| c.group_id == group_id)
        {
            return Err(Retcode::RetTutorialTutorialNotFound);
        }

        let mut player_info = self.player_info.borrow_mut();
        let tutorial_comp = player_info.data.tutorial_bin.as_mut().unwrap();

        if tutorial_comp
            .tutorial_guide_list
            .iter()
            .any(|t| t.group_id == group_id)
        {
            return Err(Retcode::RetTutorialUnlockAlready);
        }

        tutorial_comp.tutorial_guide_list.push(TutorialGuideBin {
            group_id,
            status: TutorialStatus::TutorialUnlock.into(),
        });

        Ok(TutorialGuide {
            id: group_id,
            status: TutorialStatus::TutorialUnlock.into(),
        })
    }

    pub fn tutorial_list_proto(&self) -> Vec<Tutorial> {
        let player_info = self.player_info.borrow();
        let tutorial_comp = player_info.data.tutorial_bin.as_ref().unwrap();

        tutorial_comp
            .tutorial_list
            .iter()
            .map(|t| Tutorial {
                id: t.id,
                status: t.status,
            })
            .collect()
    }

    pub fn tutorial_guide_list_proto(&self) -> Vec<TutorialGuide> {
        let player_info = self.player_info.borrow();
        let tutorial_comp = player_info.data.tutorial_bin.as_ref().unwrap();

        tutorial_comp
            .tutorial_guide_list
            .iter()
            .map(|t| TutorialGuide {
                id: t.group_id,
                status: t.status,
            })
            .collect()
    }
}
