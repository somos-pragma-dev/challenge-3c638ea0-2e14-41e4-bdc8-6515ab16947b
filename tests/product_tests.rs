use crate::models::Product;

#[test]
fn test_product_validation() {
let product = Product {
id: 1,
name: "Producto1".to_string(),
price: 10.0,
stock: 100,
category: "Electrónicos".to_string(),
};
assert!(product.validate().is_ok());
let invalid_product = Product {
id: 1,
name: "Producto1".to_string(),
price: -10.0,
stock: 100,
category: "Electrónicos".to_string(),
};
assert!(invalid_product.validate().is_err());
}