use super::*;
use anyhow::Result;
use crate::{net::PlayerSession};
// use rand::{thread_rng, Rng};

const GIVE_RELIC_USAGE: &'static str = "Usage: /give relic [id] [level] [main_affix] [sub_affix_count] [sub_affix1_id]:[sub_affix1_cnt] ...";
const GIVE_MAT_USAGE: &'static str = "Usage: /give material [id] [quantity]";
const GIVE_EQUIP_USAGE: &'static str = "Usage: /give equipment [id] [level] [rank] [promotion]";

pub async fn give_command(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() {
        return send_text(session, "Usage: /give [material|relic|equipment]").await;
    }

    match args[0] {
        "relic" => give_relic_cmd(&args[1..], session).await,
        "material" => give_material(&args[1..], session).await,
        "equipment" => give_equip_item(&args[1..], session).await,
        _ => send_text(session, "Usage: /give [material|relic|equipment]").await,
    }
}

async fn give_relic_cmd(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.len() < 3 {
        return send_text(session, GIVE_RELIC_USAGE).await;
    }

    let id = args[0].parse::<u32>()?;
    let level = args[1].parse::<u32>()?;
    let main_affix = args[2].parse::<u32>()?;
    let sub_affix_count = args[3].parse::<usize>()?;

    // let mut is_rng;
    let mut sub_affix_params = Vec::with_capacity(sub_affix_count);

    // if sub_affix_count <= 0 || sub_affix_count > 4 {
    //     let mut rng = thread_rng();
    //     sub_affix_params = Vec::with_capacity(rng.gen_range(1..=4));
    //     is_rng = 1;
    //     for _ in 0..4 {
    //         let mut sub_affix_id = rng.gen_range(1..=12);
    //         while sub_affix_id == main_affix {
    //             sub_affix_id = rng.gen_range(1..=12);
    //         }
    //         let sub_affix_cnt = rng.gen_range(1..=5);
    //         sub_affix_params.push((sub_affix_id, sub_affix_cnt));
    //     }
    if args.is_empty() {
        return send_text(session, GIVE_RELIC_USAGE).await;
    }

    let sub_affix_args = &args[4..];
    for i in 0..sub_affix_count {
        let sub_affix_data: Vec<&str> = sub_affix_args[i].split(':').collect();
        if sub_affix_data.len() != 2 {
            return send_text(session, GIVE_RELIC_USAGE).await;
        }

        let sub_affix_id = sub_affix_data[0].parse::<u32>()?;
        let sub_affix_cnt = sub_affix_data[1].parse::<u32>()?;
        sub_affix_params.push((sub_affix_id, sub_affix_cnt));
    }

    let item_mgr = session.context.item_mgr.borrow();
    item_mgr.give_relic(id, level, main_affix, sub_affix_params)?;

    session.send(
        CMD_PLAYER_SYNC_SC_NOTIFY,
        PlayerSyncScNotify {
            relic_list: item_mgr.relic_list_proto(),
            ..Default::default()
        },
    ).await?;

    send_text(session, &format!("Relic with ID {id}, Main Stat ID: {main_affix} has been added successfully")).await
}

// to fix same Substat as main stat (please make it work)
// Update: doesnt work :(
// fn generate_unique_affix(main_affix: u32, existing_affixes: &Vec<(u32, u32)>) -> u32 {
//     let mut rng = thread_rng();
//     loop {
//         let sub_affix_id = rng.gen_range(1..=12);
//         if sub_affix_id != main_affix && !existing_affixes.iter().any(|(id, _)| *id == sub_affix_id) {
//             return sub_affix_id;
//         }
//     }
// }

async fn give_material(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.len() < 2 {
        return send_text(session, GIVE_MAT_USAGE).await;
    }

    let id = args[0].parse::<u32>()?;
    let quantity = args[1].parse::<u32>()?;

    let item_mgr = session.context.item_mgr.borrow();
    item_mgr.add_material(id, quantity)?;

    session.send(
        CMD_PLAYER_SYNC_SC_NOTIFY,
        PlayerSyncScNotify {
            material_list: item_mgr.material_list_proto(),
            ..Default::default()
        },
    ).await?;

    send_text(session, &format!("Material ID {id} with amount {quantity} has been added successfully")).await
}

async fn give_equip_item(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.len() < 4 {
        return send_text(session, GIVE_EQUIP_USAGE).await;
    }

    let tid = args[0].parse::<u32>()?;
    let mut level = args[1].parse::<u32>()?;
    let mut rank = args[2].parse::<u32>()?;
    let mut promotion = args[3].parse::<u32>()?;

    level = level.clamp(1, 80);
    rank = rank.clamp(1, 5);
    promotion = promotion.clamp(1, 6);

    let item_mgr = session.context.item_mgr.borrow();
    item_mgr.give_equipment_cmd(tid, level, rank, promotion)?;

    session.send(
        CMD_PLAYER_SYNC_SC_NOTIFY,
        PlayerSyncScNotify {
            equipment_list: item_mgr.equipment_list_proto(),
            ..Default::default()
        },
    ).await?;

    send_text(session, &format!("Equipment ID {tid}, Level {level}, Rank {rank}, Promotion {promotion} has been added successfully")).await
}
