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
    NewMessageRequest, NewMessageResponse, PeerClosedRequest, PeerClosedResponse};

struct ConnectedClient {
    user_id: String,
    tx: Sender<Result<NewMessageResponse, Status>>
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
    connected_peers_to_peers: Vec<ConnectedPeerToPeer>,
    connected_clients: HashMap<String, Sender<Result<NewMessageResponse, Status>>>
}

#[tonic::async_trait]
impl Chat for MyChat {

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

    type SearchingPeerStream=mpsc::Receiver<Result<SearchingPeerResponse,Status>>;
    async fn searching_peer(&mut self, request: Request<SearchingPeerRequest>) -> 
    Result<Response<Self::SearchingPeerStream>, Status>
    {
        println!("Got a searching_peer request from {:?}", request.remote_addr());
        
        let (mut tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();
        
        self.searching_peers.entry(user_id_from_request.clone()).or_insert(request.get_ref().radius_distance_in_meters);


        /*fixme: 
0. сохраняются user id у тех пользователей, который отключились(должны удаляться user id тех, кто отключился).
1. инфа о собеседнике отправляется одному клиенту(когда 2 клиента подходят и найдены, нужно отправить инфу обоим клиентам(user_id, status, radius))
        */
        let mut is_found_peer = false;
        for (key, val) in &self.searching_peers {
            if user_id_from_request != *key {
                if request.get_ref().radius_distance_in_meters == *val {
                    let new_connected_peer_to_peer = ConnectedPeerToPeer{
                        user_id1: user_id_from_request.clone(),
                        user_id2: (*key).clone()
                    };
                    self.connected_peers_to_peers.push(new_connected_peer_to_peer);
                    let peer_id: String = (*key).clone();
                    let peer_radius_distance_in_meters = request.get_ref().radius_distance_in_meters;
                    let peer_status = request.get_ref().status.clone();
                    is_found_peer = true;

                    let mut tx_tmp = tx.clone();
                    tokio::spawn(async move {
                        // looping and sending our response using stream
                        let reply = chatservice::SearchingPeerResponse {
                            response_code: 1,
                            user_id: peer_id,
                            radius_distance_in_meters: peer_radius_distance_in_meters,
                            status: peer_status
                        };
                        // sending response to our channel
                        tx_tmp.send(Ok(reply)).await;
                    });
                }
            }
        }

        if is_found_peer == true {
            // returning our reciever so that tonic can listen on reciever and send the response to client
            return Ok(Response::new(rx));
        } else {
            tokio::spawn(async move {
                for _ in 0..1 {
                    let reply = chatservice::SearchingPeerResponse {
                        response_code: 2,
                        user_id: "no_user_id".to_string(),
                        radius_distance_in_meters: -1,
                        status: "".to_string()
                    };
                    tx.send(Ok(reply)).await;
                }
            });
            return Ok(Response::new(rx));
        }
    }

    type NewMessageStream=mpsc::Receiver<Result<NewMessageResponse, Status>>;
    async fn new_message(
        &mut self,
        request: Request<NewMessageRequest>,
    ) -> Result<Response<Self::NewMessageStream>, Status>
    {
        println!("Got a new_message request from {:?}", request.remote_addr());
        //println!("Message: {}!", request.into_inner().message);

        let (mut tx, rx) = mpsc::channel(4);
        

        let user_id_from_request = request.get_ref().user_id.clone();
        let message_from_request = request.get_ref().message.clone();

        if message_from_request == "" {
            // fixme: doubtful situation, user not receiving message from server
            self.connected_clients.entry(user_id_from_request.clone()).or_insert(tx.clone());
        } else {
            for connected_peer_to_peer in &self.connected_peers_to_peers {
                if connected_peer_to_peer.user_id1 == user_id_from_request {
                    let user_id2 = &(connected_peer_to_peer.user_id2);
                    // send message to user_id2
                    let tx_tmp_ref = self.connected_clients.get(user_id2).unwrap();
                    let mut tx_tmp = (*tx_tmp_ref).clone();
                    let reply = chatservice::NewMessageResponse {
                        response_code: 1,
                        message: message_from_request.clone()
                    };
                    tokio::spawn(async move {
                        tx_tmp.send(Ok(reply)).await;
                        println!("sent a new_message ");
                    });
                    break;
                } else if connected_peer_to_peer.user_id2 == user_id_from_request {
                    let user_id1 = &(connected_peer_to_peer.user_id1);
                    // send message to user_id1
                    let tx_tmp_ref = self.connected_clients.get(user_id1).unwrap();
                    let mut tx_tmp = (*tx_tmp_ref).clone();
                    let reply = chatservice::NewMessageResponse {
                        response_code: 1,
                        message: message_from_request.clone()
                    };
                    tokio::spawn(async move {
                        // sending response to our channel
                        tx_tmp.send(Ok(reply)).await;
                        println!("sent a new_message ");
                    });
                    break;
                }
            }
        }
        return Ok(Response::new(rx));
    }
    
    async fn peer_closed(
        &mut self,
        request: Request<PeerClosedRequest>,
    ) -> Result<Response<PeerClosedResponse>, Status>{
        println!("Got a peer_closed request from {:?}", request.remote_addr());
        let user_id_from_request = request.get_ref().user_id.clone();

        // удалить из searching_peers user_id, если остался
        self.searching_peers.remove(&user_id_from_request);
        
        // удалить из connected_peers_to_peers записать по user_id, если есть
        self.connected_peers_to_peers.retain(|connected_peer|{
            &connected_peer.user_id1 == &user_id_from_request || &connected_peer.user_id2 == &user_id_from_request
        });

        self.connected_clients.remove(&user_id_from_request);
        
        let reply = chatservice::PeerClosedResponse {
            response_code: 1
        };
        return Ok(Response::new(reply));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "192.168.0.100:50051".parse().unwrap();
    let chat = MyChat::default();
    //chat.searching_peers = HashMap::new();
    //chat.connected_peers_to_peers = Vec::new();
    //chat.connected_clients = HashMap::new();

    println!("ChatServer listening on {}", addr);

    Server::builder()
        .add_service(ChatServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}
