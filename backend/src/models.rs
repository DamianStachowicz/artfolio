use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Image {
  pub id: String,
  pub url: String,
  pub description: String,
}

impl Image {
  pub fn new(id: &str, url: &str, description: &str) -> Self {
    Image {
      id: id.to_string(),
      url: url.to_string(),
      description: description.to_string(),
    }
  }
}
