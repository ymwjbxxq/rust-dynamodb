pub mod handler {
  use crate::dtos::request::Request;
  use crate::models::product::Product;
  use crate::queries::get_by_id::GetById;
  use crate::queries::get_by_id::GetByIdQuery;
  use lambda_runtime::{Context, Error};

  pub async fn execute(event: Request, _ctx: Context) -> Result<Option<Product>, Error> {
    log::info!("input {:?}", event);
    let id = event.pk.expect("id must be set");

    // init aws config
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    log::info!("Fetching product {}", id);
    let product = GetById::new(client).await
                          .execute(&id).await?;

    Ok(match product {
      Some(item) => Some(item),
      None => None,
    })
  }
}
