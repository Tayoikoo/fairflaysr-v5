use super::*;

pub async fn refill_sp(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() || args[0] != "sp" {
        return send_text(session, "Usage: /refill sp").await;
    }

    // Access the LineupManager from the session context
    let lineup_mgr = session.context.lineup_mgr.borrow();

    // Refill the MP and get the current lineup info, ensuring it includes all characters
    let updated_lineup = lineup_mgr.refill_skillpoint();

    // Send the updated lineup information to the player
    session.send(
        CMD_SYNC_LINEUP_NOTIFY,
        SyncLineupNotify {
            lineup: Some(updated_lineup),
            ..Default::default()
        },
    ).await?;

    // Confirmation message
    send_text(session, "MP has been refilled to the maximum").await?;

    Ok(())
}

// async fn refillsp(args: &[&str], session: &PlayerSession) -> Result<()> {
//     if args.is_empty() || args[0] != "sp" {
//         return send_text(session, "Usage: /refill sp").await;
//     }

//     // Access the LineupManager from the session context
//     let mut lineup_mgr = session.context.lineup_mgr.borrow_mut();

//     // Call the refill_mp method to refill to max
//     let new_mp = lineup_mgr.refill_skillpoint();

//     // Send the updated lineup information to the player
//     session.send(
//         CMD_SYNC_LINEUP_NOTIFY,
//         SyncLineupNotify {
//             lineup: Some(LineupInfo {
//                 mp: new_mp,
//                 ..Default::default()
//             }),
//             ..Default::default()
//         },
//     ).await?;

//     // Confirmation message
//     send_text(session, &format!("MP has been refilled to the maximum: {new_mp}")).await?;

//     Ok(())
// }