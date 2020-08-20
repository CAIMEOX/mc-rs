extern crate serde_json;
use self::serde_json::Value;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use uuid::Uuid;
use crate::res::ResourcePack;
use crate::beh::BehaviorPack;
//Manifest
#[derive(Serialize, Deserialize)]
struct Header {
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

pub struct McPack {
    resource: ResourcePack,
    behavior: BehaviorPack,
}

impl McPack {
    pub fn new(name: &str, description: &str) -> Self{
        Self {
            resource: ResourcePack::new(name.to_string(),description.to_string()),
            behavior: BehaviorPack::new(name.to_string(),description.to_string()),
        }
    }

    pub fn init_all(&self){
        self.resource.init();
        self.behavior.init();
    }

    pub fn new_particle(&self, identifier: &str, material: &str, texture: &str, v:Value){
        self.resource.add_particle(identifier, material, texture, v)
    }
}
