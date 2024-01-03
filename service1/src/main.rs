use api::echo_service_client::EchoServiceClient;
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
        println!("Got a request from {:?}", request.remote_addr());

        let mut client = EchoServiceClient::connect("http://service2:8002").await.unwrap();

        let response = client.echo_from_service2(request).await.unwrap();
        
        let reply = EchoResponse {
            message: format!("{}\n This message is forwarded from service 1!", response.into_inner().message),
        };

        Ok(Response::new(reply))
    }

    async fn echo_from_service2(
        &self,
        request: Request<EchoRequest> ,
    ) ->  Result<Response<EchoResponse>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() {
    let addr = "[::0]:8001".parse().unwrap();
    let greeter = EchoImplement;

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(EchoServiceServer::new(greeter))
        .serve(addr)
        .await.unwrap();
}
