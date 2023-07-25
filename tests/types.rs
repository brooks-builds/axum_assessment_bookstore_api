use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAuthor {
    name: String,
}

impl CreateAuthor {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let name = Alphanumeric.sample_string(&mut rng, 8);

        Self { name }
    }
}
