use anyhow::Ok;
use salvo::prelude::*;
use api::{EchoRequest, EchoResponse, echo_service_client::EchoServiceClient};

mod error;

#[handler]
async fn hello() -> &'static str{
    "Hello, this a echo microservice project."
}

#[handler]
async fn echo(req: &mut Request, resp: &mut Response) {
    let message = req.form::<String>("message").await.unwrap();
    println!("gateway get request {:?}", message);

    let echo_requst = EchoRequest { message };

    let mut client = EchoServiceClient::connect("http://service1:8001").await.unwrap();
    let echo_response = client.echo_from_service1(echo_requst).await.unwrap();
    resp.render(Text::Plain(echo_response.into_inner().message));
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .hoop(CatchPanic::new())
        .get(hello)
        .push(
            Router::with_path("echo")
                .post(echo)
        );
    println!("{router:#?}");
    let acceptor = TcpListener::new("[::0]:8000").bind().await;
    Server::new(acceptor).serve(router).await;
}
