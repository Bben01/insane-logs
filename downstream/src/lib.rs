use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Stat {
    cpu: f32,
    mem: f32,
    disk: f32,
    requests: usize,
}

pub fn down() {
    let stat = Stat {
        cpu: 0.0,
        mem: 0.0,
        disk: 0.0,
        requests: 0,
    };

    log::stat!(stat);
}
