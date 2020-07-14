use tonic::{transport::Server, Request, Response, Status};

pub mod chatservice {
    tonic::include_proto!("chatservice");
}

use chatservice::chat_server::{Chat, ChatServer};
use chatservice::{NewPeerRequest, NewPeerResponse, NewMessageRequest, NewMessageResponse};

#[derive(Default)]
pub struct MyChat {}

#[tonic::async_trait]
impl Chat for MyChat {

    async fn new_peer(&self, request: Request<NewPeerRequest>)-> Result<Response<NewPeerResponse>, Status> {
        println!("Got a new peer request from {:?}", request.remote_addr());

        let reply = chatservice::NewPeerResponse {
            response_code: 1
        };
        return Ok(Response::new(reply));
    }

    async fn new_message(
        &self,
        request: Request<NewMessageRequest>,
    ) -> Result<Response<NewMessageResponse>, Status> {
        println!("Got a new_message request from {:?}", request.remote_addr());
        println!("Message: {}!", request.into_inner().message);

        let reply = chatservice::NewMessageResponse {
            response_code: 1,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let chat = MyChat::default();

    println!("ChatServer listening on {}", addr);

    Server::builder()
        .add_service(ChatServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}
