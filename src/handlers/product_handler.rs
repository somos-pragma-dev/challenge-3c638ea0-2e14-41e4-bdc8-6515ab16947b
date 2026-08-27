use actix_web::{web, HttpResponse, Responder};
use crate::models::Product;
use crate::db::execute;

pub async fn create_product(new_product: web::Json<crate::models::NewProduct>) -> impl Responder {
let product = Product {
id: 0,
name: new_product.name.clone(),
price: new_product.price,
stock: new_product.stock,
category: new_product.category.clone(),
};
if let Err(e) = product.validate() {
return HttpResponse::BadRequest().body(e);
}
let conn = web::Data::<diesel::SqliteConnection>::get().get_ref().clone();
let result = diesel::insert_into(crate::schema::products::table)
.values(&product)
.execute(&conn);
match result {
Ok(_) => HttpResponse::Ok().body("Producto creado"),
Err(_) => HttpResponse::InternalServerError().body("Error al crear producto"),
}
}

pub async fn get_product(path: web::Path<(i32,)>) -> impl Responder {
let (id,) = path.into_inner();
let conn = web::Data::<diesel::SqliteConnection>::get().get_ref().clone();
let result = crate::schema::products::table
.filter(crate::schema::products::id.eq(id))
.first::<Product>(&conn);
match result {
Ok(product) => HttpResponse::Ok().json(product),
Err(_) => HttpResponse::NotFound().body("Producto no encontrado"),
}
}

pub async fn update_product(path: web::Path<(i32,)>, new_product: web::Json<crate::models::NewProduct>) -> impl Responder {
let (id,) = path.into_inner();
let conn = web::Data::<diesel::SqliteConnection>::get().get_ref().clone();
let result = crate::schema::products::table
.filter(crate::schema::products::id.eq(id))
.first::<Product>(&conn);
match result {
Ok(mut product) => {
product.name = new_product.name.clone();
product.price = new_product.price;
product.stock = new_product.stock;
product.category = new_product.category.clone();
if let Err(e) = product.validate() {
return HttpResponse::BadRequest().body(e);
}
let result = diesel::update(crate::schema::products::table)
.filter(crate::schema::products::id.eq(id))
.set(&product)
.execute(&conn);
match result {
Ok(_) => HttpResponse::Ok().body("Producto actualizado"),
Err(_) => HttpResponse::InternalServerError().body("Error al actualizar producto"),
}
},
Err(_) => HttpResponse::NotFound().body("Producto no encontrado"),
}
}

pub async fn delete_product(path: web::Path<(i32,)>) -> impl Responder {
let (id,) = path.into_inner();
let conn = web::Data::<diesel::SqliteConnection>::get().get_ref().clone();
let result = diesel::delete(crate::schema::products::table)
.filter(crate::schema::products::id.eq(id))
.execute(&conn);
match result {
Ok(_) => HttpResponse::Ok().body("Producto eliminado"),
Err(_) => HttpResponse::InternalServerError().body("Error al eliminar producto"),
}
}