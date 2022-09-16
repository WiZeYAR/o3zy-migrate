mod util;
mod wifi;
mod wifi_parse;

/* -------- CONSTANTS -------- */

pub const PATH_SH_SETUP: &str = "";
pub const PATH_SH_UPDATE: &str = "";
pub const GIT_REPO_PATH: &str = "";
pub const WLAN_DEVICE_NAME: &str = "wlp174s0";

/* -------- MAIN SECTION -------- */
fn main() -> Result<(), util::Error> {
    wifi::wifi_setup()?;
    Ok(())
}
