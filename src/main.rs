mod util;
mod wifi;
mod wifi_parse;

/* -------- CONSTANTS -------- */

pub const PATH_SH_SETUP: &str = "";
pub const PATH_SH_UPDATE: &str = "";
pub const GIT_REPO_PATH: &str = "";
pub const WLAN_DEVICE_NAME: &str = "wlan0";

/* -------- MAIN SECTION -------- */
fn main() -> Result<(), util::Error> {
    simple_logger::init().unwrap();

    // Setting up WLAN, if required
    wifi::wifi_setup()?;

    // Terminating
    Ok(())
}
