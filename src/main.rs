use std::net::{Ipv4Addr, SocketAddr};
use serde::Deserialize;
use warp::{Filter, Reply};
#[cfg(feature = "swagger")]
use warp::{http::{Response, StatusCode, Uri},
           path::{FullPath, Tail},
           Rejection,
};
#[cfg(feature = "swagger")]
use utoipa_swagger_ui::Config;
use pinyin::ToneRepresentation;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let home = warp::path!("hello" / String)
        .and(warp::get())
        .map(hello);
    let pinyin = warp::path!("pinyin" / String)
        .and(warp::get())
        .and(warp::query::<PinYinQuery>())
        .map(pinyin_handler);
    let web = home.or(pinyin);
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3030));
    #[cfg(feature = "swagger")] {
        use utoipa::OpenApi;

        #[derive(OpenApi)]
        #[openapi(paths(hello, pinyin_handler))]
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

#[derive(Deserialize)]
pub struct PinYinQuery {
    #[serde(alias = "t", default)]
    tone_repr: ToneRepresentation,
}

/// Return pinyin of a Chinese characters separated by space.
#[cfg_attr(feature = "swagger",
utoipa::path(
get,
path = "/pinyin/{s}",
responses((status = 200, description = "Return pinyin of a Chinese characters separated by space")),
params(
    ("s"=String, Path, description="String to convert"),
    ("t"=inline(Option<ToneRepresentation>), Query, description="How to represent the tone of a pinyin syllable."),
)
))]
fn pinyin_handler(s: String, q: PinYinQuery) -> impl Reply {
     pinyin::pinyin(&s, q.tone_repr)
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
