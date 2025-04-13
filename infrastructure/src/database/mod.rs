use dotenv::{dotenv, var};
use error::DatabaseError;
use supabase_rs::SupabaseClient;

pub mod error;
pub mod models;

pub fn database() -> Result<SupabaseClient, DatabaseError> {
    dotenv().ok();

    let url = var("SUPABASE_URL").map_err(DatabaseError::DotEnvError)?;
    let key = var("SUPABASE_KEY").map_err(DatabaseError::DotEnvError)?;

    SupabaseClient::new(url, key).map_err(DatabaseError::SupabaseError)
}
