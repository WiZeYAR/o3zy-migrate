mod util;
mod wifi_parse;

use std::process::Command;
use util::*;
use wifi_parse::*;

/* -------- CONSTANTS -------- */

const PATH_SH_SETUP: &str = "";
const PATH_SH_UPDATE: &str = "";
const GIT_REPO_PATH: &str = "";
const WLAN_DEVICE_NAME: &str = "wlp174s0";

/* -------- MAIN SECTION -------- */
fn main() {
    println!("{:#?}", WiFi::scan().unwrap())
}
// fn main() -> Result<(), Error> {
//     online::sync::check(Some(3)).unwrap();
//     Ok(())
//         // ---- DEPLOYING SETUP SH
//         .and_then(|()| load_file(PATH_SH_SETUP, include_bytes!("../assets/update.sh")))
//         // ---- DEPLOYING UPDATE SH
//         .and_then(|()| load_file(PATH_SH_UPDATE, include_bytes!("../assets/update.sh")))
//         // ---- CHECKING OUT REPO
//         .and_then(|()| run_cmd(Command::new("git").arg("clone").arg(GIT_REPO_PATH)))
//         // ---- INSTALLING NPM DEPENDENCIES
//         .and_then(|()| run_cmd(Command::new("npm").arg("install").arg("--prod")))
//         // ---- SETTING UP PM2
//         .and_then(|()| todo!())
//         // ---- STOPPING NODE-RED
//         .and_then(|()| todo!())
//         // ---- SETTING UP DEVICE.JSON
//         .and_then(|()| todo!())
// }
