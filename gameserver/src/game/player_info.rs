use proto::*;

use crate::util;

pub struct PlayerInfo {
    pub uid: u32,
    pub data: PlayerDataBin,
}

impl PlayerInfo {
    pub fn new() -> Self {
        Self {
            uid: 0,
            data: PlayerDataBin::default(),
        }
    }

    pub fn init_player_data(&mut self) {
        self.data = PlayerDataBin {
            basic_bin: Some(PlayerBasicCompBin {
                level: 70,
                nickname: String::from("FeixiaoSEGS"),
                created_timestamp: util::cur_timestamp_ms() as i64,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn get_mc_path_type(&self) -> i32 {
        self.data.multipath_bin.as_ref().unwrap().cur_mc_path
    }

    pub fn get_march7th_path_type(&self) -> i32 {
        self.data.multipath_bin.as_ref().unwrap().cur_march_path
    }

    // pub fn get_cur_basic_type(&self) -> i32 {
    //     self.data.hero_bin.as_ref().unwrap().cur_basic_type
    // }
}
