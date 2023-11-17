use interfaces::Provider;
use tenants::TenantService;
use users::UserService;

#[tokio::main]
async fn main() {
    let state = std::sync::Arc::new(Provider {
        user_service: &UserService {},
        tenant_service: &TenantService {},
    });

    let router = axum::Router::new()
        .nest("/users", users::router())
        .nest("/tenants", tenants::router())
        .with_state(state);

    println!("Server Started!");
    axum::Server::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(router.into_make_service())
        .await
        .unwrap();
}
