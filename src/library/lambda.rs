pub mod handler {
  use crate::dtos::request::Request;
  use crate::models::product::Product;
  use crate::queries::get_by_id::GetById;
  use crate::queries::get_by_id::GetByIdQuery;
  use lambda_runtime::{Context, Error};

  pub async fn execute(event: Request, _ctx: Context) -> Result<Option<Product>, Error> {
    log::info!("input {:?}", event);
    let id = event.pk.expect("id must be set");
    let query = GetById::new().await;
    log::info!("Fetching product {}", id);
    let product = query.execute(&id).await?;

    Ok(match product {
       Some(item) => Some(item),
       None => None,
    })
  }
}
