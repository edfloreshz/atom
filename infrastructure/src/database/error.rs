#[derive(Debug)]
pub enum DatabaseError {
    SupabaseError(supabase_rs::errors::ErrorTypes),
    DotEnvError(dotenv::Error),
}
