use crate::pack::Manifest;
use std::fs;
use identicon_rs::Identicon;
use std::io::Write;

pub struct BehaviorPack {
    manifest: Manifest,
}

impl BehaviorPack {
    pub fn new(name: String, description: String) -> Self{
        Self {
            manifest: Manifest::new(
                name,description,"data".to_string()
            ),
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


}