use actix_web::{ web, HttpResponse, Responder, Error };
use serde::Serialize;
use crate::{ models::Image, services::AppState };
use actix_multipart::Multipart;
use futures::StreamExt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize)]
struct PaginatedResponse {
  images: Vec<Image>,
  total_count: usize,
  page: usize,
  per_page: usize,
}

pub async fn get_images(
  state: web::Data<AppState>,
  path: web::Path<(usize, usize)> // (page, per_page)
) -> impl Responder {
  let (page, per_page) = path.into_inner();

  if page < 1 || per_page < 1 {
    return HttpResponse::BadRequest().body("Invalid page or per_page value");
  }

  // Lock the shared state to access the images
  let images = state.images.lock().await;
  let total_count = images.len();

  if (page - 1) * per_page >= total_count {
    return HttpResponse::NotFound().body(
      "No images found for the requested page"
    );
  }

  // Calculate the start and end for pagination
  let start = (page - 1) * per_page;
  let end = std::cmp::min(start + per_page, total_count);

  // Slice the images for the requested page
  let paginated_images = images[start..end].to_vec();

  HttpResponse::Ok().json(PaginatedResponse {
    images: paginated_images,
    total_count,
    page,
    per_page,
  })
}

// Handle file uploads
pub async fn upload_image(
  mut payload: Multipart
) -> Result<HttpResponse, Error> {
  // Process each field in the multipart request
  while let Some(item) = payload.next().await {
    let mut field = item?;

    // Check if the field is a file (should be the image)
    let filename = field
      .content_disposition()
      .get_filename()
      .unwrap_or("image.png");
    let file_ext = filename.split('.').last().unwrap_or("png"); // Default to png if no extension

    // Create a unique filename for the uploaded image
    let unique_filename = format!("{:?}.{}", Uuid::new_v4(), file_ext);

    // Define the file path where the image will be saved
    let filepath = Path::new("./uploads").join(unique_filename.clone());

    // Create the file on the server
    let mut f = File::create(filepath)?;

    // Save the file content to the server
    while let Some(chunk) = field.next().await {
      let data = chunk?;
      f.write_all(&data)?;
    }

    return Ok(
      HttpResponse::Ok().json(
        format!("File uploaded as {}", unique_filename.clone())
      )
    );
  }

  // If no file is uploaded, return an error response
  Err(actix_web::error::ErrorBadRequest("No file uploaded"))
}

pub async fn get_image_by_id(
  image_id: web::Path<String>
) -> Result<actix_files::NamedFile, actix_web::Error> {
  let image_id = image_id.into_inner();
  let filename = format!("uploads/{}", image_id);
  let path = Path::new(&filename);

  // Check if the file exists and serve it
  if path.exists() {
    Ok(actix_files::NamedFile::open(path)?)
  } else {
    Err(actix_web::error::ErrorNotFound("File not found"))
  }
}
