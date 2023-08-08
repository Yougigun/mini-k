use std::ops::Deref;

use actix_web::{http, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use k_scheduler::kscheduler_client::KschedulerClient;
use k_scheduler::{
    ContainerInfo, CreateContainerRequest, CreateContainerResponse, DeleteContainerRequest,
    DeleteContainerResponse, GetContainerRequest, GetContainerResponse, ListContainersResponse,
};

pub mod k_scheduler {
    tonic::include_proto!("k_scheduler");
}

pub fn buildScope(prefix: &str) -> actix_web::Scope {
    web::scope(prefix)
        .route("", web::get().to(list_container)) // get all containers
        .route("", web::post().to(create_container)) // create new container
        .route("/{cid}", web::get().to(get_container)) //get specific container
        .route("/{cid}", web::delete().to(delete_container)) //delete container
}

async fn list_container(config: web::Data<crate::Config>) -> impl Responder {
    let mut client = KschedulerClient::connect(config.k_scheduler_url.clone())
        .await
        .unwrap();
    let request = tonic::Request::new(());

    let grpc_resp: tonic::Response<ListContainersResponse> =
        client.list_containers(request).await.unwrap();
    let containers = ListContainersHttpResp {
        containers: grpc_resp
            .get_ref()
            .containers
            .iter()
            .map(|c| Container {
                cid: c.cid.clone(),
                name: c.name.clone(),
                status: c.status.clone(),
                image: c.image.clone(),
                command: c.command.clone(),
                env: c.env.clone(),
                port: c.port,
                instanceID: c.instance_id.clone(),
                ip: c.ip.clone(),
                domain: c.domain.clone(),
            })
            .collect(),
        code: grpc_resp.get_ref().code,
        message: grpc_resp.get_ref().message.clone(),
    };
    // let response = serde_json::to_string(&containers).unwrap();
    HttpResponse::Ok().json(containers)
}

// a req struct for create container
#[derive(Serialize, Deserialize, Default)]
struct CreateContainerReq {
    name: String,
    image: String,
    instance: String,
}

async fn create_container(req_body: String, config: web::Data<crate::Config>) -> impl Responder {
    let mut client = KschedulerClient::connect(config.k_scheduler_url.clone())
        .await
        .unwrap();

    let http_req: CreateContainerHttpRequest = serde_json::from_str(&req_body).unwrap();
    let request = tonic::Request::new(CreateContainerRequest {
        name: http_req.name,
        image: http_req.image,
        command: http_req.command,
        env: http_req.env,
        port: http_req.port,
        cpu: http_req.cpu,
        memory: http_req.memory,
    });

    let resp_grpc: tonic::Response<k_scheduler::CreateContainerResponse> =
        client.create_container(request).await.unwrap();
    let CreateContainerResponse {
        cid,
        name,
        code,
        message,
    } = resp_grpc.into_inner();

    HttpResponse::Ok().json(CreateContainerHttpResp {
        cid,
        name,
        code,
        message,
    })
}

async fn get_container(
    _req_body: String,
    config: web::Data<crate::Config>,
    cid: web::Path<String>,
) -> impl Responder {
    let mut client = KschedulerClient::connect(config.k_scheduler_url.clone())
        .await
        .unwrap();
    let request = tonic::Request::new(GetContainerRequest {
        cid: cid.into_inner(),
    });
    let grpc_resp = client.get_container(request).await.unwrap();

    let GetContainerResponse {
        container: container_info,
        code,
        message,
    } = grpc_resp.into_inner();
    let container = container_info.map(
        |ContainerInfo {
             cid,
             name,
             status,
             image,
             command,
             env,
             port,
             instance_id,
             ip,
             domain,
         }| Container {
            cid,
            name,
            status,
            image,
            command,
            env,
            port,
            ip,
            domain,
            instanceID: instance_id,
        },
    );

    HttpResponse::Ok().json(GetContainerHttpResp {
        container,
        code,
        message,
    })
}

async fn delete_container(
    _req_body: String,
    config: web::Data<crate::Config>,
    cid: web::Path<String>,
) -> impl Responder {
    let mut client = KschedulerClient::connect(config.k_scheduler_url.clone())
        .await
        .unwrap();
    let grpc_req = tonic::Request::new(DeleteContainerRequest {
        cid: cid.into_inner(),
    });

    let grpc_resp = client
        .delete_container(grpc_req)
        .await
        .unwrap()
        .into_inner();
    let resp = DeleteContainerHttpResp {
        cid: grpc_resp.cid,
        name: grpc_resp.name,
        code: grpc_resp.code,
        message: grpc_resp.message,
    };
    HttpResponse::Ok().json(resp)
}

#[derive(Serialize, Deserialize)]
pub struct CreateContainerHttpRequest {
    pub image: String,
    pub name: String,
    pub command: String,
    pub env: std::collections::HashMap<String, String>,
    pub port: i32,
    pub cpu: i32,
    pub memory: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GetContainerHttpRequest {
    pub cid: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteContainerHttpRequest {
    pub cid: String,
}

#[derive(Serialize, Deserialize)]

pub struct DeleteContainerHttpResp {
    pub cid: String,
    pub name: String,
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetContainerHttpResp {
    pub container: Option<Container>,
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub cid: String,
    pub name: String,
    pub status: String,
    pub image: String,
    pub command: String,
    pub env: std::collections::HashMap<String, String>,
    pub port: i32,
    pub instanceID: String,
    pub ip: String,
    pub domain: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateContainerHttpResp {
    pub cid: String,
    pub name: String,
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListContainersHttpResp {
    pub containers: Vec<Container>,
    pub code: i32,
    pub message: String,
}
