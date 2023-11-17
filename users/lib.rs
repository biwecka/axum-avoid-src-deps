use std::sync::Arc;
use axum::extract::State;
use interfaces::{Provider, ITenantService, IUserService};

/// Builds the router for all /users routes
pub fn router() -> axum::Router<std::sync::Arc<Provider<'static>>> {
    axum::Router::new()
        .route("/", axum::routing::get(get_users))
        .route("/", axum::routing::post(create_user))
}

/// Route handler for GET /users
async fn get_users() -> String {
    println!("GET /users");
    String::from("Ok")
}

/// Route handler for POST /users
async fn create_user(State(s): State<Arc<Provider<'_>>>) -> String {
    println!("POST /users");

    // This is where `nject` allowed me to use `.provide()` like this:
    // `let tenant_service: &dyn ITenantService = s.provide();`
    let tenant_service: &dyn ITenantService = s.tenant_service;
    tenant_service.get();

    let user_service: &dyn IUserService = s.user_service;
    user_service.create();

    String::from("Ok")
}

/// Users Service
pub struct UserService;

impl interfaces::IUserService for UserService {
    fn create(&self) {
        println!("UserSerivce->create called");
    }
}
