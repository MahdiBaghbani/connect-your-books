use salvo::Router;

mod v1;

pub fn setup(middlewares: Router) -> Router {
    let router: Router = Router::new().path("api").push(v1::setup());
    middlewares.push(router)
}
