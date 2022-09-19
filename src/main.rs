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
use std::process::Command;
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
        // ---- DEPLOYING SETUP SH
        .and_then(|_| util::load_file(PATH_SH_SETUP, include_bytes!("../assets/update.sh")))
        //
        // ---- DEPLOYING UPDATE SH
        .and_then(|_| util::load_file(PATH_SH_UPDATE, include_bytes!("../assets/update.sh")))
        //
        // ---- CHECKING OUT REPO
        .and_then(|_| {
            util::run_cmd(
                Command::new("su")
                    .arg("-c")
                    .arg(format!(
                        "git clone --branch {} {} {}",
                        GIT_BRANCH, GIT_REPO_URL, GIT_REPO_PATH
                    ))
                    .arg(""),
            )
        })
        //
        // ---- UPGRADING SYSTEM AND INSTALLING DEPENDENCIES
        .and_then(|_| {
            run_cmd(
                Command::new("sudo")
                    .arg("bash")
                    .arg("-c")
                    .arg("curl -fsSL https://deb.nodesource.com/setup_14.x | bash -"),
            )?;
            run_cmd(
                Command::new("sudo")
                    .arg("apt-get")
                    .arg("update")
                    .env("DEBIAN_FRONTEND", "noninteractive"),
            )?;
            run_cmd(
                Command::new("sudo")
                    .arg("apt-get")
                    .arg("upgrade")
                    .arg("-y")
                    .env("DEBIAN_FRONTEND", "noninteractive"),
            )?;
            run_cmd(
                Command::new("sudo")
                    .arg("apt-get")
                    .arg("install")
                    .arg("-y")
                    .arg("nodejs")
                    .env("DEBIAN_FRONTEND", "noninteractive"),
            )?;
            run_cmd(
                Command::new("su")
                    .arg("-c")
                    .arg("npm install --prod")
                    .arg("pi")
                    .current_dir(GIT_REPO_PATH),
            )
        })
        //
        // ---- SETTING UP PM
        .and_then(|_| run_cmd(Command::new("npm").arg("i").arg("-g").arg("pm2")))
        .and_then(|_| run_cmd(Command::new("pm2").arg("start").arg("server.js")))
        .and_then(|_| run_cmd(Command::new("pm2").arg("startup")))
        .and_then(|_| run_cmd(Command::new("pm2").arg("save")))
        //
        // ---- STOPPING NODE-RED
        .and_then(|_| {
            run_cmd(
                Command::new("sudo")
                    .arg("systemctl")
                    .arg("disable")
                    .arg("nodered.service"),
            )
        })
        //
        // ---- SETTING UP DEVICE.JSON
        .and_then(|_| todo!())
        // ---- REBOOTING
        .and_then(|()| run_cmd(Command::new("sudo").arg("shutdown").arg("-r")))
        .and(Ok(()))
}
