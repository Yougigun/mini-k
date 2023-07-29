#![allow(non_snake_case)]
use k_scheduler::greeter_client::GreeterClient;
use k_scheduler::HelloRequest;

pub mod k_scheduler {
    tonic::include_proto!("k_scheduler");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);
    println!("{}", response.into_inner().message);

    Ok(())
}
