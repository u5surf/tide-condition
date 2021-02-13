use tide::http::conditional::{IfNoneMatch, ETag};
use tide::{Body, Middleware, Next, Request, Response, StatusCode};

/// A middleware for request static files with conditional header.
///
/// ## Example
/// ```rust
/// # async_std::task::block_on(async {
/// let mut app = tide::new();
///
/// app.with(tide_condition::ConditionMiddleware::new());
/// # })
/// ```
#[derive(Clone, Debug)]
pub struct ConditionMiddleware {}

impl Default for ConditionMiddleware {
    fn default() -> Self {
        ConditionMiddleware {}
    }
}

impl ConditionMiddleware {
    /// Creates a new CompressMiddleware.
    ///
    /// Uses the default minimum body size threshold (1024 bytes).
    ///
    /// ## Example
    /// ```rust
    /// # async_std::task::block_on(async {
    /// let mut app = tide::new();
    ///
    /// app.with(tide_condition::ConditionMiddleware::new());
    /// # })
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ConditionMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        // Incoming Request data
        // Need to grab these things before the request is consumed by `next.run()`.
        // Propagate to route
        let if_none_match = IfNoneMatch::from_headers(&req)?;
        let mut res: Response = next.run(req).await;
        let body_str = res.take_body().into_string().await.unwrap();
        let etag = get_etag(&body_str);
        if if_none_match.iter().any(|directive| directive.value() == etag.value()) {
            res = Response::new(StatusCode::NotModified);
            etag.apply(&mut res);
            return Ok(res);
        }
        res.set_body(string_to_body(&body_str));
        etag.apply(&mut res);
        Ok(res)
    }
}

fn get_etag(body : &str) -> ETag {
    ETag::new(format!("{:x}", md5::compute(body.to_string())))
}

fn string_to_body(body: &str) -> Body {
    Body::from_string(body.to_string())
}
