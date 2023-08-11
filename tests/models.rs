use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng, Rng,
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
pub struct TestBook {
    pub id: Option<i32>,
    pub name: String,
    pub price: Option<i32>,
    pub in_stock: Option<bool>,
    pub authors: Option<Vec<TestAuthor>>,
}

impl TestBook {
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let name = Alphanumeric.sample_string(&mut rng, 8);
        let price = rng.gen_range(0..100);
        let in_stock = rng.gen_bool(0.5);

        Self {
            id: None,
            name,
            price: Some(price),
            in_stock: Some(in_stock),
            authors: None,
        }
    }
}
