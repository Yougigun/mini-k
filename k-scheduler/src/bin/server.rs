#![allow(non_snake_case)]
use k_scheduler::kscheduler_server::{Kscheduler, KschedulerServer};
use k_scheduler::{
    ContainerInfo, CreateContainerRequest, CreateContainerResponse, DeleteContainerRequest,
    DeleteContainerResponse, GetContainerRequest, GetContainerResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod k_scheduler {
    tonic::include_proto!("k_scheduler");
}

#[derive(Debug, Default)]
pub struct MyKscheduler {}
#[tonic::async_trait]
impl Kscheduler for MyKscheduler {
    async fn create_container(
        &self,
        request: Request<CreateContainerRequest>,
    ) -> Result<Response<CreateContainerResponse>, Status> {
        let reply = k_scheduler::CreateContainerResponse {
            cid: "xxx".to_string(),
            name: "xxx".to_string(),
            code: 0,
            message: "xxx".to_string(),
        };
        Ok(Response::new(reply))
    }
    async fn get_container(
        &self,
        request: Request<GetContainerRequest>,
    ) -> Result<Response<GetContainerResponse>, Status> {
        let reply = k_scheduler::GetContainerResponse {
            container: Some(ContainerInfo {
                cid: request.into_inner().cid,
                name: "xxx".to_string(),
                image: "xxx".to_string(),
                instance_id: "xxx".to_string(),
                port: 0,
                status: "xxx".to_string(),
                ip: "xxx".to_string(),
                domain: "xxx".to_string(),
                env: std::collections::HashMap::new(),
                command: "xxx".to_string(),
            }),
            code: 0,
            message: "xxx".to_string(),
        };
        Ok(Response::new(reply))
    }
    // delete container
    async fn delete_container(
        &self,
        request: Request<DeleteContainerRequest>,
    ) -> Result<Response<DeleteContainerResponse>, Status> {
        let cid = request.into_inner().cid;
        let reply = k_scheduler::DeleteContainerResponse {
            cid,
            name: "xxx".to_string(),
            code: 0,
            message: "xxx".to_string(),
        };
        Ok(Response::new(reply))
    }

    // list containers
    async fn list_containers(
        &self,
        _request: Request<()>,
    ) -> Result<Response<k_scheduler::ListContainersResponse>, Status> {
        let reply = k_scheduler::ListContainersResponse {
            containers: vec![k_scheduler::ContainerInfo {
                cid: "xxx".to_string(),
                name: "xxx".to_string(),
                image: "xxx".to_string(),
                instance_id: "xxx".to_string(),
                port: 0,
                status: "xxx".to_string(),
                ip: "xxx".to_string(),
                domain: "xxx".to_string(),
                env: std::collections::HashMap::new(),
                command: "xxx".to_string(),
            }],
            code: 0,
            message: "xxx".to_string(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    let PORT = env::var("PORT").unwrap();
    let addr = format!("[::]:{}", PORT).parse()?;
    let k_scheduler_server = MyKscheduler::default();

    Server::builder()
        .add_service(KschedulerServer::new(k_scheduler_server))
        .serve(addr)
        .await?;

    Ok(())
}
