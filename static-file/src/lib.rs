use axum::Router;
use axum::routing::get;

#[no_mangle]
pub fn add_route(router: Router) -> Router {
    router.route("/c", get(|| async { "C" }))
}
