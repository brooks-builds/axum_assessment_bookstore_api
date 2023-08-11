use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestAuthor {
    pub id: Option<i32>,
    pub name: String,
    pub books: Option<Vec<TestBook>>,
}

impl TestAuthor {
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let name = Alphanumeric.sample_string(&mut rng, 8);

        Self {
            id: None,
            name,
            books: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestBook {}
