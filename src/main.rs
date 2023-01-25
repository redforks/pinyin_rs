use std::net::{Ipv4Addr, SocketAddr};
use warp::Filter;
use utoipa::{
    OpenApi
};

#[derive(OpenApi)]
#[openapi(
paths(hello)
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));
    let home = warp::path("home")
        .and(warp::get())
        .map(hello);
    let web = api_doc.or(home);

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
