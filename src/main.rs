//Example 1 : Getting Started with Server

// use std::convert::Infallible;
// use std::net::SocketAddr;
// use hyper::{Body, Request, Response, Server};
// use hyper::service::{make_service_fn, service_fn};

// #[tokio::main]
// async fn main() {
//     //we'll bind to 127.0.0.1:3000
//     let addr = SocketAddr::from(([127,0,0,1], 3000));

//     //A 'Service' is needed for every connection, sp this
//     //creates one from our "hello_world" function

//     let server = Server::bind(&addr).serve(make_service_fn(|_conn| async {
//         //service_fn converts our function into a 'Service'
//         Ok::<_, Infallible>(service_fn(hello_world))
//     }));

//     //Run this server for...forever!

//     if let Err(e) = server.await {
//         eprintln!("Server error: {}", e);
//     }
// }

// async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new("Hello, Sidra! - From HYper".into()))
// }

//Example 2 : Getting Started with Server

use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello from example 2 of hyper!")))
}

#[tokio::main]
pub async fn main() -> Result<(),Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    //For every connection, we must make a 'Service' to handle all incoming
    //HTTP requests on said connection

    let make_svc = make_service_fn(|_conn| {
         // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async {
            Ok::<_,Infallible>(service_fn(hello))
        }
    });

    //creating socket addr on which server will respond
    let addr = ([127,0,0,1], 3000).into();

    //We'll bind to 127.0.0.1:3000
    //Also assigning service to the server
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())

}