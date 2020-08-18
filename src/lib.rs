mod beh;
mod pack;
mod res;

#[cfg(test)]
mod tests {
    use crate::pack::{Particle,Manifest};

    #[test]
    fn manifest() {
        let manifest = Manifest::new(
            "test".to_string(),
            "Powered by CAIMEO".to_string(),
            "data".to_string(),
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
}
