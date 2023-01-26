use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use warp::http::{Response, StatusCode, Uri};
use warp::path::{FullPath, Tail};
use utoipa::{
    OpenApi
};
use utoipa_swagger_ui::Config;

#[derive(OpenApi)]
#[openapi(
paths(hello)
)]
struct ApiDoc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let config = Arc::new(Config::from("/api-doc.json"));
    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));
    let home = warp::path("hello")
        .and(warp::get())
        .map(hello);
    let web = api_doc.or(home).or(swagger_ui);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3030));
    warp::serve(web).run(addr).await
}

#[utoipa::path(
get,
path = "/hello",
responses((status = 200, description = "hello"))
)]
fn hello() -> impl warp::Reply {
    warp::reply::html(
        "<h1>Hello, world!</h1>".to_string(),
    )
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
