use actix_web::{ web, App, HttpServer, middleware };
use actix_cors::Cors; // Import Cors

mod models;
mod handlers;
mod services;

use handlers::get_images;
use services::AppState;
use handlers::{ get_image_by_id, upload_image };

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create app state with images (You might later connect to a database)
  let app_state = AppState::new();

  HttpServer::new(move || {
    let cors = Cors::default()
      .allow_any_origin() // Allow requests from any origin
      .allow_any_method() // Allow any HTTP method
      .allow_any_header(); // Allow any header

    App::new()
      .wrap(cors) // Apply CORS middleware
      .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Trim)) // Normalize paths
      .app_data(web::Data::new(app_state.clone())) // Share state
      .route("/images/{page}/{per_page}", web::get().to(get_images)) // Route for fetching paginated images
      .route("/image/{id}", web::get().to(get_image_by_id)) // Route for fetching a single image by ID
      .route("/upload", web::post().to(upload_image)) // Route for uploading an image
  })
    .bind("127.0.0.1:8080")?
    .run().await
    .map_err(|e| e.into())
}
