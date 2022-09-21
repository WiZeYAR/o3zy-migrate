/* -------- CONSTANTS -------- */

pub const PATH_SH_SETUP: &str = "/home/o3zy/Desktop/setup.sh";
pub const PATH_SH_UPDATE: &str = "/home/o3zy/Desktop/update.sh";
pub const GIT_BRANCH: &str = "RELEASE";
pub const GIT_REPO_PATH: &str = "/home/pi/Desktop/device-backend";
pub const GIT_REPO_URL: &str = formatcp!(
    "https://{}:x-oauth-basic@github.com/o3zy/device-backend.git",
    str_replace!(include_str!("../assets/github-token"), '\n', ""),
);
pub const WLAN_DEVICE_NAME: &str = "wlan0";

/* -------- IMPORTS -------- */

use const_format::{formatcp, str_replace};
use util::*;

/* -------- MODULES -------- */

mod util;
mod wifi;
mod wifi_parse;

/* -------- MAIN SECTION -------- */
fn main() -> Result<(), Error> {
    simple_logger::init().unwrap();
    Ok(())
        //
        // ---- SETTING THE INTERNET UP
        .and_then(|_| wifi::setup())
        //
        // ---- UPGRADING SYSTEM AND INSTALLING NODE
        .and_then(|_| {
            run_cmd_many(
                [
                    "curl -sL https://deb.nodesource.com/setup_14.x | sudo bash -",
                    "DEBIAN_FRONTEND=noninteractive apt-get update",
                    "DEBIAN_FRONTEND=noninteractive apt-get -y upgrade",
                    "DEBIAN_FRONTEND=noninteractive apt-get -y install nodejs",
                    "systemctl disable nodered.service",
                ],
                "root",
                GIT_REPO_PATH,
            )
        })
        //
        // ---- DEPLOYING NODE PROJECT
        .and_then(|_| {
            run_cmd_many(
                [
                    formatcp!(
                        "git clone --branch {} {} {}",
                        GIT_BRANCH,
                        GIT_REPO_URL,
                        GIT_REPO_PATH
                    ),
                    "chmod -R u+x scripts/",
                    "npm i --prod",
                ],
                "pi",
                GIT_REPO_PATH,
            )
        })
        //
        // ---- SETTING UP PM2
        .and_then(|_| {
            run_cmd_many(
                [
                    "npm i -g pm2",
                    "pm2 start server.js",
                    "pm2 startup",
                    "pm2 save",
                ],
                "root",
                GIT_REPO_PATH,
            )
        })
        //
        // ---- SETTING UP DEVICE.JSON
        .and_then(|_| Ok(()))
        //
        // ---- REBOOTING
        .and_then(|_| run_cmd_as("shutdown -r", "root", "/"))
        .and(Ok(()))
}
