use salvo::Router;

use docs::{setup_openapi, setup_swagger_ui};

use crate::schemas::AppState;

mod docs;
mod healthcheck;
mod statics;
mod v1;

pub fn setup(app_state: &AppState, middlewares: Router) -> Router {
    // merge all routers into one router.
    let mut router: Router = Router::new()
        .path("api")
        .push(healthcheck::setup())
        .push(v1::setup());

    router = middlewares.push(router);
    // generate openapi.json.
    router = setup_openapi(router);
    // enable swagger ui.
    router = setup_swagger_ui(router);

    // push static assets into routers.
    router = router.push(statics::setup_frontend(app_state.configs.frontend.clone()));

    router
}
