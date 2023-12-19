use salvo::Router;

use crate::routers::docs::{
    setup_open_api, setup_rapidoc, setup_redoc, setup_scalar, setup_swagger,
};
use crate::schemas::AppState;

mod docs;
mod healthcheck;
mod statics;
mod v1;

pub fn setup(app_state: &AppState, middlewares: Router) -> Router {
    // merge all routers into one router.
    let mut router: Router = Router::new().path("api").push(healthcheck::setup());

    router = middlewares.push(router);

    if app_state.configs.open_api.enabled {
        // generate openapi.json.
        let open_api_path: String = app_state.configs.open_api.url_path.clone();
        router = setup_open_api(
            router,
            &open_api_path,
            "Connect Your Books OpenAPI Document.",
            "v0.0.1",
        );

        if app_state.configs.open_api.swagger.enabled {
            // enable swagger ui.
            router = setup_swagger(
                router,
                app_state.configs.open_api.swagger.url_path.clone(),
                open_api_path.clone(),
            );
        }

        if app_state.configs.open_api.scalar.enabled {
            // enable scalar.
            router = setup_scalar(
                router,
                app_state.configs.open_api.scalar.url_path.clone(),
                open_api_path.clone(),
            );
        }

        if app_state.configs.open_api.rapi_doc.enabled {
            // enable rapidoc.
            router = setup_rapidoc(
                router,
                app_state.configs.open_api.rapi_doc.url_path.clone(),
                open_api_path.clone(),
            );
        }

        if app_state.configs.open_api.re_doc.enabled {
            // enable redoc.
            router = setup_redoc(
                router,
                app_state.configs.open_api.re_doc.url_path.clone(),
                open_api_path.clone(),
            );
        }
    }

    // push static assets into routers.
    router = router.push(statics::setup_frontend(app_state.configs.frontend.clone()));

    router
}
