use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use std::collections::HashMap;

/*pub mod chatservice {
    tonic::include_proto!("chatservice");
}*/

mod chatservice;
use chatservice::chat_server::{Chat, ChatServer};
use chatservice::{NewPeerRequest, NewPeerResponse, SearchingPeerRequest, SearchingPeerResponse, 
    NewMessageRequest, NewMessageResponse};

struct ConnectedClient {
    user_id: String,
    connection: Sender<Result<SearchingPeerResponse, Status>>
}

struct SearchingPeer {
    user_id: String,
    radius_distance_in_meters: i32
}

struct ConnectedPeerToPeer {
    user_id1: String,
    user_id2: String,
}

#[derive(Default)]
pub struct MyChat {
    searching_peers: HashMap<String, i32>,//Vec<SearchingPeer>,
    connected_peers_to_peers: Vec<ConnectedPeerToPeer>
}

#[tonic::async_trait]
impl Chat for MyChat {
    type SearchingPeerStream=mpsc::Receiver<Result<SearchingPeerResponse,Status>>;

    async fn new_peer(&mut self, request: Request<NewPeerRequest>)-> Result<Response<NewPeerResponse>, Status>
    {
        println!("Got a new peer request from {:?}", request.remote_addr());

        //let connected_client = ConnectedClient{user_id: request.get_ref().user_id.clone(), connection: tx.clone()};
        //self.connected_clients.push(connected_client);

        let reply = chatservice::NewPeerResponse {
            response_code: 1
        };
        return Ok(Response::new(reply));
    }

    async fn searching_peer(&mut self, request: Request<SearchingPeerRequest>) -> 
    Result<Response<Self::SearchingPeerStream>, Status>
    {
        println!("Got a searching_peer request from {:?}", request.remote_addr());
        
        let (mut tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();
        
        self.searching_peers.entry(user_id_from_request.clone()).or_insert(request.get_ref().radius_distance_in_meters);

        let mut peer_id: String = "".to_owned();
        let mut is_found_peer = false;
        for (key, val) in &self.searching_peers {
            if user_id_from_request != *key {
                if request.get_ref().radius_distance_in_meters == *val {
                    let new_connected_peer_to_peer = ConnectedPeerToPeer{user_id1: user_id_from_request.clone(),
                         user_id2: (*key).clone()};
                    self.connected_peers_to_peers.push(new_connected_peer_to_peer);
                    peer_id = (*key).clone();
                    is_found_peer = true;
                    break;
                }
            }
        }

        if is_found_peer == true {
            tokio::spawn(async move {
                // looping and sending our response using stream
                    let reply = chatservice::SearchingPeerResponse {
                        response_code: 1,
                        user_id: peer_id
                    };
                // sending response to our channel
                    tx.send(Ok(reply)).await;
                });
                // returning our reciever so that tonic can listen on reciever and send the response to client
            return Ok(Response::new(rx));
        } else {
            tokio::spawn(async move {
                for _ in 0..1 {
                    let reply = chatservice::SearchingPeerResponse {
                        response_code: 2,
                        user_id: "no_user_id".to_string()
                    };
                    tx.send(Ok(reply)).await;
                }
            });
            return Ok(Response::new(rx));
        }
    }

    async fn new_message(
        &self,
        request: Request<NewMessageRequest>,
    ) -> Result<Response<NewMessageResponse>, Status>
    {
        println!("Got a new_message request from {:?}", request.remote_addr());
        //println!("Message: {}!", request.into_inner().message);

        for connected_peer_to_peer in &self.connected_peers_to_peers {
            if connected_peer_to_peer.user_id1 == request.get_ref().user_id {

            } else if connected_peer_to_peer.user_id2 == request.get_ref().user_id {

            }
        }

        let reply = chatservice::NewMessageResponse {
            response_code: 1,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "192.168.0.100:50051".parse().unwrap();
    let chat = MyChat::default();

    println!("ChatServer listening on {}", addr);

    Server::builder()
        .add_service(ChatServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}
