use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn set_cookie(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let cookies = request.extensions().get::<Cookies>().unwrap().clone();

        let name = request.into_inner().name;

        let reply = hello_world::HelloReply {
            message: name.clone(),
        };

        let cookie = Cookie::new("name", name);

        cookies.add(cookie);

        Ok(Response::new(reply))
    }

    async fn get_cookie(&self, request: Request<()>) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let cookies = request.extensions().get::<Cookies>().unwrap().clone();

        let name = if let Some(cookie) = cookies.get("name") {
            cookie.to_string()
        } else {
            "name cookie not found".to_string()
        };

        let reply = hello_world::HelloReply { message: name };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000".parse().unwrap();

    let greeter = MyGreeter::default();
    let greeter = GreeterServer::new(greeter);
    let greeter = tonic_web::config()
        .allow_origins(vec!["127.0.0.1"])
        .enable(greeter);

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(CookieManagerLayer::new())
        .add_service(greeter)
        .serve(addr)
        .await?;

    Ok(())
}
