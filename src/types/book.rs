use entity::books::Model;

pub struct Book {
    pub name: String,
    pub price: i32,
    pub in_stock: bool,
}

impl From<Model> for Book {
    fn from(value: Model) -> Self {
        Self {
            name: value.name,
            price: value.price,
            in_stock: value.in_stock,
        }
    }
}
