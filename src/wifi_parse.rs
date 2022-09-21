use crate::{
    util::{run_cmd, Error, run_cmd_as},
    WLAN_DEVICE_NAME,
};
use const_format::formatcp;
use itertools::*;
use nom::{
    self, bytes::streaming::tag, character::complete::alphanumeric1, number::complete::float,
    sequence::delimited,
};
use std::{process::Command};
use std::rc::Rc;

#[derive(Debug)]
enum WiFiEntry {
    SSID(Rc<String>),
    Security(Rc<String>),
    Signal(u32),
    Age(u32),
}

type IResult<'a> = nom::IResult<&'a str, WiFiEntry>;

#[derive(Debug)]
pub struct WiFi {
    pub ssid: Rc<String>,
    pub security: Rc<String>,
    pub signal: u32,
    pub age: u32,
}

impl WiFi {
    pub fn scan() -> Result<Vec<WiFi>, Error> {
        let raw_data = run_cmd_as(formatcp!("iwlist {} scan",WLAN_DEVICE_NAME,) , "root", "/")?;
        let raw_entries: Vec<Vec<&str>> = {
            let mut entries = vec![];
            let mut entry = vec![];
            for line in raw_data.lines().skip(1).map(str::trim) {
                if line.contains("Cell") && line.contains("- Address: ") {
                    entries.push(entry);
                    entry = vec![];
                }
                entry.push(line)
            }
            entries
        };
        let entries = raw_entries
            .into_iter()
            .map(|entry| {
                entry
                    .into_iter()
                    .map(Self::parse)
                    .filter_map(Result::ok)
                    .map(|(_, w)| w)
                    .collect_vec()
            })
            .filter_map(|entry| match entry.as_slice() { 
                [ 
                    WiFiEntry::Signal(signal), 
                    WiFiEntry::SSID(ssid), 
                    WiFiEntry::Age(age), 
                    WiFiEntry::Security(security),
                ] => Some(WiFi{
                        ssid:ssid.clone(),
                        security:security.clone(),
                        signal:signal.clone(),
                        age:age.clone(),
                }),
                _ => None,
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    fn parse(input: &str) -> IResult {
        Self::parse_ssid(input)
            .or_else(|_| Self::parse_security(input))
            .or_else(|_| Self::parse_signal(input))
            .or_else(|_| Self::parse_age(input))
    }

    fn parse_ssid(input: &str) -> IResult {
        let (_, res) = delimited(tag("ESSID:\""), alphanumeric1, tag("\""))(input)?;
        Ok(("", WiFiEntry::SSID(Rc::new(res.to_owned()))))
    }

    fn parse_security(input: &str) -> IResult {
        let (input, _) = tag("IE: IEEE")(input)?;
        Ok(("", WiFiEntry::Security(Rc::new(format!("IEEE {}", input)))))
    }

    fn parse_signal(input: &str) -> IResult {
        let (input, _) = tag("Quality=")(input)?;
        let (input, div1) = float(input)?;
        let (input, _) = tag("/")(input)?;
        let (_, div2) = float(input)?;
        Ok(("", WiFiEntry::Signal((div1 / div2 * 100.0) as u32)))
    }

    fn parse_age(input: &str) -> IResult {
        let (_, age) = delimited(tag("Extra: Last beacon: "), float, tag("ms"))(input)?;
        Ok(("", WiFiEntry::Age(age as u32)))
    }
}
