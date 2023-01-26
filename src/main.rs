use std::net::{Ipv4Addr, SocketAddr};
use warp::{Filter, Reply};
#[cfg(feature = "swagger")]
use warp::{http::{Response, StatusCode, Uri},
           path::{FullPath, Tail},
           Rejection,
};
#[cfg(feature = "swagger")]
use utoipa_swagger_ui::Config;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let home = warp::path!("hello" / String)
        .and(warp::get())
        .map(hello);
    let web = home;
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3030));
    #[cfg(feature = "swagger")] {
        use utoipa::OpenApi;

        #[derive(OpenApi)]
        #[openapi(paths(hello))]
        struct ApiDoc;

        let api_doc = warp::path("api-doc.json")
            .and(warp::get())
            .map(|| warp::reply::json(&ApiDoc::openapi()));
        let web = web.or(api_doc);
        let config = std::sync::Arc::new(Config::from("/api-doc.json"));
        let swagger_ui = warp::path("swagger-ui")
            .and(warp::get())
            .and(warp::path::full())
            .and(warp::path::tail())
            .and(warp::any().map(move || config.clone()))
            .and_then(serve_swagger);
        let web = web.or(swagger_ui);
        warp::serve(web).run(addr).await
    }

    #[cfg(not(feature = "swagger"))] {
        warp::serve(web).run(addr).await
    }
}

#[cfg_attr(feature = "swagger",
utoipa::path(
get,
path = "/hello/{name}",
responses((status = 200, description = "hello")),
params(
    ("name"=String, Path, description="name to hello")
)
))]
fn hello(name: String) -> impl Reply {
    warp::reply::html( format!("<h1>Hello, {}!</h1>", name) )
}

#[cfg(feature = "swagger")]
async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: std::sync::Arc<Config<'static>>,
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
