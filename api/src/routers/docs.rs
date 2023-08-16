use salvo::prelude::*;

pub fn setup_openapi(routers: Router) -> Router {
    // create api documentation from routers.
    let docs: OpenApi = OpenApi::new("Connect Your Books API", "0.0.1").merge_router(&routers);

    // push to routes.
    routers.push(docs.into_router("/api/docs/openapi.json"))
}

pub fn setup_swagger_ui(routers: Router) -> Router {
    // create swagger ui from json.
    let swagger_ui: SwaggerUi = SwaggerUi::new("/api/docs/openapi.json");

    // push to routes.
    routers.push(swagger_ui.into_router("swagger-ui"))
}
