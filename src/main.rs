use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/*pub mod chatservice {
    tonic::include_proto!("chatservice");
}*/

mod chatservice;
use chatservice::chat_server::{Chat, ChatServer};
use chatservice::{NewPeerRequest, NewPeerResponse, SearchingPeerRequest, SearchingPeerResponse, 
    NewMessageRequest, NewMessageResponse, NewCollectiveMessageRequest,
    NewCollectiveMessageResponse, TypingMessageRequest, TypingMessageResponse, 
    ChatClosedRequest, ChatClosedResponse, CollectiveChatClosedRequest, 
    CollectiveChatClosedResponse, PeerClosedRequest, PeerClosedResponse};

struct ConnectedClient {
    user_id: String,
    tx: Sender<Result<NewMessageResponse, Status>>
}

struct SearchingPeer {
    radius_distance_in_meters: i32,//replace radius_distance_in_meters with lat, lng
    status: String,
    tx: Sender<Result<SearchingPeerResponse, Status>>
}

struct ConnectedPeerToPeer {//1-to-1 relation for personal chat
    user_id1: String,
    user_id2: String,
}

#[derive(Default)]
pub struct MyChat {
    connected_clients: HashSet<String>,
    searching_peers: HashMap<String, SearchingPeer>,
    connected_peer_to_peer: HashMap<String, String>,
    connected_peer_to_peers: HashMap<String, HashSet<String>>,
    personal_chat_message_senders: HashMap<String, Sender<Result<NewMessageResponse, Status>>>,
    collective_chat_message_senders: HashMap<String, Sender<Result<NewCollectiveMessageResponse, Status>>>,
    typing_message_senders: HashMap<String, Sender<Result<TypingMessageResponse, Status>>>,
    chat_closed_clients: HashMap<String, Sender<Result<ChatClosedResponse, Status>>>,
    collective_chat_closed_clients: HashMap<String, Sender<Result<CollectiveChatClosedResponse, Status>>>,
    pending_messages: HashMap<String, VecDeque<String>>//todo: save not sent messages to the queue
}

#[tonic::async_trait]
impl Chat for MyChat {

