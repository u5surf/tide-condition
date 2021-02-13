use tide::http::{headers, Method, Request, StatusCode, Url};
use tide::Response;

const TEXT: &str = concat![
    "Chunk one\n",
    "data data\n",
    "\n",
    "Chunk two\n",
    "data data\n",
    "\n",
    "Chunk three\n",
    "data data\n",
];

#[async_std::test]
async fn etag_response() {
    let mut app = tide::new();
    app.with(tide_condition::ConditionMiddleware::new());
    app.at("/").get(|_| async {
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(TEXT.to_owned());
        Ok(res)
    });

    let req = Request::new(Method::Get, Url::parse("http://_/").unwrap());
    let mut res: tide::http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res[headers::ETAG], "\"a176d42deb17b7d468560f3c27996ebf\"");
    assert_eq!(res.body_string().await.unwrap(), TEXT);
}

#[async_std::test]
async fn if_none_match_304_response() {
    let mut app = tide::new();
    app.with(tide_condition::ConditionMiddleware::new());
    app.at("/").get(|_| async {
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(TEXT.to_owned());
        Ok(res)
    });

    let mut req = Request::new(Method::Get, Url::parse("http://_/").unwrap());
    req.append_header(headers::IF_NONE_MATCH, "\"a176d42deb17b7d468560f3c27996ebf\"");
    let mut res: tide::http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), 304);
    assert_eq!(res[headers::ETAG], "\"a176d42deb17b7d468560f3c27996ebf\"");
    assert_eq!(res.body_string().await.unwrap(), "");
}

#[async_std::test]
async fn if_none_match_200_response() {
    let mut app = tide::new();
    app.with(tide_condition::ConditionMiddleware::new());
    app.at("/").get(|_| async {
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(TEXT.to_owned());
        Ok(res)
    });

    let mut req = Request::new(Method::Get, Url::parse("http://_/").unwrap());
    req.append_header(headers::IF_NONE_MATCH, "\"b176d42deb17b7d468560f3c27996ebf\"");
    let mut res: tide::http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res[headers::ETAG], "\"a176d42deb17b7d468560f3c27996ebf\"");
    assert_eq!(res.body_string().await.unwrap(), TEXT);
}

