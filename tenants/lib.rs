use interfaces::Provider;

/// Builds the router for all /tenants routes
pub fn router() -> axum::Router<std::sync::Arc<Provider<'static>>> {
    axum::Router::new()
        .route("/", axum::routing::get(get_tenants))
        .route("/", axum::routing::post(create_tenant))
}

/// Route handler for GET /tenants
async fn get_tenants() -> String {
    println!("GET /tenants");
    String::from("Ok")
}

/// Route handler for POST /tenants
async fn create_tenant() -> String {
    println!("POST /tenants");
    String::from("Ok")
}

/// Users Service
pub struct TenantService;

impl interfaces::ITenantService for TenantService {
    fn get(&self) {
        println!("TenantService->get called");
    }
}
