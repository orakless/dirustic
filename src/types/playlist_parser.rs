use tokio::process::Command;
use crate::dirustic_error::Error;
use crate::dirustic_error::Error::YtDlpError;

const YOUTUBE_DL_COMMAND: &str = "yt-dlp";
const YOUTUBE_DL_PLAYLIST_ARGS: &str = "-s --flat-playlist --print url";

pub fn is_playlist(url: &str) -> bool {
    url.contains("list=")
}
pub async fn get_items_from_playlist(url: &str) -> Result<Vec<String>, Error> {
    let output = Command::new(YOUTUBE_DL_COMMAND)
        .args(YOUTUBE_DL_PLAYLIST_ARGS.split(" "))
        .arg(url)
        .output().await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::YtDlpNotFound
            } else {
                Error::UnknownError { e: e.to_string() }
            }
        })?;

    if !output.status.success() {
        return Err(YtDlpError { e: String::from_utf8_lossy(&output.stderr).to_string() })?;
    }

    let mut items = String::from_utf8(output.stdout)?
        .split("\n")
        .collect::<Vec<&str>>()
        .iter().map(|s| s.to_string())
        .collect::<Vec<String>>();

    items.pop();

    Ok(items)
}
