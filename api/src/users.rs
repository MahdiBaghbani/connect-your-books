use salvo::oapi::extract::*;
use salvo::prelude::*;

#[endpoint]
async fn hello(name: QueryParam<String, false>) -> String {
    format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}

pub fn router() -> Router {
    Router::new()
        .path("users")
        .push(Router::new().path("hello").get(hello))
}
