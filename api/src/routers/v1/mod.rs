use salvo::Router;

mod hello;

pub fn setup() -> Router {
    Router::new().path("v1").push(hello::router())
}
