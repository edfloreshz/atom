use atom_infrastructure::SupabaseClient;

#[derive(Clone)]
pub struct AppState {
    database: SupabaseClient,
}

impl AppState {
    pub fn new(database: SupabaseClient) -> Self {
        Self { database }
    }
}