    async fn new_peer(&mut self, request: Request<NewPeerRequest>)-> Result<Response<NewPeerResponse>, Status>
    {
        println!("Got a new peer request from {:?}", request.remote_addr());

        let user_id_from_request = request.get_ref().user_id.clone();
        println!("Got user_id_from_request: {}", &user_id_from_request);

        //let connected_client = ConnectedClient{user_id: request.get_ref().user_id.clone(), connection: tx.clone()};
        self.connected_clients.insert(user_id_from_request);

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
        // todo: replace radius distance with latitude longitude coordinates
        let radius_distance_in_meters_from_request = request.get_ref().radius_distance_in_meters;
        let status_from_request = request.get_ref().status.clone();
        
        let new_searching_peer = SearchingPeer{
            radius_distance_in_meters: radius_distance_in_meters_from_request,
            status: status_from_request.clone(),
            tx: tx.clone()
        };
        self.searching_peers.entry(user_id_from_request.clone()).or_insert(new_searching_peer);


        //сохранить stream к searching peer
        let mut is_found_peer = false;
        let mut found_peer_id = "".to_string();
        for (key, val) in &self.searching_peers {
            if &user_id_from_request != key {
                // todo: change radius distance to latitude longitude coords
                if radius_distance_in_meters_from_request == (*val).radius_distance_in_meters {
                    /*let new_connected_peer_to_peer = ConnectedPeerToPeer{
                        user_id1: user_id_from_request.clone(),
                        user_id2: (*key).clone()
                    };*/
                    //self.connected_peers_to_peers.push(new_connected_peer_to_peer);
                    //self.connected_peer_to_peers.entry(user_id_from_request.clone()).or_insert((*key).clone());
                    if self.connected_peer_to_peers.contains_key(&user_id_from_request) == true {
                        let peers = self.connected_peer_to_peers.get_mut(&user_id_from_request).unwrap();
                        peers.insert((*key).clone());
                    } else {
                        let mut peers = HashSet::new();
                        peers.insert((*key).clone());
                        self.connected_peer_to_peers.insert(user_id_from_request.clone(), peers);
                    }
                    if self.connected_peer_to_peers.contains_key(key) == true {
                        let peers = self.connected_peer_to_peers.get_mut(key).unwrap();
                        peers.insert(user_id_from_request.clone());
                    } else {
                        let mut peers = HashSet::new();
                        peers.insert(user_id_from_request.clone());
                        self.connected_peer_to_peers.insert((*key).clone(), peers);
                    }

                    is_found_peer = true;
                    found_peer_id = (*key).clone();

                    let peer_id: String = (*key).clone();
                    let peer_radius_distance_in_meters = (*val).radius_distance_in_meters;
                    let peer_status = (*val).status.clone();
                    let reply_to_peer1 = chatservice::SearchingPeerResponse {
                        response_code: 1,
                        user_id: peer_id,
                        radius_distance_in_meters: peer_radius_distance_in_meters,
                        status: peer_status
                    };

                    let mut tx_tmp = tx.clone();
                    tokio::spawn(async move {
                        // sending response to client
                        tx_tmp.send(Ok(reply_to_peer1)).await;
                    });

                    let peer2_id: String = user_id_from_request.clone();
                    let peer2_radius_distance_in_meters = radius_distance_in_meters_from_request;
                    let peer2_status = status_from_request.clone();
                    let reply_to_peer2 = chatservice::SearchingPeerResponse {
                        response_code: 1,
                        user_id: peer2_id,
                        radius_distance_in_meters: peer2_radius_distance_in_meters,
                        status: peer2_status
                    };

                    tx_tmp = (*val).tx.clone();
                    tokio::spawn(async move {
                        // sending response to client
                        tx_tmp.send(Ok(reply_to_peer2)).await;
                    });
                }
            }
        }

        if is_found_peer == true {
            // remove those searching peers who found each other
            /*self.searching_peers.remove(&user_id_from_request);
            self.searching_peers.remove(&found_peer_id);
            self.searching_peers.retain(|key, val|{
                key != &user_id_from_request && radius_distance_in_meters_from_request == (*val).radius_distance_in_meters
            });*/
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
        }
        // returning our reciever so that tonic can listen on reciever and send the response to client
        return Ok(Response::new(rx));
    }

