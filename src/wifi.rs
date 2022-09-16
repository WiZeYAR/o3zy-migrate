use crate::util::{run_cmd, Error};
use crate::wifi_parse::*;
use dialoguer::console::Term;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use itertools::Itertools;
use log::*;
use std::process::Command;
use std::time::Duration;

pub fn wifi_setup() -> Result<(), Error> {
    info!("Looking for possible WiFi spots...");
    let spots = loop {
        let spots = WiFi::scan()?;
        if !spots.is_empty() {
            break spots;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    .into_iter()
    .filter(|WiFi { age, .. }| *age < 5000)
    .sorted_by(|WiFi { signal: a, .. }, WiFi { signal: b, .. }| Ord::cmp(b, a))
    .unique_by(|WiFi { ssid, .. }| ssid.clone())
    .take(10)
    .collect_vec();
    let spot_strings = spots
        .iter()
        .map(|WiFi { ssid, signal, .. }| format!("[Signal {}%]\tSSID: '{}'", signal, ssid))
        .collect_vec();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&spot_strings)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .map_err(|e| Error::Other(Box::new(e)))?;
    let WiFi { ssid, .. } = match selection {
        Some(index) => spots.into_iter().skip(index).next(),
        None => None,
    }
    .ok_or(Error::Abort)?;
    let password = Input::<String>::new()
        .with_prompt(format!("Enter password for '{}'", ssid))
        .interact_text()
        .map_err(|e| Error::Other(Box::new(e)))?;
    run_cmd(Command::new("sudo").arg("mkdir").arg("-p").arg("/etc/wpa_supplicant"))?;
    run_cmd(Command::new("sudo").arg("bash").arg(
     format!(
        "printf \'{}\' >/etc/wpa_supplicant/wpa_supplicant.conf",
        format!(
            r#"country=IT\nctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev\nupdate_config=1\mnetwork={{\nssid="{}"\npsk="{}"\n}}"#,
            ssid, password
        ),
    )
    // println!("{}", s);
    ))?;
    Ok(())
}
