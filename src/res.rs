use crate::pack::Manifest;
extern crate serde_json;
use identicon_rs::Identicon;
use self::serde_json::Value;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::io::Write;

pub struct ResourcePack {
    manifest: Manifest,
}

impl ResourcePack {
    pub fn new(name: String, description: String) -> Self{
        Self {
            manifest: Manifest::new(
                name,description,"resource".to_string()
            ),
        }
    }

    pub fn init(&self){
        let name = &self.manifest.name();
        fs::create_dir_all(format!("{}/resource_pack/textures/frames", name));
        fs::create_dir_all(format!("{}/resource_pack/particles", name));
        let pack_icon = Identicon::new_default(
            &name,
        );
        pack_icon.save_image(&format!("{}/resource_pack/pack_icon.png", name));
        if let Ok(mut res) = fs::File::create(&format!("{}/resource_pack/manifest.json", name)) {
            res.write_all(self.manifest.get_json().unwrap().as_ref());
        } else {
            panic!("Unable to create Manifest file!");
        }
    }

    pub fn add_particle(&self, identifier: &str, material: &str, texture: &str, v:Value){
        let name = &self.manifest.name();
        let mut data = fs::File::create(&format!("{}/resource_pack/particles/{}.json", name, identifier)).expect("Unable to add particle");
        let particle = Particle::new(identifier.to_string(), material.to_string(), format!("{}/resource_pack/textures/frames/{}", name, texture), v);
        data.write_all(&particle.get_json().unwrap().as_ref());
    }
}


//Particle
#[derive(Serialize, Deserialize)]
pub struct Particle {
    format_version: String,
    particle_effect: ParticleDescription,
    components: serde_json::value::Value,
}

#[derive(Serialize, Deserialize)]
struct ParticleDescription {
    identifier: String,
    basic_render_parameters: BasicRenderParameters,
}

#[derive(Serialize, Deserialize)]
struct BasicRenderParameters {
    material: String,
    texture: String, // Path
}

impl Particle {
    pub fn new(identifier: String, material: String, texture: String, components: Value) -> Self {
        Self {
            format_version: "1.10.0".to_string(),
            particle_effect: ParticleDescription {
                identifier,
                basic_render_parameters: BasicRenderParameters { material, texture },
            },
            components,
        }
    }

    pub fn get_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}

