extern crate serde_json;
use self::serde_json::Value;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use uuid::Uuid;

//Manifest
#[derive(Serialize, Deserialize)]
pub struct Header {
    description: String,
    name: String,
    uuid: String,
    version: [u8; 3],
}

#[derive(Serialize, Deserialize)]
struct Module {
    description: String,
    r#type: String,
    uuid: String,
    version: [u8; 3],
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    format_version: u8,
    header: Header,
    modules: [Module; 1],
}

impl Manifest {
    pub fn new(name: String, description: String, t: String) -> Self {
        Manifest {
            format_version: 0,
            header: Header {
                description: description.clone(),
                name,
                uuid: Uuid::new_v4().to_hyphenated().to_string(),
                version: [1, 0, 0],
            },
            modules: [Module {
                description,
                uuid: Uuid::new_v4().to_hyphenated().to_string(),
                version: [1, 0, 0],
                r#type: t,
            }],
        }
    }

    pub fn get_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    pub fn name(&self) -> String {
        self.header.name.clone()
    }

}
