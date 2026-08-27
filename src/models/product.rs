use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
pub id: i32,
pub name: String,
pub price: f64,
pub stock: i32,
pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProduct {
pub name: String,
pub price: f64,
pub stock: i32,
pub category: String,
}

impl Product {
pub fn validate(&self) -> Result<(), String> {
if self.price < 0.0 {
return Err("El precio no puede ser negativo".to_string());
}
Ok(())
}
}