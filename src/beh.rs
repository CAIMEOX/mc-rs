use crate::pack::Manifest;
use std::fs;
use identicon_rs::Identicon;
use std::io::Write;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub struct BehaviorPack {
    manifest: Manifest,
    animations: Animations,
    entities: HashMap<String, Entity>,
}

impl BehaviorPack {
    pub fn new(name: String, description: String) -> Self{
        Self {
            manifest: Manifest::new(
                name,description,"data".to_string()
            ),
            animations: Animations {
                format_version: "1.10.0".to_string(),
                animations: HashMap::new()
            },
            entities: HashMap::new()
        }
    }

    pub fn init(&self){
        let name = &self.manifest.name();
        fs::create_dir_all(format!("{}/behavior_pack/functions/", name));
        let pack_icon = Identicon::new_default(
            &name,
        );
        pack_icon.save_image(&format!("{}/behavior_pack/pack_icon.png", name));
        if let Ok(mut res) = fs::File::create(&format!("{}/behavior_pack/manifest.json", name)) {
            res.write_all(self.manifest.get_json().unwrap().as_ref());
        } else {
            panic!("Unable to create Manifest file!");
        }
    }

    pub fn add_animation(&mut self, name: &str, timeline: Vec<String>, l: bool, len: u32){
        let mut tl = HashMap::new();
        let mut index:f32 = 0.0;
        for event in timeline {
            tl.insert(index.to_string(), event);
            index += 1.0;
        }
        self.animations.animations.insert(name.to_string(), Animation {
            timeline: tl,
            animation_length: {if len > 0 {len} else {index as u32}},
            r#loop: l
        });
    }

    pub fn add_entity(&mut self, identifier: &str, is_spawnable: bool, is_summonable: bool, is_experimental: bool) {
        self.entities.insert(identifier.to_string(),Entity {
            description: Description {
                identifier: identifier.to_string(),
                is_spawnable,
                is_summonable,
                is_experimental,
                script: Script { animate: vec![] },
                animations: HashMap::new()
            }
        });
    }

    pub fn add_script(&mut self, name: &str, anime_name: &str, anime_path: &str) {
        if let Some(entity) = self.entities.get_mut(name) {
            entity.add_animation(anime_name, anime_path);
            entity.add_script(anime_name);
        }
    }

    pub fn exit(self){
        let name = &self.manifest.name();
        if !self.animations.animations.is_empty() {
            fs::create_dir(format!("{}/behavior_pack/animations", name));
            if let Ok(mut anime) = fs::File::create(&format!("{}/behavior_pack/animations/mc-rs.json", name)) {
                anime.write_all(self.animations.get_json().unwrap().as_ref());
            } else {
                panic!("Unable to create Animation file!");
            }
        }
        if !self.entities.is_empty() {
            fs::create_dir(format!("{}/behavior_pack/entity", name));
            for entity in self.entities {
                let entity_value = serde_json::json!({
                    "format_version": "1.10.0",
                    "minecraft:entity": entity.1
                });
                if let Ok(mut entity) = fs::File::create(&format!("{}/behavior_pack/entity/{}.json", name, entity.0)) {
                    entity.write_all(serde_json::to_string(&entity_value).unwrap().as_ref());
                } else {
                    panic!("Unable to create Entity file!");
                }
            }
        }
    }

}

#[derive(Serialize, Deserialize)]
struct Animations {
    format_version: String,
    animations: HashMap<String, Animation>
}

#[derive(Serialize, Deserialize)]
struct Animation {
    timeline: HashMap<String, String>,
    animation_length: u32,
    r#loop:bool
}

impl Animations {
    fn get_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}

#[derive(Serialize, Deserialize)]
struct Entity {
    description: Description
}

impl Entity {
    pub fn add_animation(&mut self, index: &str, animation_path: &str) {
        self.description.animations.insert(index.to_string(), animation_path.to_string());
    }

    pub fn add_script(&mut self, script: &str) {
        self.description.script.animate.push(script.to_string());
    }
}

#[derive(Serialize, Deserialize)]
struct Description {
    identifier: String,
    is_spawnable: bool,
    is_summonable: bool,
    is_experimental: bool,
    script: Script,
    animations: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
struct Script {
    animate: Vec<String>
}