    type NewMessageStream=mpsc::Receiver<Result<NewMessageResponse, Status>>;
    async fn new_message(&mut self, request: Request<NewMessageRequest>) -> 
    Result<Response<Self::NewMessageStream>, Status>
    {
        println!("Got a new_message request from {:?}", request.remote_addr());
        //println!("Message: {}!", request.into_inner().message);

        let (tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();
        let message_from_request = request.get_ref().message.clone();
        let user_id2_from_request = request.get_ref().to_user_id.clone();
        println!("Got user_id_from_request: {}", &user_id_from_request);
        println!("Got user_id2_from_request: {}", &user_id2_from_request);
        self.connected_peer_to_peer.entry(user_id_from_request.clone()).or_insert(user_id2_from_request.clone());

        if message_from_request == "" {
            self.personal_chat_message_senders.entry(user_id_from_request.clone()).or_insert(tx.clone());
            if self.pending_messages.contains_key(&user_id_from_request) == true {
                let messages = self.pending_messages.get_mut(&user_id_from_request);
                if let Some(msgs) = messages {
                    while msgs.len() > 0 {
                        let message = msgs.pop_front();
                        if let Some(msg) = message {
                            let reply = chatservice::NewMessageResponse {
                                response_code: 1,
                                message: msg
                            };
                            let mut tx_tmp = tx.clone();
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                            });
                        }
                    }
                }
            }
        } else {
            if self.connected_clients.contains(&user_id2_from_request) == true
            {
                println!("if self.connected_clients.contains(&user_id2_from_request) == true");
                if self.personal_chat_message_senders.contains_key(&user_id2_from_request) == true {
                    if self.connected_peer_to_peer.contains_key(&user_id2_from_request) == true {
                        let another_peer = self.connected_peer_to_peer.get(&user_id2_from_request).unwrap();
                        if another_peer == &user_id_from_request {
                            println!("if self.personal_chat_message_senders.contains_key(&user_id2_from_request) == true");
                            let tx_tmp_ref = self.personal_chat_message_senders.get(&user_id2_from_request).unwrap();
                            let mut tx_tmp = (*tx_tmp_ref).clone();
                            let reply = chatservice::NewMessageResponse {
                                response_code: 1,
                                message: message_from_request.clone()
                            };
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                            });
                        } else {
                            if self.pending_messages.contains_key(&user_id2_from_request) == true {
                                println!("if self.pending_messages.contains_key(&user_id2_from_request) == true");
                                let messages = self.pending_messages.get_mut(&user_id2_from_request);
                                if let Some(msgs) = messages {
                                    msgs.push_back(message_from_request);// fixme: insertion order is not kept
                                }
                            } else {
                                println!("else");
                                let mut messages:VecDeque<String> = VecDeque::new();
                                messages.push_back(message_from_request);
                                self.pending_messages.insert(user_id2_from_request, messages);// fixme: insertion order is not kept
                            }
                        }
                    }
                } else {
                    if self.pending_messages.contains_key(&user_id2_from_request) == true {
                        println!("if self.pending_messages.contains_key(&user_id2_from_request) == true");
                        let messages = self.pending_messages.get_mut(&user_id2_from_request);
                        if let Some(msgs) = messages {
                            msgs.push_back(message_from_request);// fixme: insertion order is not kept
                        }
                    } else {
                        println!("else");
                        let mut messages:VecDeque<String> = VecDeque::new();
                        messages.push_back(message_from_request);
                        self.pending_messages.insert(user_id2_from_request, messages);// fixme: insertion order is not kept
                    }
                }
            } else {
                if self.pending_messages.contains_key(&user_id2_from_request) == true {
                    println!("if self.pending_messages.contains_key(&user_id2_from_request) == true");
                    let messages = self.pending_messages.get_mut(&user_id2_from_request);
                    if let Some(msgs) = messages {
                        msgs.push_back(message_from_request);// fixme: insertion order is not kept
                    }
                } else {
                    println!("else");
                    let mut messages:VecDeque<String> = VecDeque::new();
                    messages.push_back(message_from_request);
                    self.pending_messages.insert(user_id2_from_request, messages);// fixme: insertion order is not kept
                }
            }
        }
        return Ok(Response::new(rx));
    }
    
    type NewCollectiveMessageStream=mpsc::Receiver<Result<NewCollectiveMessageResponse, Status>>;
    async fn new_collective_message(&mut self, request: Request<NewCollectiveMessageRequest>) -> 
    Result<Response<Self::NewCollectiveMessageStream>, Status>
    {
        println!("Got a new_collective_message request from {:?}", request.remote_addr());

        let (tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();
        let message_from_request = request.get_ref().message.clone();

        if message_from_request == "" {
            self.collective_chat_message_senders.entry(user_id_from_request.clone()).or_insert(tx.clone());
        } else {
            let peers = self.connected_peer_to_peers.get(&user_id_from_request).unwrap();
            for peer in peers {
                if self.collective_chat_message_senders.contains_key(peer) == true {
                    let tx_tmp_ref = self.collective_chat_message_senders.get(peer).unwrap();
                    let mut tx_tmp = (*tx_tmp_ref).clone();
                    let reply = chatservice::NewCollectiveMessageResponse {
                        response_code: 1,
                        message: message_from_request.clone()
                    };
                    tokio::spawn(async move {
                        tx_tmp.send(Ok(reply)).await;
                    });
                }
            }
        }
        return Ok(Response::new(rx));
    }

    type TypingMessageStream=mpsc::Receiver<Result<TypingMessageResponse, Status>>;
    async fn typing_message(&mut self,request: Request<TypingMessageRequest>) -> 
    Result<Response<Self::TypingMessageStream>, Status>
    {
        println!("Got a typing_message request from {:?}", request.remote_addr());

        let (tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();
        let user_id2_from_request = request.get_ref().to_user_id.clone();
        self.typing_message_senders.entry(user_id_from_request.clone()).or_insert(tx.clone());

        if &user_id2_from_request != "" {
            if self.connected_peer_to_peer.contains_key(&user_id2_from_request) {
                if let Some(another_peer) = self.connected_peer_to_peer.get(&user_id2_from_request) {
                    if another_peer == &user_id_from_request {
                        if self.typing_message_senders.contains_key(&user_id2_from_request) == true {
                            let tx_tmp_ref = self.typing_message_senders.get(&user_id2_from_request).unwrap();
                            let mut tx_tmp = (*tx_tmp_ref).clone();
                            let reply = chatservice::TypingMessageResponse {
                                response_code: 1,
                            };
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                                println!("sent a typing_messages ");
                            });
                        }
                    }
                }
            }
        } else {
            if self.typing_message_senders.len() > 1 {
                // send "typing message" to many users in the same radius
                let peers = self.connected_peer_to_peers.get(&user_id_from_request).unwrap();
                for peer in peers {
                    if self.typing_message_senders.contains_key(peer) == true {
                        let tx_tmp_ref = self.typing_message_senders.get(peer).unwrap();
                        let mut tx_tmp = (*tx_tmp_ref).clone();
                        let reply = chatservice::TypingMessageResponse {
                            response_code: 1,
                        };
                        tokio::spawn(async move {
                            tx_tmp.send(Ok(reply)).await;
                            //println!("sent a typing_messages ");
                        });
                    }
                }
            }
        }
        return Ok(Response::new(rx));
    }

    type ChatClosedStream = mpsc::Receiver<Result<ChatClosedResponse, Status>>;
    async fn chat_closed(&mut self, request: Request<ChatClosedRequest>) -> 
    Result<Response<Self::ChatClosedStream>, Status>
    {
        println!("Got a chat_closed request from {:?}", request.remote_addr());

        let (tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();
        let is_closed = request.get_ref().is_closed;
        let user_id2_from_request = request.get_ref().to_user_id.clone();

        // это нужно для того, чтобы сохранить stream, чтобы потом в этот stream отправить
        // инфу, что собеседник закрыл чат
        self.chat_closed_clients.entry(user_id_from_request.clone()).or_insert(tx.clone());

        if is_closed == true {
            if &user_id2_from_request != "" {
                if let Some(another_peer) = self.connected_peer_to_peer.get(&user_id2_from_request) {
                    if another_peer == &user_id_from_request {
                        if self.chat_closed_clients.contains_key(&user_id2_from_request) == true {
                            let tx_tmp_ref = self.chat_closed_clients.get(&user_id2_from_request).unwrap();
                            let mut tx_tmp = (*tx_tmp_ref).clone();
                            let reply = ChatClosedResponse{};
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                                println!("sent a chat_closed ");
                            });
                        }
                    }
                }
                self.personal_chat_message_senders.remove(&user_id_from_request);
                //self.personal_chat_message_senders.remove(&user_id2_from_request);
                self.typing_message_senders.remove(&user_id_from_request);
                //self.typing_message_senders.remove(&user_id2_from_request);
                self.chat_closed_clients.remove(&user_id_from_request);
                //self.chat_closed_clients.remove(&user_id2_from_request);
                self.connected_peer_to_peer.remove(&user_id_from_request);
                self.connected_peer_to_peer.remove(&user_id2_from_request);
            }
            /*self.connected_peer_to_peers.remove(&user_id_from_request);
            for (_, val) in &mut self.connected_peer_to_peers {
                val.remove(&user_id_from_request);
            }*/
        }

        return Ok(Response::new(rx));
    }

    type CollectiveChatClosedStream = mpsc::Receiver<Result<CollectiveChatClosedResponse, Status>>;
    async fn collective_chat_closed(&mut self, request: Request<CollectiveChatClosedRequest>) -> 
    Result<Response<Self::CollectiveChatClosedStream>, Status>
    {
        println!("Got a collective_chat_closed request from {:?}", request.remote_addr());

        let (tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();
        let is_closed = request.get_ref().is_closed;

        self.collective_chat_closed_clients.entry(user_id_from_request.clone()).or_insert(tx.clone());

        if is_closed == true {
            let peers = self.connected_peer_to_peers.get(&user_id_from_request).unwrap();
            for user_id2 in peers {
                if self.collective_chat_closed_clients.contains_key(user_id2) == true {
                    let tx_tmp_ref = self.collective_chat_closed_clients.get(user_id2).unwrap();
                    let mut tx_tmp = (*tx_tmp_ref).clone();
                    let reply = CollectiveChatClosedResponse{};
                    tokio::spawn(async move {
                        tx_tmp.send(Ok(reply)).await;
                        println!("sent a collective_chat_closed ");
                    });
                }
            }
            self.collective_chat_message_senders.remove(&user_id_from_request);
            //self.collective_chat_message_senders.remove(user_id2);
            self.typing_message_senders.remove(&user_id_from_request);
            //self.typing_message_senders.remove(user_id2);
            self.collective_chat_closed_clients.remove(&user_id_from_request);
            //self.collective_chat_closed_clients.remove(user_id2);
            /*self.connected_peer_to_peers.remove(&user_id_from_request);
            for (_, val) in &mut self.connected_peer_to_peers {
                val.remove(&user_id_from_request);
            }*/
        }
        return Ok(Response::new(rx));
    }

    async fn peer_closed(&mut self, request: Request<PeerClosedRequest>) -> 
    Result<Response<PeerClosedResponse>, Status>
    {
        println!("Got a peer_closed request from {:?}", request.remote_addr());
        let user_id_from_request = request.get_ref().user_id.clone();

        // удалить из списка подключённых клиентов
        self.connected_clients.remove(&user_id_from_request);

        // удалить из searching_peers user_id, если остался
        self.searching_peers.remove(&user_id_from_request);
        
        self.connected_peer_to_peer.retain(|key, val| {
            key != &user_id_from_request || val != &user_id_from_request
        });

        // удалить из connected_peers_to_peers по user_id, если есть
        self.connected_peer_to_peers.remove(&user_id_from_request);
        for (_, val) in &mut self.connected_peer_to_peers {
                val.remove(&user_id_from_request);
        };

        self.personal_chat_message_senders.remove(&user_id_from_request);
        self.collective_chat_message_senders.remove(&user_id_from_request);

        self.typing_message_senders.remove(&user_id_from_request);

        // удалить подключённого клиента
        self.chat_closed_clients.remove(&user_id_from_request);
        self.collective_chat_closed_clients.remove(&user_id_from_request);
        
        let reply = chatservice::PeerClosedResponse {
            response_code: 1
        };
        return Ok(Response::new(reply));
    }
}

