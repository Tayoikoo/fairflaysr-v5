use anyhow::Result;
use prost::Message;
use proto::PlayerDataBin;

pub fn serialize_player_bin(player_bin: &PlayerDataBin) -> String {
    let data = player_bin.encode_to_vec();
    rbase64::encode(&data)
}

pub fn deserialize_player_bin(data_b64: &str) -> Result<PlayerDataBin> {
    let data = rbase64::decode(data_b64)?;
    let player_bin = PlayerDataBin::decode(&*data)?;

    Ok(player_bin)
}
