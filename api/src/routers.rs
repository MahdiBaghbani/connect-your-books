use salvo::Router;

mod docs;
mod v1;

use docs::{setup_swagger_ui, setup_openapi};

pub fn setup(middlewares: Router) -> Router {
    let mut routers: Router = Router::new().path("api").push(v1::setup());
    routers = middlewares.push(routers);
    routers = setup_openapi(routers);
    routers = setup_swagger_ui(routers);

    routers
}
