use crate::utils::notify::notify;
use std::collections::HashMap;
/// This function is going to POST the readings to the KAA server.
///
/// #Arguments
/// function takes a &str as KAA server url.
///
/// #Return
/// Function is going to return a Response as a result.
static MAGNETOMETER_READING: &str = "Magnetometer_Readings";
pub async fn post(url: &str) -> reqwest::Response {
    let mut map = HashMap::new();
    let content = notify();
    map.insert(MAGNETOMETER_READING, content);
    let client = reqwest::Client::new();
    let response = match client.post(url).json(&map).send().await {
        Ok(response) => response,
        Err(error) => {
            panic!("Error here {}", error)
        }
    };

    response
}
