use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;

pub fn create_reqwest_client() -> ClientWithMiddleware {
    // retry up to 3 times with increasing intervals between attempts.
    let retry_policy: ExponentialBackoff = ExponentialBackoff::builder().build_with_max_retries(3);
    ClientBuilder::new(reqwest::Client::new())
        // trace HTTP requests. See the tracing crate to make use of these traces.
        .with(TracingMiddleware::default())
        // retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}
