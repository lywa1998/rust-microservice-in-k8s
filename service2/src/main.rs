use api::echo_service_server::{EchoService, EchoServiceServer};
use api::{EchoRequest, EchoResponse};
use tonic::{transport::Server, Request, Response, Status};

struct EchoImplement;

#[tonic::async_trait]
impl EchoService for EchoImplement {
    async fn echo_from_service1(
        &self,
        request: Request<EchoRequest> ,
    ) ->  Result<Response<EchoResponse>, Status> {
        unimplemented!()
    }

    async fn echo_from_service2(
        &self,
        request: Request<EchoRequest> ,
    ) ->  Result<Response<EchoResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = EchoResponse {
            message: format!("Hello {} from service 2!", request.into_inner().message),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8002".parse().unwrap();
    let greeter = EchoImplement;

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(EchoServiceServer::new(greeter))
        .serve(addr)
        .await.unwrap();
}
