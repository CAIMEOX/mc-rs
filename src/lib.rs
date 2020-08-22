pub mod beh;
pub mod res;
pub mod pack;

#[cfg(test)]
mod tests {

    use crate::pack::McPack;
    use crate::pack::{Manifest};
    use crate::res::Particle;
    // #[test]
    fn manifest() {
        let manifest = Manifest::new(
            "test".to_string(),
            "Powered by CAIMEO".to_string(),
            "".to_string(),
        );
        println!("Manifest: {}", manifest.get_json().unwrap());
    }

    // #[test]
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
    use serde_json::json;

    #[test]
    fn new_pack() {
        let mut w = McPack::new("caimeo-test", "DEV");
        w.init_all();
        w.new_particle("id", "material", "./", json!(
        {
        "a":"b"
        }
        ));
        w.behavior.add_animation("anime.test",vec![
        "/say Ok".to_string()
        ], true, 1);
        w.behavior.add_entity("caimeo",true, true, true);
        w.behavior.add_script("caimeo", "test", "anime.test");
        w.behavior.exit();
    }
}
