use actix_web::{App, HttpServer, web}; // Import necessary components
mod models;  // Declare the models module
mod handlers;
mod routes;
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_pool = db::get_db_pool().await;


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}