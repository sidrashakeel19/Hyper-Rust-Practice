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

// use std::convert::Infallible;
// use hyper::{Body, Request, Response, Server};
// use hyper::service::{make_service_fn, service_fn};

// async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new(Body::from("Hello from example 2 of hyper!")))
// }

// #[tokio::main]
// pub async fn main() -> Result<(),Box<dyn std::error::Error + Send + Sync>> {
//     pretty_env_logger::init();

//     //For every connection, we must make a 'Service' to handle all incoming
//     //HTTP requests on said connection

//     let make_svc = make_service_fn(|_conn| {
//          // This is the `Service` that will handle the connection.
//         // `service_fn` is a helper to convert a function that
//         // returns a Response into a `Service`.
//         async {
//             Ok::<_,Infallible>(service_fn(hello))
//         }
//     });

//     //creating socket addr on which server will respond
//     let addr = ([127,0,0,1], 3000).into();

//     //We'll bind to 127.0.0.1:3000
//     //Also assigning service to the server
//     let server = Server::bind(&addr).serve(make_svc);

//     println!("Listening on http://{}", addr);
//     server.await?;
//     Ok(())

// }

// Example 3 :  Routing

// use std::convert::Infallible;
// use hyper::service::{make_service_fn, service_fn};
// use hyper::{Body, Request, Response, Server, Method, StatusCode};

// #[tokio::main]
// pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     pretty_env_logger::init();
    
//     //For every connection, we must make a 'Service' to handle all incoming
//     //HTTP requests on said connection

//     let make_svc = make_service_fn(|_conn| {
//          // This is the `Service` that will handle the connection.
//         // `service_fn` is a helper to convert a function that
//         // returns a Response into a `Service`.
//         async {
//             Ok::<_,Infallible>(service_fn(echo))
//         }
//     });

//     //creating socket addr on which server will respond
//     let addr = ([127,0,0,1], 3000).into();

//     //We'll bind to 127.0.0.1:3000
//     //Also assigning service to the server
//     let server = Server::bind(&addr).serve(make_svc);


//     println!("Listening on http://{}", addr);
//     server.await?;
//     Ok(())
// }

// async fn echo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     let mut response = Response::new(Body::empty());

//     match (req.method(), req.uri().path()) {
//         (&Method::GET, "/") => {
//             *response.body_mut() = Body::from("Try POSTing data to /echo");
//         },
//         (&Method::POST, "/echo") => {
//             *response.body_mut() = req.into_body();
//         },
//         _ => {
//             *response.status_mut() = StatusCode::NOT_FOUND;
//         },
//     };

//     Ok(response)
// }

// -------------------------- Example 4 : Routing --------------------------
// -------------------------------------------------------------------------

use futures::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo"
            // "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        // Convert to uppercase before sending back to client using a stream.
        (&Method::POST, "/uppercase") => {
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
        }

        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive as we did with the above uppercase endpoint.
        // So here we do `.await` on the future, waiting on concatenating the full body,
        // then afterwards the content can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
