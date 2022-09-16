use std::{
    fs::File,
    io::Write,
    process::{Command, Output},
};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    CMD(Output),
    Other(Box<dyn std::error::Error>),
    Abort,
}

pub fn run_cmd(cmd: &mut Command) -> Result<String, Error> {
    cmd.output().map_err(Error::IO).and_then(|out| {
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            Err(Error::CMD(out))
        }
    })
}


pub fn load_file(path: &str, src: &[u8]) -> Result<(), Error> {
    File::create(path)
        .and_then(|mut file| file.write(src))
        .map_err(Error::IO)
        .map(|_| ())
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
//         // ---- SETTING UP PM
//         .and_then(|()| todo!())
//         // ---- STOPPING NODE-RED
//         .and_then(|()| todo!())
//         // ---- SETTING UP DEVICE.JSON
//         .and_then(|()| todo!())
// }
