use salvo::Router;

use crate::schemas::AppState;

mod docs;
mod statics;
mod v1;

use docs::{setup_openapi, setup_swagger_ui};

pub fn setup(app_state: &AppState, middlewares: Router) -> Router {
    let mut routers: Router = Router::new().path("api").push(v1::setup());
    routers = middlewares.push(routers);
    routers = setup_openapi(routers);
    routers = setup_swagger_ui(routers);

    // push static assets into routers.
    routers = routers.push(statics::setup_frontend(app_state.configs.frontend.clone()));

    routers
}
