use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Router};
use tower_http::services::ServeDir;

#[no_mangle]
pub fn add_route(router: Router) -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    let service = handle_404.into_service();

    let serve_dir = ServeDir::new("binaries/dist").not_found_service(service);

    router
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
}
