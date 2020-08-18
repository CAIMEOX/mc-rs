use crate::res::ResourcePack;
use crate::beh::BehaviorPack;
use uuid::adapter::compact::deserialize;

mod beh;
mod pack;
mod res;

struct McPack {
    resource: ResourcePack,
    behavior: BehaviorPack,
}

impl McPack {
    fn new(name: &str, description: &str) -> Self{
        Self {
            resource: ResourcePack::new(name.to_string(),description.to_string()),
            behavior: BehaviorPack::new(name.to_string(),description.to_string()),
        }
    }

    fn init_all(self){
        self.resource.init();
        self.behavior.init();
    }


}

#[cfg(test)]
mod tests {
    use crate::pack::{Manifest};
    use crate::res::Particle;
    #[test]
    fn manifest() {
        let manifest = Manifest::new(
            "test".to_string(),
            "Powered by CAIMEO".to_string(),
            "".to_string(),
        );
        println!("Manifest: {}", manifest.get_json().unwrap());
    }

    #[test]
    fn particle() {
        let p = Particle::new(
            "caimeo".to_string(),
            "particles_alpha".to_string(),
            "r".to_string(),
            serde_json::json!({
                    "minecraft:emitter_rate_instant": {
                        "num_particles": 1
                    },
                    "minecraft:emitter_lifetime_once": {
                        "active_time": 0
                    },
                }),
        );
        println!("Particle: {}", p.get_json().unwrap());
    }

    use crate::res::ResourcePack;
    use crate::beh::BehaviorPack;
    use crate::McPack;

    #[test]
    fn new_pack() {
        let w = McPack::new("test", "DEV");
        w.init_all();
    }
}
