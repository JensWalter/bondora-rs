#[derive(Debug, Clone)]
pub struct Configuration{
  pub base_path: String,
  pub token: String,
}

impl Default for Configuration {
  fn default() -> Configuration {
    Configuration {
      base_path: "https://api.bondora.com".to_string(),
      token: "".to_string(),
    }
  }
}

pub struct APIClient {
  pub configuration: Configuration,
}

impl APIClient {
  pub fn new(configuration: Configuration) -> APIClient {
    APIClient {
      configuration: configuration,
    }
  }
}
