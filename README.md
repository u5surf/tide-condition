# tide-condition

Outgoing reduce response body middleware with condition headers for the [Tide][] server framework.

```
#[async_std::main]
async fn main() -> tide::Result {
    let mut app = tide::new();
    app.with(tide_condition::ConditionMiddleware::new());
}
```

## Features
- Support for comparing ETag with If-None-Match values
  - if `If-None-Match` value matches with content entity tags, It only returns 304 response. 
- Sets the [`ETag`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Etag) headers.
  - ETag is md5 digest of requested contents.


[Tide]: https://github.com/http-rs/tide
