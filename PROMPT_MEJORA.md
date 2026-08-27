# Prompt para Mejorar el Codigo Base

Copia y pega el siguiente contenido completo en un asistente de IA (Claude, ChatGPT, etc.)
para obtener un ZIP con el proyecto corregido y listo para compilar.

---

```
Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Exportación en ZIP
Empaqueta todos los archivos en un único archivo ZIP descargable respetando exactamente
la estructura de rutas indicada por los marcadores.
El ZIP debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: El ZIP debe estar listo para descargar al finalizar. No preguntes si el usuario
quiere generarlo. Simplemente genera el archivo y proporciona el enlace de descarga; No debes desplegar en el chat el resumen de lo que arreglaste al Zip, solo entregalo.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación o descripciones sin código, genera los archivos
correspondientes sin aplicar análisis de compilación
No agregues texto después del enlace de descarga del ZIP
No preguntes si el usuario quiere el ZIP: simplemente generalo siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:
// === ARCHIVO: Cargo.toml ===
[package]
name = "inventory_api"
version = "0.1.0"
edition = "2018"

[dependencies]
actix-web = "4.0"
diesel = { version = "2.0", features = ["sqlite"] }
dotenv = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"


// === ARCHIVO: src/main.rs ===
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod models;
mod routes;
mod handlers;
mod db;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
dotenv().ok();
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
db::establish_connection(&database_url);

HttpServer::new(|| {
App::new()
.wrap(middleware::error_handler())
.service(routes::product_routes())
})
.bind("127.0.0.1:8080")?
.run()
.await
}


// === ARCHIVO: src/models/product.rs ===
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


// === ARCHIVO: src/routes/product_routes.rs ===
use actix_web::{web, HttpResponse, Responder};
use crate::handlers::product_handler::*;

pub fn product_routes(cfg: &mut web::ServiceConfig) {
cfg.service(web::resource("/products").route(web::post().to(create_product)))
.service(web::resource("/products/{id}").route(web::get().to(get_product))
.route(web::put().to(update_product))
.route(web::delete().to(delete_product)));
}


// === ARCHIVO: src/handlers/product_handler.rs ===
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


// === ARCHIVO: src/db/database.rs ===
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn establish_connection(database_url: &str) -> SqliteConnection {
SqliteConnection::establish(database_url)
.expect(&format!("Error al conectar a la base de datos en {}", database_url))
}


// === ARCHIVO: src/schema.rs ===
table! {
products {
id -> Integer,
name -> Varchar,
price -> Float,
stock -> Integer,
category -> Varchar,
}
}


// === ARCHIVO: src/middleware/error_handler.rs ===
use actix_web::{dev::Payload, Error, HttpResponse, Result};

pub fn error_handler() -> impl actix_web::middleware::Middleware {
actix_web::middleware::errhandlers::MiddlewareFactory::new(|err: &Error| HttpResponse::InternalServerError().body(format!("{:?}", err)))
}


// === ARCHIVO: tests/product_tests.rs ===
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

```
