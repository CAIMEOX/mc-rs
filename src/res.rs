use crate::pack::Manifest;
use std::path::Path;
use std::fs;
use identicon_rs::Identicon;
use std::io::Write;

pub struct ResourcePack {
    manifest: Manifest,
    path: &'static Path
}

impl ResourcePack {
    pub fn new(name: String, description: String, path: &'static Path) -> Self{
        Self {
            manifest: Manifest::new(
                name,description,"resource".to_string()
            ),
            path
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

    pub fn add_particle(&self, file_name: String){
        let name = &self.manifest.name();
        fs::File::create(&format!("{}/resource_pack/particles/{}", name, file_name)).map_err(|e| {
            panic!("Unable to add particle: {}", e);
        });
    }
}