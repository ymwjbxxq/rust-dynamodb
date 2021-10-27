use serde::{Deserialize};

#[derive(Deserialize, Debug, Default)]
pub struct Request {
  pub pk: Option<String>,
}
