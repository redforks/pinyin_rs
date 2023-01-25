use std::net::{Ipv4Addr, SocketAddr};
use warp::Filter;

#[tokio::main]
async fn main() {
    let web = warp::path("home")
        .and(warp::get())
        .then(|| async {
        warp::reply::html(
            "<h1>Hello, world!</h1>".to_string(),
        )
    });

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3030));
    warp::serve(web).run(addr).await
}
