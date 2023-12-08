use salvo::prelude::*;

pub fn setup_open_api(router: Router, path: &str, title: &str, version: &str) -> Router {
    // create api documentation from routers.
    let docs: OpenApi = OpenApi::new(title, version).merge_router(&router);

    // push to router.
    router.unshift(docs.into_router(path))
}

pub fn setup_swagger(router: Router, path: String, open_api_path: String) -> Router {
    // create swagger ui from json.
    let swagger: SwaggerUi = SwaggerUi::new(open_api_path);

    // push to router.
    router.unshift(swagger.into_router(path))
}

pub fn setup_scalar(router: Router, path: String, open_api_path: String) -> Router {
    // create scalar from json.
    let scalar: Scalar = Scalar::new(open_api_path);

    // push to router.
    router.unshift(scalar.into_router(path))
}

pub fn setup_rapidoc(router: Router, path: String, open_api_path: String) -> Router {
    // create rapidoc from json.
    let rapidoc: RapiDoc = RapiDoc::new(open_api_path);

    // push to router.
    router.unshift(rapidoc.into_router(path))
}

pub fn setup_redoc(router: Router, path: String, open_api_path: String) -> Router {
    // create redoc from json.
    let redoc: ReDoc = ReDoc::new(open_api_path);

    // push to router.
    router.unshift(redoc.into_router(path))
}
