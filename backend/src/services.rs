use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::Image;

#[derive(Clone)]
pub struct AppState {
  pub images: Arc<Mutex<Vec<Image>>>,
}

impl AppState {
  pub fn new() -> Self {
    let images = vec![
      Image::new(
        "1",
        "http://127.0.0.1:8080/image/czarodziejka-jasność-20+kontrast-30.jpg",
        "A beautiful sunset"
      ),
      Image::new(
        "2",
        "http://127.0.0.1:8080/image/Spooky_cat.png",
        "A mountain view"
      ),
      Image::new(
        "3",
        "http://127.0.0.1:8080/image/pianka.png",
        "A serene beach"
      )
    ];

    AppState {
      images: Arc::new(Mutex::new(images)),
    }
  }
}
