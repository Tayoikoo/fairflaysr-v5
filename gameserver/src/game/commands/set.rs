use super::*;
use common::data::EXCEL_COLLECTION;

const SET_SKILL_USAGE: &'static str = "Usage: /set avatar max_trace [avatar_id]";
const SET_EIDOLON_USAGE: &'static str = "Usage: /set avatar eidolon [avatar_id] [1-6]";
const SET_ENERGY_USAGE: &'static str = "Usage: /set avatar energy [1-10000]";
const SET_MONSTER_LEVEL_USAGE: &'static str = "Usage: /set monster level [1-100]";
const SET_STAGE_ID_USAGE: &'static str = "Usage: /set stage id [stage_id]";
const SET_CYCLE_COUNT_USAGE: &'static str = "Usage: /set stage cycle [1-30]";

pub async fn set_command(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() {
        return send_text(session, "Usage: /set [avatar|monster|stage] [args]").await;
    }

    match args[0] {
        "avatar" => match args.get(1) {
            Some(&"max_trace") => max_traces(&args[2..], session).await,
            Some(&"eidolon") => eidolon(&args[2..], session).await,
            Some(&"energy") => set_energy(&args[2..], session).await,
            _ => send_text(session, "Usage: /set avatar [max_trace|eidolon|energy] [args]").await,
        },
        "monster" => match args.get(1) {
            Some(&"level") => set_monster_level(&args[2..], session).await,
            _ => send_text(session, "Usage: /set monster [level] [args]").await,
        },
        "stage" => match args.get(1) {
            Some(&"id") => set_stage_id(&args[2..], session).await,
            Some(&"cycle") => set_stage_cycle(&args[2..], session).await,
            _ => send_text(session, "Usage: /set stage [id|cycle] [args]").await,
        },        
        _ => send_text(session, "Usage: /set [avatar|monster|stage] [args]").await,
    }
}

async fn max_traces(args: &[&str], session: &PlayerSession) -> Result<()> {
    let Some(Ok(avatar_id)) = args.get(0).map(|s| s.parse::<u32>()) else {
        return send_text(session, SET_SKILL_USAGE).await;
    };

    {
        let mut player_info = session.context.player.borrow_mut();
        let avatar_comp = player_info.data.avatar_bin.as_mut().unwrap();

        let Some(avatar) = avatar_comp
            .avatar_list
            .iter_mut()
            .find(|a| a.avatar_id == avatar_id)
        else {
            return send_text(session, &format!("Avatar {avatar_id} doesn't exist")).await;
        };

        EXCEL_COLLECTION
            .avatar_skill_tree_configs
            .iter()
            .filter(|c| c.avatar_id == avatar_id)
            .map(|c| (c.point_id, c.max_level))
            .for_each(|(pt, lv)| {
                if let Some(skill_tree) = avatar
                    .skill_tree_list
                    .iter_mut()
                    .find(|st| st.point_id == pt)
                {
                    skill_tree.level = lv
                } else {
                    avatar.skill_tree_list.push(AvatarSkillTreeBin {
                        point_id: pt,
                        level: lv,
                    })
                }
            });        
    }
    
    let avatar_mgr = session.context.avatar_mgr.borrow();
    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            PlayerSyncScNotify {
                avatar_sync: Some(AvatarSync {
                    avatar_list: avatar_mgr.avatar_list_proto(),
                }),
                ..Default::default()
            },
        )
        .await?;

    send_text(
        session,
        &format!("Successfully maxed out traces of avatar {avatar_id}"),
    )
    .await
}

async fn eidolon(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() {
        return send_text(session, SET_EIDOLON_USAGE).await;
    }

    let avatar_id = args[0].parse::<u32>()?;
    let mut rank = args[1].parse::<u32>()?;
    
    rank = rank.clamp(0, 6);

    let avatar_mgr = session.context.avatar_mgr.borrow();
    avatar_mgr.set_rank(avatar_id, rank)?;
    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            PlayerSyncScNotify {
                avatar_sync: Some(AvatarSync {
                    avatar_list: avatar_mgr.avatar_list_proto(),
                }),
                ..Default::default()
            },
        )
        .await?;
    session
        .send(CMD_DRESS_AVATAR_SC_RSP, DressAvatarScRsp::default())
        .await?;

    send_text(session, &format!("Avatar {avatar_id} is set to E{rank}")).await
}

pub async fn set_monster_level(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() {
        return send_text(session, SET_MONSTER_LEVEL_USAGE).await;
    }

    let mut level = match args[0].parse::<u32>() {
        Ok(level) => level,
        Err(_) => return send_text(session, "Invalid level. Please enter a valid number.").await,
    };

    level = level.clamp(1,100);

    {
        let game_context = session.context.clone();
        let mut monster_level = game_context.monster_level.lock().unwrap();
        *monster_level = level;
    }
    send_text(session, &format!("Monster level set to {}", level)).await
}

pub async fn set_stage_cycle(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() {
        return send_text(session, SET_CYCLE_COUNT_USAGE).await;
    }

    let mut c_count = match args[0].parse::<u32>() {
        Ok(c_count) => c_count,
        Err(_) => return send_text(session, "Invalid count. Please enter a valid number.").await,
    };
        c_count = c_count.clamp(1, 30);
    {
        let game_context = session.context.clone();
        let mut cycle_count = game_context.cycle_count.lock().unwrap();
        *cycle_count = c_count;
    }

    send_text(session, &format!("Battle Cycle Count Set to {}", c_count)).await
}

pub async fn set_stage_id(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() {
        return send_text(session, SET_STAGE_ID_USAGE).await;
    }

    let s_id = match args[0].parse::<u32>() {
        Ok(s_id) => s_id,
        Err(_) => return send_text(session, "Invalid count. Please enter a valid number.").await,
    };

    {
        let game_context = session.context.clone();
        let mut stageid = game_context.stage_id.lock().unwrap();
        *stageid = s_id;
    }

    send_text(session, &format!("Battle Stage ID Set to {}", s_id)).await
}

pub async fn set_energy(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() {
        return send_text(session, SET_ENERGY_USAGE).await;
    }

    let mut e_amount = match args[0].parse::<u32>() {
        Ok(e_amount) => e_amount,
        Err(_) => return send_text(session, "Invalid count. Please enter a valid number.").await,
    };
    
    e_amount = e_amount.clamp(1, 10000);

    {
        let game_context = session.context.clone();
        let mut energy_amount = game_context.energy.lock().unwrap();
        *energy_amount = e_amount;
    }

    send_text(session, &format!("Battle Avatar Energy Set to {}", e_amount)).await
}