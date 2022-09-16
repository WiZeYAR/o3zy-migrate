use crate::util::{run_cmd, Error};
use crate::{wifi_parse::*, WLAN_DEVICE_NAME};
use dialoguer::console::Term;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use itertools::Itertools;
use log::*;
use std::process::Command;
use std::time::Duration;

pub fn wifi_setup() -> Result<(), Error> {
    info!("Checking if the internet connection exists");
    if let Ok(()) = online::sync::check(None) {
        info!("Connection is already established");
        return Ok(());
    }

    info!("Looking for possible WiFi spots...");

    let spots = loop {
        let spots = WiFi::scan()?;
        if !spots.is_empty() {
            break spots;
        }
        std::thread::sleep(Duration::from_millis(250));
    };

    trace!("Network beacons found: {:?}", &spots);

    let spots = spots
        .into_iter()
        .filter(|WiFi { age, .. }| *age < 5000)
        .sorted_by(|WiFi { signal: a, .. }, WiFi { signal: b, .. }| Ord::cmp(b, a))
        .unique_by(|WiFi { ssid, .. }| ssid.clone())
        .take(10)
        .collect_vec();

    trace!("Processed network list: {:?}", &spots);

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
    run_cmd(
        Command::new("sudo")
            .arg("mkdir")
            .arg("-p")
            .arg("/etc/wpa_supplicant"),
    )?;
    let supplicant_file = format!(
        r#"
country=IT
ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
update_config=1
network={{
    ssid="{}"
    psk="{}"
}}"#,
        ssid, password
    );
    run_cmd(Command::new("bash").arg("-c").arg(format!(
        "echo '{}' | sudo tee /etc/wpa_supplicant/wpa_supplicant.conf",
        supplicant_file
    )))?;
    run_cmd(
        Command::new("sudo")
            .arg("wpa_cli")
            .arg("-i")
            .arg(WLAN_DEVICE_NAME)
            .arg("reconfigure"),
    )?;

    info!("Waiting for 10 seconds to set the network up...");
    std::thread::sleep(Duration::from_secs(10));
    info!("Testing the internet connection");
    online::sync::check(None).map_err(|x| Error::Other(Box::new(x)))?;
    info!("Internet connection successful");
    Ok(())
}
