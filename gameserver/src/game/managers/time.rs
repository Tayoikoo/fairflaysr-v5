use crate::util;

pub struct TimeManager {
    pub client_time_ms: u64,
    pub server_time_ms: u64,
    pub last_save_time: u64,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            client_time_ms: 0,
            server_time_ms: 0,
            last_save_time: 0,
        }
    }

    pub fn update(&mut self, client_time_ms: u64) {
        self.client_time_ms = client_time_ms;
        self.server_time_ms = util::cur_timestamp_ms();
    }

    pub fn on_player_save(&mut self) {
        self.last_save_time = util::cur_timestamp_seconds();
    }

    pub fn seconds_since_last_save(&self) -> u64 {
        util::cur_timestamp_seconds() - self.last_save_time
    }
}
