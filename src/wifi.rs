use crate::wifi_parse::*;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

#[derive(Default)]
struct UIState {
    wifi_vec: Vec<WiFi>,
}

pub fn wifi_setup() -> Result<(), Box<dyn std::error::Error>> {
    use dialoguer::Confirm;

    let items = vec!["Item 1", "item 2"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => println!("User selected item : {}", items[index]),
        None => println!("User did not select anything"),
    }

    Ok(())
}
