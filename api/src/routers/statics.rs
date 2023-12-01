use salvo::prelude::*;
use salvo::serve_static::StaticDir;

pub fn setup_frontend(path: String) -> Router {
    let frontend: StaticDir = StaticDir::new([path]).defaults("index.html").auto_list(true);

    Router::new().path("<**rest_path>").get(frontend)
}
