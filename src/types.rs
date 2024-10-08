use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipient {
    pub address: String,
    pub display_name: String,
}

impl Recipient {
    pub fn new(address: &str, display_name: Option<&str>) -> Self {
        Recipient {
            address: address.to_string(),
            display_name: display_name.unwrap_or(address).to_string(),
        }
    }
}
