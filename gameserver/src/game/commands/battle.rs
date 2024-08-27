use super::*;
use crate::game::gameplay_config::reload_gameplay_config;


pub async fn battle_command(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() || args[0] != "refresh" {
        return send_text(session, "Usage: /battle refresh").await;
    }

    let _ = reload_gameplay_config();
    send_text(session, &format!("Battle Config Reloaded!")).await?;
    Ok(())
}