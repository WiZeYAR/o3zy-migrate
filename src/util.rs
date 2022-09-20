use std::{
    convert::AsRef,
    ffi::OsStr,
    fs::File,
    io::Write,
    path::Path,
    process::{Command, Output},
};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    CMD(Output),
    Other(Box<dyn std::error::Error>),
    Abort,
}

pub fn run_cmd_as(
    cmd: impl AsRef<str>,
    user: impl AsRef<str>,
    dir: impl AsRef<str>,
) -> Result<String, Error> {
    log::debug!(
        "Running command `{}` at `{}` on behalf of `{}`",
        cmd.as_ref(),
        user.as_ref(),
        dir.as_ref()
    );
    let res = match Command::new("su")
        .arg("-c")
        .arg(cmd.as_ref())
        .arg(user.as_ref())
        .current_dir(dir.as_ref())
        .output()
        .map_err(Error::IO)?
    {
        Output { status, stdout, .. } if status.success() => {
            log::debug!("Finished running command `{}`", cmd.as_ref());
            Ok(String::from_utf8_lossy(&stdout).to_string())
        }
        output => {
            log::error!("Error running command `{}`", cmd.as_ref());
            Err(Error::CMD(output))
        }
    };
    log::trace!("{:#?}", res);
    res
}

#[deprecated]
pub fn run_cmd(cmd: &mut Command) -> Result<String, Error> {
    let command_string = Some(cmd.get_program())
        .into_iter()
        .chain(cmd.get_args())
        .map(|x| x.to_string_lossy())
        .map(|x| [x, " ".into()])
        .flatten()
        .collect::<String>();
    log::debug!("Running command:\t`{}`", command_string);
    let out = cmd
        .output()
        .map_err(Error::IO)
        .and_then(|out| {
            if out.status.success() {
                Ok(String::from_utf8_lossy(&out.stdout).to_string())
            } else {
                Err(Error::CMD(out))
            }
        })
        .or_else(|err| {
            log::error!("{:#?}", err);
            Err(err)
        });
    log::debug!("Finished running command:\t`{}`", command_string);
    log::trace!("{:#?}", out);
    out
}

pub fn load_file(path: &str, src: &[u8]) -> Result<(), Error> {
    File::create(path)
        .and_then(|mut file| file.write(src))
        .map_err(Error::IO)
        .map(|_| ())
}
