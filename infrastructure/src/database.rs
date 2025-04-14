use std::sync::Arc;

use dotenv::{dotenv, var};
use postgrest::Postgrest;

use crate::error::InfrastructureError;

pub fn client() -> Result<Arc<Postgrest>, InfrastructureError> {
    dotenv().ok();

    let url = var("SUPABASE_URL")?;
    let key = var("SUPABASE_KEY")?;

    tracing::info!("Connecting to Supabase...");
    tracing::info!("{}", url);
    tracing::info!("{}", key);

    let client = Postgrest::new(url)
        .insert_header("apikey", key)
        .schema("public");

    tracing::info!("Connected to Supabase!");

    Ok(Arc::new(client))
}