// compute distance in meters between 2 geopoints
fn compute_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Based on http://www.ngs.noaa.gov/PUBS_LIB/inverse.pdf
    // using the "Inverse Formula" (section 4)
    use std::f64::consts::PI;
    let MAXITERS = 20;
    // Convert lat/long to radians
    let mut tmp_lat1 = lat1;
    tmp_lat1 *= PI / 180.0;
    let mut tmp_lat2 = lat2;
    tmp_lat2 *= PI / 180.0;
    let mut tmp_lon1 = lon1;
    tmp_lon1 *= PI / 180.0;
    let mut tmp_lon2 = lon2;
    tmp_lon2 *= PI / 180.0;

    let a:f64 = 6378137.0; // WGS84 major axis
    let b:f64 = 6356752.3142; // WGS84 semi-major axis
    let f:f64 = (a - b) / a;
    let a_sq_minus_b_sq_over_bsq = (a * a - b * b) / (b * b);

    let L:f64 = tmp_lon2 - tmp_lon1;
    let mut A:f64 = 0.0;
    let u1:f64 = ((1.0 - f) * tmp_lat1.tan()).atan();
    let u2:f64 = ((1.0 - f) * tmp_lat2.tan()).atan();

    let cos_u1:f64 = u1.cos();
    let cos_u2:f64 = u2.cos();
    let sin_u1:f64 = u1.sin();
    let sin_u2:f64 = u2.sin();
    let cos_u1_cosu2:f64 = cos_u1 * cos_u2;
    let sin_u1_sin_u2:f64 = sin_u1 * sin_u2;

    let mut sigma:f64 = 0.0;
    let mut delta_sigma:f64 = 0.0;
    let mut cos_sq_alpha:f64;
    let mut cos_2sm:f64;
    let mut cos_sigma:f64;
    let mut sin_sigma:f64;
    let mut cos_lambda:f64;
    let mut sin_lambda:f64;

    let mut lambda:f64 = L; // initial guess
    for _ in 0..MAXITERS {
        let lambda_orig = lambda;
        cos_lambda = lambda.cos();
        sin_lambda = lambda.sin();
        let t1:f64 = cos_u2 * sin_lambda;
        let t2:f64 = cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda;
        let sin_sq_sigma:f64 = t1 * t1 + t2 * t2; // (14)
        sin_sigma = sin_sq_sigma.sqrt();
        cos_sigma = sin_u1_sin_u2 + cos_u1_cosu2 * cos_lambda; // (15)
        sigma = sin_sigma.atan2(cos_sigma); // (16)
        let sin_alpha = if sin_sigma == 0.0 {0.0} else
            {cos_u1_cosu2 * sin_lambda / sin_sigma}; // (17)
        cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;
        if cos_sq_alpha == 0.0 {
            cos_2sm = 0.0
        } else {
            cos_2sm = cos_sigma - 2.0 * sin_u1_sin_u2 / cos_sq_alpha
        }; // (18)

        let u_squared = cos_sq_alpha * a_sq_minus_b_sq_over_bsq; // defn
        A = 1.0 + (u_squared / 16384.0) * // (3)
            (4096.0 + u_squared *
             (-768.0 + u_squared * (320.0 - 175.0 * u_squared)));
        let B = (u_squared / 1024.0) * // (4)
            (256.0 + u_squared *
             (-128.0 + u_squared * (74.0 - 47.0 * u_squared)));
        let c = (f / 16.0) *
            cos_sq_alpha *
            (4.0 + f * (4.0 - 3.0 * cos_sq_alpha)); // (10)
        let cos_2smsq = cos_2sm * cos_2sm;
        delta_sigma = B * sin_sigma * // (6)
            (cos_2sm + (B / 4.0) *
             (cos_sigma * (-1.0 + 2.0 * cos_2smsq) -
              (B / 6.0) * cos_2sm *
              (-3.0 + 4.0 * sin_sigma * sin_sigma) *
              (-3.0 + 4.0 * cos_2smsq)));

        lambda = L +
            (1.0 - c) * f * sin_alpha *
            (sigma + c * sin_sigma *
             (cos_2sm + c * cos_sigma *
              (-1.0 + 2.0 * cos_2sm * cos_2sm))); // (11)

        let delta = (lambda - lambda_orig) / lambda;
        if delta.abs() < 1.0e-12 {
            break;
        }
    }

    let distance = b * A * (sigma - delta_sigma);
    return distance;
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
