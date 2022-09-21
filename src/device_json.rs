use crate::util::Error;
use crate::{GIT_REPO_PATH, NODE_RED_CONFIG_PATH};
use serde_derive::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
struct DeviceInfo {
    #[serde(rename = "gnr_code")]
    code: String,

    #[serde(rename = "gnr_model")]
    model: String,
}

pub fn setup() -> Result<(), Error> {
    let DeviceInfo { model, code } =
        serde_json::from_str(&std::fs::read_to_string(NODE_RED_CONFIG_PATH).map_err(Error::IO)?)
            .map_err(|e| Error::Other(Box::new(e)))?;
    let mut device_json: Value = serde_json::from_str(match model.as_str() {
        "o3zy20g" => include_str!("../assets/device_json/o3zy20g.json"),
        "o3zy40g+uv" => include_str!("../assets/device_json/o3zy20g.json"),
        "o3zy40g" => include_str!("../assets/device_json/o3zy20g.json"),
        "o3zy60g+uv" => include_str!("../assets/device_json/o3zy20g.json"),
        "o3zy60g" => include_str!("../assets/device_json/o3zy20g.json"),
        "o3zycar" => include_str!("../assets/device_json/o3zy20g.json"),
        _ => unimplemented!("The model, specified in the NodeRed configuration is invalid"),
    })
    .map_err(|e| Error::Other(Box::new(e)))?;
    device_json
        .as_object_mut()
        .and_then(|map| map.get_mut("Data"))
        .and_then(|val| val.as_object_mut())
        .and_then(|map| map.get_mut("Code"))
        .map(|val| *val = json!(code))
        .expect("An included device.json template has a wrong format (no Device.Code)");
    std::fs::write(GIT_REPO_PATH, serde_json::to_string(&device_json).unwrap())
        .map_err(Error::IO)?;
    Ok(())
}
