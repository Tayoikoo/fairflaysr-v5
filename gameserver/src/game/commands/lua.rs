use super::*;
use std::fs::File;
use std::io::Read;

pub async fn windy_command(args: &[&str], session: &PlayerSession) -> Result<()> {
    if args.is_empty() {
        return send_text(session, "Usage: /lua [lua file]").await;
    }

    let file_name = args[0];

    // Construct the file path
    let file_path = format!("lua/{}.lua", file_name);

    // Attempt to open the file
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return send_text(session, &format!("Failed to open file: {}", file_path)).await,
    };

    // Read the file contents into a vector
    let mut file_contents = Vec::new();
    if let Err(_) = file.read_to_end(&mut file_contents) {
        return send_text(session, &format!("Failed to read file: {}", file_path)).await;
    }

    // Create the download data structure
    let windseed = ClientDownloadDataScNotify {
        download_data: Some(ClientDownloadData {
            version: 51,
            time: util::cur_timestamp_ms() as i64,
            data: file_contents,
        }),
    };

    // Send the download data to the client
    session.send(CMD_CLIENT_DOWNLOAD_DATA_SC_NOTIFY, windseed).await?;

    // Notify the user that the command was successful
    send_text(session, &format!("Client Executed from file {file_name}.lua")).await
}
