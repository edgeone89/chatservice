#![allow(non_snake_case)]
use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::UnboundedReceiver;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::mpsc::Sender as stdSender;
use std::sync::mpsc::Receiver as stdReceiver;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::stream::StreamExt;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::BufWriter;
use std::io::BufReader;
/*pub mod chatservice {
    tonic::include_proto!("chatservice");
}*/
const CHAT_SERVER_ADDRESS: &str = "192.168.0.100:50051";
const PUSH_NOTIFITCATION_SERVER_ADDRESS: &str = "http://192.168.0.100:50052";

mod chatservice;
use chatservice::chat_server::{Chat, ChatServer};
use chatservice::{NewPeerRequest, NewPeerResponse, SearchingPeerRequest, SearchingPeerResponse, 
    NewMessageRequest, NewMessageResponse, NewCollectiveMessageRequest,
    NewCollectiveMessageResponse, TypingMessageRequest, TypingMessageResponse, 
    ChatClosedRequest, ChatClosedResponse, CollectiveChatClosedRequest, 
    CollectiveChatClosedResponse, PeerClosedRequest, PeerClosedResponse,
    AdminStatusRequest, AdminStatusResponse, BlockUserInCollectiveChatRequest,
    BlockUserInCollectiveChatResponse, ClearCollectiveChatRequest, ClearCollectiveChatResponse,
    BlockUserInPersonalChatRequest, BlockUserInPersonalChatResponse,
    ClearPersonalChatRequest, ClearPersonalChatResponse,
    ReportUserRequest, ReportUserResponse,UploadImageRequest,UploadImageResponse,
    DownloadImageRequest, DownloadImageResponse
};

mod pushnotificationsservice;
use pushnotificationsservice::push_notifications_client::PushNotificationsClient;
use pushnotificationsservice::PushNotificationRequest;

struct ConnectedClient {
    is_admin_on: bool,
    //blocked_by_admin_ids_in_collective_chat: HashMap<String, UserBlockTime>,
    //blocked_in_personal_chats: HashMap<String, UserBlockTime>,
    stream_blocked_in_collective_chat: Option<Sender<Result<BlockUserInCollectiveChatResponse, Status>>>,
    stream_blocked_in_personal_chat: Option<Sender<Result<BlockUserInPersonalChatResponse, Status>>>,
    stream_clear_collective_chat: Option<Sender<Result<ClearCollectiveChatResponse, Status>>>,
    stream_clear_personal_chat: Option<Sender<Result<ClearPersonalChatResponse, Status>>>,
    collective_chat_closed_clients: Option<Sender<Result<CollectiveChatClosedResponse, Status>>>,
    chat_closed_clients: Option<Sender<Result<ChatClosedResponse, Status>>>,
    image_name: Option<String>
}

struct SearchingPeer {
    lat: f64,
    lng: f64,
    status: String,
    status_color_id: i32,
    visible_in_radius_in_meters: i32,
    user_name: String,
    description: String,
    tx: Sender<Result<SearchingPeerResponse, Status>>
}

struct ConnectedPeerToPeer {//1-to-1 relation for personal chat
    user_id1: String,
    user_id2: String,
}

enum UserBlockTime {
    OneHour,
    TreeHours,
    FiveHours,
    Always
}

#[derive(Default)]
struct HABChat {
    connected_clients: HashMap<String, ConnectedClient>,
    searching_peers: HashMap<String, SearchingPeer>,
    connected_peer_to_peer: HashMap<String, String>,
    connected_peer_to_peers: HashMap<String, HashSet<String>>,
    personal_chat_message_senders: HashMap<String, Sender<Result<NewMessageResponse, Status>>>,
    collective_chat_message_senders: HashMap<String, Sender<Result<NewCollectiveMessageResponse, Status>>>,
    typing_message_senders: HashMap<String, Sender<Result<TypingMessageResponse, Status>>>,
    //chat_closed_clients: HashMap<String, Sender<Result<ChatClosedResponse, Status>>>,
    //collective_chat_closed_clients: HashMap<String, Sender<Result<CollectiveChatClosedResponse, Status>>>,
} 

#[tonic::async_trait]
impl Chat for HABChat {

    async fn new_peer(&mut self, request: Request<NewPeerRequest>)-> Result<Response<NewPeerResponse>, Status>
    {
        println!("Got a new peer request from {:?}", request.remote_addr());

        let user_id_from_request = request.get_ref().user_id.clone();
        println!("Got user_id_from_request: {}", &user_id_from_request);

        let connected_client = ConnectedClient{
            is_admin_on: false,
            //blocked_by_admin_ids_in_collective_chat: HashMap::new(),
            //blocked_in_personal_chats: HashMap::new(),
            stream_blocked_in_collective_chat: Option::None,
            stream_blocked_in_personal_chat: Option::None,
            stream_clear_collective_chat: Option::None,
            stream_clear_personal_chat: Option::None,
            collective_chat_closed_clients: Option::None,
            chat_closed_clients: Option::None,
            image_name: Option::None
        };
        self.connected_clients.insert(user_id_from_request, connected_client);

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
        let user_name_from_request = request.get_ref().user_name.clone();
        let lat_from_request = request.get_ref().latitude;
        let lng_from_request = request.get_ref().longitude;
        let status_from_request = request.get_ref().status.clone();
        let status_color_id_from_request = request.get_ref().status_color_id;
        let visible_in_radius_in_meters_from_request = request.get_ref().visible_in_radius_in_meters;
        let description_from_request = request.get_ref().description.clone();
        let is_searching = request.get_ref().is_searching;

        println!("searching_peer: user_id={}",&user_id_from_request);
        println!("lat_from_request={}",lat_from_request);
        println!("lng_from_request={}",lng_from_request);
        
        if self.searching_peers.contains_key(&user_id_from_request) == true {
            let searching_peer = self.searching_peers.get_mut(&user_id_from_request).unwrap();
            searching_peer.lat = lat_from_request;
            searching_peer.lng = lng_from_request;
            searching_peer.status = status_from_request.clone();
            searching_peer.status_color_id = status_color_id_from_request;
            searching_peer.visible_in_radius_in_meters = visible_in_radius_in_meters_from_request;
            searching_peer.user_name = user_name_from_request.clone();
            searching_peer.description = description_from_request.clone()
        } else {
            let new_searching_peer = SearchingPeer{
                lat: lat_from_request,
                lng: lng_from_request,
                status: status_from_request.clone(),
                status_color_id: status_color_id_from_request,
                visible_in_radius_in_meters: visible_in_radius_in_meters_from_request,
                user_name: user_name_from_request.clone(),
                description: description_from_request.clone(),
                tx: tx.clone()
            };
            self.searching_peers.insert(user_id_from_request.clone(), new_searching_peer);
        }

        let mut is_found_peer = false;
        if is_searching == true {
            for (key, val) in &self.searching_peers {
                if &user_id_from_request != key {
                    let lat1 = lat_from_request;
                    let lon1 = lng_from_request;
                    let lat2 = (*val).lat;
                    let lon2 = (*val).lng;
                    let actual_distance_between_peers = compute_distance(lat1, lon1, lat2, lon2) as i32;
                    println!("actual_distance_between_peers={}",actual_distance_between_peers);
                    if actual_distance_between_peers <= (*val).visible_in_radius_in_meters
                    {
                        println!("if actual_distance_between_peers <= (*val).visible_in_radius_in_meters");
                        if self.connected_peer_to_peers.contains_key(&user_id_from_request) == true {
                            let peers = self.connected_peer_to_peers.get_mut(&user_id_from_request).unwrap();
                            peers.insert((*key).clone());
                        } else {
                            let mut peers = HashSet::new();
                            peers.insert((*key).clone());
                            self.connected_peer_to_peers.insert(user_id_from_request.clone(), peers);
                        }
    
                        let peer_id: String = (*key).clone();
                        let peer_radius_distance_in_meters = (*val).visible_in_radius_in_meters;
                        let peer_status = (*val).status.clone();
                        let peer_status_color_id = (*val).status_color_id;
                        let peer_user_name = (*val).user_name.clone();
                        let peer_description = (*val).description.clone();
                        let reply_to_peer1 = chatservice::SearchingPeerResponse {
                            response_code: 1,
                            user_id: peer_id,
                            radius_distance_in_meters: peer_radius_distance_in_meters,
                            status: peer_status,
                            status_color_id: peer_status_color_id,
                            user_name: peer_user_name,
                            description: peer_description
                        };
    
                        let mut tx_tmp = tx.clone();
                        tokio::spawn(async move {
                            // sending response to client
                            tx_tmp.send(Ok(reply_to_peer1)).await;
                            println!("searching_peer: sent response");
                        });
                        is_found_peer = true;
                    } else {
                        let mut tx_tmp = tx.clone();
                        tokio::spawn(async move {
                            for _ in 0i32..1 {
                                let reply = chatservice::SearchingPeerResponse {
                                    response_code: 2,
                                    user_id: "no_user_id".to_string(),
                                    radius_distance_in_meters: -1,
                                    status: "".to_string(),
                                    status_color_id: -1,
                                    user_name: "".to_string(),
                                    description: "".to_string()
                                };
                                tx_tmp.send(Ok(reply)).await;
                            }
                        });
                    }
                    if actual_distance_between_peers <= visible_in_radius_in_meters_from_request
                    {
                        println!("if actual_distance_between_peers <= visible_in_radius_in_meters");
                        
                        if self.connected_peer_to_peers.contains_key(key) == true {
                            let peers = self.connected_peer_to_peers.get_mut(key).unwrap();
                            peers.insert(user_id_from_request.clone());
                        } else {
                            let mut peers = HashSet::new();
                            peers.insert(user_id_from_request.clone());
                            self.connected_peer_to_peers.insert((*key).clone(), peers);
                        }
                        let peer2_id: String = user_id_from_request.clone();
                        let peer2_radius_distance_in_meters = visible_in_radius_in_meters_from_request;
                        let peer2_status = status_from_request.clone();
                        let peer2_status_color_id = status_color_id_from_request;
                        let peer2_user_name = user_name_from_request.clone();
                        let peer2_description = description_from_request.clone();
                        let reply_to_peer2 = chatservice::SearchingPeerResponse {
                            response_code: 1,
                            user_id: peer2_id,
                            radius_distance_in_meters: peer2_radius_distance_in_meters,
                            status: peer2_status,
                            status_color_id: peer2_status_color_id,
                            user_name: peer2_user_name,
                            description: peer2_description
                        };
    
                        let mut tx_tmp = (*val).tx.clone();
                        tokio::spawn(async move {
                            // sending response to client
                            tx_tmp.send(Ok(reply_to_peer2)).await;
                            println!("searching_peer: sent response");
                        });
                    } else {
                        let mut tx_tmp = (*val).tx.clone();
                        tokio::spawn(async move {
                            for _ in 0i32..1 {
                                let reply = chatservice::SearchingPeerResponse {
                                    response_code: 2,
                                    user_id: "no_user_id".to_string(),
                                    radius_distance_in_meters: -1,
                                    status: "".to_string(),
                                    status_color_id: -1,
                                    user_name: "".to_string(),
                                    description: "".to_string()
                                };
                                tx_tmp.send(Ok(reply)).await;
                            }
                        });
                    }
                }
            }
        } else {

        }

        if is_found_peer == true {
            // remove those searching peers who found each other
            /*self.searching_peers.remove(&user_id_from_request);
            self.searching_peers.remove(&found_peer_id);
            self.searching_peers.retain(|key, val|{
                key != &user_id_from_request && radius_distance_in_meters_from_request == (*val).radius_distance_in_meters
            });*/
        } else {
            println!("is_found_peer == false");
            tokio::spawn(async move {
                for _ in 0i32..1 {
                    let reply = chatservice::SearchingPeerResponse {
                        response_code: 2,
                        user_id: "no_user_id".to_string(),
                        radius_distance_in_meters: -1,
                        status: "".to_string(),
                        status_color_id: -1,
                        user_name: "".to_string(),
                        description: "".to_string()
                    };
                    tx.send(Ok(reply)).await;
                }
            });
        }
        
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
        } else {
            if self.connected_clients.contains_key(&user_id2_from_request) == true
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
                            let mut client = PushNotificationsClient::connect(PUSH_NOTIFITCATION_SERVER_ADDRESS).await.unwrap();
                            let request = Request::new(PushNotificationRequest{
                                user_id: user_id_from_request,
                                message: message_from_request,
                                to_user_id: user_id2_from_request
                            });
                            client.send_push_notification(request).await;
                        }
                    }
                } else {
                    let mut client = PushNotificationsClient::connect(PUSH_NOTIFITCATION_SERVER_ADDRESS).await.unwrap();
                    let request = Request::new(PushNotificationRequest{
                        user_id: user_id_from_request,
                        message: message_from_request,
                        to_user_id: user_id2_from_request
                    });
                    client.send_push_notification(request).await;
                }
            } else {
                let mut client = PushNotificationsClient::connect(PUSH_NOTIFITCATION_SERVER_ADDRESS).await.unwrap();
                let request = Request::new(PushNotificationRequest{
                    user_id: user_id_from_request,
                    message: message_from_request,
                    to_user_id: user_id2_from_request
                });
                client.send_push_notification(request).await;
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
        let admin_id_from_request = request.get_ref().admin_id.clone();
        let user_name_from_request = request.get_ref().user_name.clone();
        let is_admin_from_request = request.get_ref().is_admin;

        if user_id_from_request != "" {
            self.collective_chat_message_senders.entry(user_id_from_request.clone()).or_insert(tx.clone());
        }
        if admin_id_from_request != "" {
            self.collective_chat_message_senders.entry(admin_id_from_request.clone()).or_insert(tx.clone());
        }

        if message_from_request == "" {
            if is_admin_from_request == true {
                if admin_id_from_request != "" {
                    if self.connected_clients.contains_key(&admin_id_from_request) == true {
                        let connected_client = self.connected_clients.get(&admin_id_from_request).unwrap();
                        let tx_tmp_ref = self.collective_chat_message_senders.get(&admin_id_from_request).unwrap();
                        let mut tx_tmp = (*tx_tmp_ref).clone();
                        if connected_client.is_admin_on == false {
                            let reply = chatservice::NewCollectiveMessageResponse {
                                response_code: -1,
                                message: "".to_string(),
                                user_name: "".to_string(),
                                user_id: "".to_string()
                            };
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                            });
                        } else {
                            let reply = chatservice::NewCollectiveMessageResponse {
                                response_code: 1,
                                message: "".to_string(),
                                user_name: user_name_from_request.clone(),
                                user_id: user_id_from_request.clone()
                            };
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                            });
                        }
                    }
                }
            } else {
                if user_id_from_request != "" {
                    if self.connected_clients.contains_key(&user_id_from_request) == true {
                        let connected_client = self.connected_clients.get(&admin_id_from_request).unwrap();
                        let tx_tmp_ref = self.collective_chat_message_senders.get(&user_id_from_request).unwrap();
                        let mut tx_tmp = (*tx_tmp_ref).clone();
                        if connected_client.is_admin_on == false {
                            let reply = chatservice::NewCollectiveMessageResponse {
                                response_code: -1,
                                message: "".to_string(),
                                user_name: "".to_string(),
                                user_id: "".to_string()
                            };
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                            });
                        } else {
                            let reply = chatservice::NewCollectiveMessageResponse {
                                response_code: 1,
                                message: "".to_string(),
                                user_name: user_name_from_request.clone(),
                                user_id: user_id_from_request.clone()
                            };
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                            });
                        }
                    }
                }
            }
        } else {
            if is_admin_from_request == true {
                if admin_id_from_request != "" {
                    if self.connected_clients.contains_key(&admin_id_from_request) == true {
                        let connected_client = self.connected_clients.get(&admin_id_from_request).unwrap();
                        if connected_client.is_admin_on == true {
                            let peers = self.connected_peer_to_peers.get(&admin_id_from_request).unwrap();
                            for peer in peers {
                                if self.collective_chat_message_senders.contains_key(peer) == true {
                                    let tx_tmp_ref = self.collective_chat_message_senders.get(peer).unwrap();
                                    let mut tx_tmp = (*tx_tmp_ref).clone();
                                    let reply = chatservice::NewCollectiveMessageResponse {
                                        response_code: 1,
                                        message: message_from_request.clone(),
                                        user_name: user_name_from_request.clone(),
                                        user_id: admin_id_from_request.clone()
                                    };
                                    tokio::spawn(async move {
                                        tx_tmp.send(Ok(reply)).await;
                                    });
                                }
                            }
                        } else {
                            let tx_tmp_ref = self.collective_chat_message_senders.get(&user_id_from_request).unwrap();
                            let mut tx_tmp = (*tx_tmp_ref).clone();
                            let reply = chatservice::NewCollectiveMessageResponse {
                                response_code: -1,
                                message: "".to_string(),
                                user_name: "".to_string(),
                                user_id: "".to_string()
                            };
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                            });
                        }
                    }
                }
            } else {
                // todo: сам себе сообщение отправляет вместо того, чтобы отправить собеседнику
                let peers = self.connected_peer_to_peers.get(&user_id_from_request).unwrap();
                println!("self.connected_peer_to_peers.get(&user_id_from_request).unwrap()");
                for peer in peers {
                    if self.collective_chat_message_senders.contains_key(peer) == true {
                        let tx_tmp_ref = self.collective_chat_message_senders.get(peer).unwrap();
                        let mut tx_tmp = (*tx_tmp_ref).clone();
                        let reply = chatservice::NewCollectiveMessageResponse {
                            response_code: 1,
                            message: message_from_request.clone(),
                            user_name: user_name_from_request.clone(),
                            user_id: user_id_from_request.clone()
                        };
                        tokio::spawn(async move {
                            tx_tmp.send(Ok(reply)).await;
                            println!("sent a new_collective_message ");
                        });
                    }
                }
            }
        }


        /*if message_from_request == "" {
            if user_id_from_request == "" {
                if admin_id_from_request != "" {
                    if self.connected_clients.contains_key(&admin_id_from_request) == true {
                        let connected_client = self.connected_clients.get(&admin_id_from_request).unwrap();
                        let tx_tmp_ref = self.collective_chat_message_senders.get(&admin_id_from_request).unwrap();
                        let mut tx_tmp = (*tx_tmp_ref).clone();
                        if connected_client.is_admin_on == false {
                            let reply = chatservice::NewCollectiveMessageResponse {
                                response_code: -1,
                                message: "".to_string(),
                                user_name: "".to_string(),
                                user_id: "".to_string()
                            };
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                            });
                        } else {
                            let reply = chatservice::NewCollectiveMessageResponse {
                                response_code: 1,
                                message: "".to_string(),
                                user_name: user_name_from_request.clone(),
                                user_id: user_id_from_request
                            };
                            tokio::spawn(async move {
                                tx_tmp.send(Ok(reply)).await;
                            });
                        }
                    }
                }
            }
        } else {
            if admin_id_from_request != "" {
                if self.connected_clients.contains_key(&admin_id_from_request) == true {
                    let connected_client = self.connected_clients.get(&admin_id_from_request).unwrap();
                    if connected_client.is_admin_on == true {
                        let peers = self.connected_peer_to_peers.get(&user_id_from_request).unwrap();
                        for peer in peers {
                            if self.collective_chat_message_senders.contains_key(peer) == true {
                                let tx_tmp_ref = self.collective_chat_message_senders.get(peer).unwrap();
                                let mut tx_tmp = (*tx_tmp_ref).clone();
                                let reply = chatservice::NewCollectiveMessageResponse {
                                    response_code: 1,
                                    message: message_from_request.clone(),
                                    user_name: user_name_from_request.clone(),
                                    user_id: user_id_from_request.clone()
                                };
                                tokio::spawn(async move {
                                    tx_tmp.send(Ok(reply)).await;
                                });
                            }
                        }
                    } else {
                        let tx_tmp_ref = self.collective_chat_message_senders.get(&user_id_from_request).unwrap();
                        let mut tx_tmp = (*tx_tmp_ref).clone();
                        let reply = chatservice::NewCollectiveMessageResponse {
                            response_code: -1,
                            message: "".to_string(),
                            user_name: "".to_string(),
                            user_id: "".to_string()
                        };
                        tokio::spawn(async move {
                            tx_tmp.send(Ok(reply)).await;
                        });
                    }
                }
            } else {
                let peers = self.connected_peer_to_peers.get(&user_id_from_request).unwrap();
                for peer in peers {
                    if self.collective_chat_message_senders.contains_key(peer) == true {
                        let tx_tmp_ref = self.collective_chat_message_senders.get(peer).unwrap();
                        let mut tx_tmp = (*tx_tmp_ref).clone();
                        let reply = chatservice::NewCollectiveMessageResponse {
                            response_code: 1,
                            message: message_from_request.clone(),
                            user_name: user_name_from_request.clone(),
                            user_id: user_id_from_request.clone()
                        };
                        tokio::spawn(async move {
                            tx_tmp.send(Ok(reply)).await;
                        });
                    }
                }
            }
        }*/
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
        //self.chat_closed_clients.entry(user_id_from_request.clone()).or_insert(tx.clone());
        if let Some(connected_client) = self.connected_clients.get_mut(&user_id_from_request) {
            if connected_client.chat_closed_clients.is_none() {
                connected_client.chat_closed_clients = Some(tx.clone());
            }
        }

        if is_closed == true {
            if &user_id2_from_request != "" {
                if let Some(another_peer) = self.connected_peer_to_peer.get(&user_id2_from_request) {
                    if another_peer == &user_id_from_request {
                        //if self.chat_closed_clients.contains_key(&user_id2_from_request) == true {
                        if self.connected_clients.contains_key(&user_id2_from_request) == true {
                            //let tx_tmp_ref = self.chat_closed_clients.get(&user_id2_from_request).unwrap();
                            //let mut tx_tmp = (*tx_tmp_ref).clone();
                            let connected_client = self.connected_clients.get(&user_id2_from_request).unwrap();
                            if let Some(mut tx_tmp) = connected_client.chat_closed_clients.clone() {
                                let reply = ChatClosedResponse{};
                                tokio::spawn(async move {
                                    tx_tmp.send(Ok(reply)).await;
                                    println!("sent a chat_closed ");
                                });
                            }
                        }
                    }
                }
                self.personal_chat_message_senders.remove(&user_id_from_request);
                //self.personal_chat_message_senders.remove(&user_id2_from_request);
                self.typing_message_senders.remove(&user_id_from_request);
                //self.typing_message_senders.remove(&user_id2_from_request);
                //self.chat_closed_clients.remove(&user_id_from_request);
                if let Some(connected_client) = self.connected_clients.get_mut(&user_id_from_request) {
                    connected_client.chat_closed_clients = None;
                }

                //self.chat_closed_clients.remove(&user_id2_from_request);
                self.connected_peer_to_peer.remove(&user_id_from_request);
                self.connected_peer_to_peer.remove(&user_id2_from_request);
                if let Some(mut connected_client) = self.connected_clients.get_mut(&user_id_from_request) {
                    connected_client.stream_clear_personal_chat = None;
                }
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

        //self.collective_chat_closed_clients.entry(user_id_from_request.clone()).or_insert(tx.clone());
        let connected_client = self.connected_clients.get_mut(&user_id_from_request).unwrap();
        if connected_client.collective_chat_closed_clients.is_none() {
            connected_client.collective_chat_closed_clients = Some(tx.clone());
        }

        if is_closed == true {
            let peers = self.connected_peer_to_peers.get(&user_id_from_request).unwrap();
            for user_id2 in peers {
                //if self.collective_chat_closed_clients.contains_key(user_id2) == true {
                  if self.connected_clients.contains_key(user_id2) == true {
                      let connected_client = self.connected_clients.get(user_id2).unwrap();
                    //let tx_tmp_ref = self.collective_chat_closed_clients.get(user_id2).unwrap();
                    //let mut tx_tmp = (*tx_tmp_ref).clone();
                    if let Some(mut tx_tmp) = connected_client.collective_chat_closed_clients.clone() {
                        let reply = CollectiveChatClosedResponse{};
                        tokio::spawn(async move {
                            tx_tmp.send(Ok(reply)).await;
                            println!("sent a collective_chat_closed ");
                        });
                    }
                }
            }
            self.collective_chat_message_senders.remove(&user_id_from_request);
            //self.collective_chat_message_senders.remove(user_id2);
            self.typing_message_senders.remove(&user_id_from_request);
            //self.typing_message_senders.remove(user_id2);
            
            //self.collective_chat_closed_clients.remove(&user_id_from_request);
            let connected_client = self.connected_clients.get_mut(&user_id_from_request).unwrap();
            connected_client.collective_chat_closed_clients = Option::None;
            connected_client.stream_clear_collective_chat = Option::None;
            
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
        //self.chat_closed_clients.remove(&user_id_from_request);
        //self.collective_chat_closed_clients.remove(&user_id_from_request);
        if user_id_from_request != "" {
            if let Some(mut connected_client) = self.connected_clients.get_mut(&user_id_from_request) {
                connected_client.collective_chat_closed_clients = Option::None;
                connected_client.chat_closed_clients = Option::None;
            }
        }
        
        let reply = chatservice::PeerClosedResponse {
            response_code: 1
        };
        return Ok(Response::new(reply));
    }

    async fn admin_status(
        &mut self,
        request: Request<AdminStatusRequest>,
    ) -> Result<Response<AdminStatusResponse>, Status> {
        println!("Got a admin_status request from {:?}", request.remote_addr());
        let user_id_from_request = &request.get_ref().user_id;
        let is_admin_on = request.get_ref().is_admin_on;
        println!("Got a admin_status={:?}", is_admin_on);

        if let Some(connected_client) = self.connected_clients.get_mut(user_id_from_request) {
            (*connected_client).is_admin_on = is_admin_on;
        }

        let reply = chatservice::AdminStatusResponse {
            response_code: 1
        };
        return Ok(Response::new(reply));
    }

    type BlockUserInCollectiveChatStream = mpsc::Receiver<Result<BlockUserInCollectiveChatResponse, Status>>;
    async fn block_user_in_collective_chat(
        &mut self,
        request: Request<BlockUserInCollectiveChatRequest>,
    ) -> Result<Response<Self::BlockUserInCollectiveChatStream>, Status> {
        println!("Got a block_user_in_collective_chat request from {:?}", request.remote_addr());
        let (tx, rx) = mpsc::channel(4);
        let admin_id_from_request = request.get_ref().admin_id.clone();
        let blocked_user_id_from_request = request.get_ref().blocked_user_id.clone();
        let blocking_time_from_request = request.get_ref().blocking_time.clone();

        if admin_id_from_request == "" {
            if blocked_user_id_from_request != "" {
                let blocked_connected_client = self.connected_clients.get_mut(&blocked_user_id_from_request).unwrap();
                if blocked_connected_client.stream_blocked_in_collective_chat.is_none() {
                    blocked_connected_client.stream_blocked_in_collective_chat = Some(tx.clone());
                }
            }
        } else {
            if blocked_user_id_from_request != "" {
                let reply = BlockUserInCollectiveChatResponse {
                    response_code: 1,
                    admin_id: admin_id_from_request,
                    blocking_time: blocking_time_from_request
                };
                let blocked_connected_client = self.connected_clients.get_mut(&blocked_user_id_from_request).unwrap();
                let stream_blocked_in_collective_chat = blocked_connected_client.stream_blocked_in_collective_chat.clone();
                if let Some(mut tx_tmp) = stream_blocked_in_collective_chat {
                    tokio::spawn(async move {
                        tx_tmp.send(Ok(reply)).await;
                        println!("sent a block_user_in_collective_chat");
                    });
                }
            }
        }
        
        return Ok(Response::new(rx));
    }

    type ClearCollectiveChatStream = mpsc::Receiver<Result<ClearCollectiveChatResponse, Status>>;
    async fn clear_collective_chat(
        &mut self,
        request: Request<ClearCollectiveChatRequest>,
    ) -> Result<Response<Self::ClearCollectiveChatStream>, Status>{
        println!("Got a clear_collective_chat request from {:?}", request.remote_addr());
        let (tx, rx) = mpsc::channel(4);
        let admin_id_from_request = request.get_ref().admin_id.clone();
        let clear_chat_from_request = request.get_ref().clear_chat.clone();
        let user_id_from_request = request.get_ref().user_id.clone();

        if admin_id_from_request == "" {
            if clear_chat_from_request == false {
                if let Some(mut connected_client) = self.connected_clients.get_mut(&user_id_from_request) {
                    if connected_client.stream_clear_collective_chat.is_none() {
                        connected_client.stream_clear_collective_chat = Some(tx.clone());
                    }
                }
            }
        } else {
            if clear_chat_from_request == true {
                let peers = self.connected_peer_to_peers.get(&admin_id_from_request).unwrap();
                for peer in peers {
                    let connected_client = self.connected_clients.get_mut(peer).unwrap();
                    if let Some(mut tx_tmp) = connected_client.stream_clear_collective_chat.clone() {
                        let reply = ClearCollectiveChatResponse {
                        };
                        tokio::spawn(async move {
                            tx_tmp.send(Ok(reply)).await;
                            println!("sent a clear_collective_chat");
                        });
                    }
                }
            }
        }

        return Ok(Response::new(rx));
    }

    type BlockUserInPersonalChatStream = mpsc::Receiver<Result<BlockUserInPersonalChatResponse, Status>>;
    async fn block_user_in_personal_chat(
        &mut self,
        request: Request<BlockUserInPersonalChatRequest>,
    ) -> Result<Response<Self::BlockUserInPersonalChatStream>, Status> {
        println!("Got a block_user_in_personal_chat request from {:?}", request.remote_addr());
        let (tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();
        let blocked_user_id_from_request = request.get_ref().blocked_user_id.clone();
        let blocking_time_from_request = request.get_ref().blocking_time.clone();

        let connected_client = self.connected_clients.get_mut(&user_id_from_request).unwrap();
        if blocked_user_id_from_request == "" {
            println!("blocked_user_id_from_request == \"\"");
            if connected_client.stream_blocked_in_personal_chat.is_none() {
                println!("if connected_client.stream_blocked_in_personal_chat.is_none");
                connected_client.stream_blocked_in_personal_chat = Some(tx.clone());
            }
        }else {
            println!("blocked_user_id_from_request != \"\"");
            if blocking_time_from_request != "" {
                println!("blocking_time_from_request != \"\"");
                let blocked_connected_client = self.connected_clients.get_mut(&blocked_user_id_from_request).unwrap();
                if let Some(mut tx_tmp) = blocked_connected_client.stream_blocked_in_personal_chat.clone() {
                    println!("if let Some(mut tx_tmp) = blocked_connected_client.stream_blocked_in_personal_chat.clone()");
                    let reply = BlockUserInPersonalChatResponse{
                        response_code: 1,
                        blocking_time: blocking_time_from_request,
                        user_id: user_id_from_request
                    };
                    tokio::spawn(async move {
                        tx_tmp.send(Ok(reply)).await;
                        println!("sent a block_user_in_personal_chat");
                    });
                }
            }
        }
        return Ok(Response::new(rx));
    }

    type ClearPersonalChatStream = mpsc::Receiver<Result<ClearPersonalChatResponse, Status>>;
    async fn clear_personal_chat(
        &mut self,
        request: Request<ClearPersonalChatRequest>,
    ) -> Result<Response<Self::ClearPersonalChatStream>, Status>{
        println!("Got a clear_personal_chat request from {:?}", request.remote_addr());
        let (mut tx, rx) = mpsc::channel(4);
        let user_id_from_request = request.get_ref().user_id.clone();
        println!("user_id_from_request: {}", &user_id_from_request);
        let admin_id_from_request = request.get_ref().admin_id.clone();
        println!("admin_id_from_request: {}", &admin_id_from_request);
        let clear_chat_from_request = request.get_ref().clear_chat;
        println!("clear_chat_from_request: {}", &clear_chat_from_request);

        if let Some(mut connected_client) = self.connected_clients.get_mut(&admin_id_from_request) {
            if clear_chat_from_request == false {
                if connected_client.stream_clear_personal_chat.is_none() {
                    println!("if connected_client.stream_clear_personal_chat.is_none()");
                    connected_client.stream_clear_personal_chat = Some(tx.clone());
                    let reply = ClearPersonalChatResponse{
                        response_code: -1
                    };
                    tokio::spawn(async move {
                        let res = tx.send(Ok(reply)).await;
                        match res {
                            Ok(_) =>println!("sent a clear_personal_chat"),
                            Err(_) =>println!(" clear_personal_chat ERROR")
                        }
                    });
                }
            } else {
                let another_connected_client = self.connected_clients.get_mut(&user_id_from_request).unwrap();
                if let Some(mut tx_tmp) = another_connected_client.stream_clear_personal_chat.clone() {
                    println!("another_connected_client.stream_clear_personal_chat.clone()");
                    let reply = ClearPersonalChatResponse{
                        response_code: 1
                    };
                    tokio::spawn(async move {
                        let res = tx_tmp.send(Ok(reply)).await;
                        match res {
                            Ok(_) =>println!("sent a clear_personal_chat"),
                            Err(e) =>println!(" clear_personal_chat ERROR: {}", e)
                        }
                    });
                }
            }
        }
        return Ok(Response::new(rx));
    }

    async fn report_user(
        &self,
        request: Request<ReportUserRequest>,
    ) -> Result<Response<ReportUserResponse>, Status> {
        let user_id_from_request = request.get_ref().user_id.clone();
        let reported_user_id_from_request = request.get_ref().reported_user_id.clone();
        let report_user_from_request = request.get_ref().report_user;
        let message_from_request = request.get_ref().message.clone();
        
        // todo: where send report?
        return Ok(Response::new(ReportUserResponse{}));
    }
    async fn upload_image(
        &mut self,
        request: Request<tonic::Streaming<UploadImageRequest>>,
    ) -> Result<Response<UploadImageResponse>, tonic::Status>
    {
        println!("upload_image request");
        let mut stream = request.into_inner();

        if let Some(uploadImageRequestResult) = stream.next().await {
            if let Ok(uploadImageRequest) = uploadImageRequestResult {
                let user_id_from_request = uploadImageRequest.user_id;
                println!("upload_image: user_id_from_request={}",&user_id_from_request);
                let file_name_from_request = uploadImageRequest.image_name;
                println!("upload_image: file_name_from_request={}",&file_name_from_request);
                // check if file with same file_name not exist
                if let Err(_) = File::open(&file_name_from_request) {
                    let file = File::create(&file_name_from_request)?;
                    let mut buf_writer = BufWriter::new(file);
                    if let Some(connected_client) = self.connected_clients.get_mut(&user_id_from_request) {
                        connected_client.image_name = Some(file_name_from_request);
                    }
                    let file_chunk = uploadImageRequest.file_chunk;
                    let res = buf_writer.write_all(file_chunk.as_slice());
            
                    while let Some(uploadImageRequestResult) = stream.next().await {
                        if let Ok(uploadImageRequest) = uploadImageRequestResult {
                            let file_chunk = uploadImageRequest.file_chunk;
                            let res = buf_writer.write_all(file_chunk.as_slice());
                        }
                    }
                    buf_writer.flush().unwrap();
                    println!("upload_image: upload success");
                } else {
                    println!("upload_image: no error, open file");
                }
            }
        }

        let reply = UploadImageResponse{
        };
        return Ok(Response::new(reply));
    }

    type DownloadImageStream= mpsc::Receiver<Result<DownloadImageResponse, tonic::Status>>;
    async fn download_image(
        &self,
        request: tonic::Request<DownloadImageRequest>,
    ) -> Result<tonic::Response<Self::DownloadImageStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();

        if let Some(connected_client) = self.connected_clients.get(&user_id_from_request) {
            if let Some(image_name) = &connected_client.image_name {
                if let Ok(file) = File::open(image_name) {
                    let mut buf_reader = BufReader::new(file);
                    let mut buf: Vec<u8> = vec![0;1024];

                    if let Ok(mut res) = buf_reader.read(buf.as_mut()) {
                        while res > 0 {
                            let reply = DownloadImageResponse {
                                response_code: 1,
                                file_chunk: buf.clone()
                            };
                            let mut tx_tmp = tx.clone();
                            tokio::spawn(async move {
                                let res = tx_tmp.send(Ok(reply)).await;
                                match res {
                                    Ok(_) =>println!("download_image: sent a download_image"),
                                    Err(e) =>println!(" download_image ERROR: {}", e)
                                }
                            });
                            if let Ok(n) = buf_reader.read(buf.as_mut()) {
                                res = n;
                            } else {
                                let reply = DownloadImageResponse {
                                    response_code: -1,
                                    file_chunk: vec![]
                                };
                                let mut tx_tmp = tx.clone();
                                tokio::spawn(async move {
                                    let res = tx_tmp.send(Ok(reply)).await;
                                    match res {
                                        Ok(_) =>println!("download_image: sent a download_image"),
                                        Err(e) =>println!(" download_image ERROR: {}", e)
                                    }
                                });
                            }
                        }
                        if res == 0 {
                            let reply = DownloadImageResponse {
                                response_code: 2,
                                file_chunk: buf.clone()
                            };
                            let mut tx_tmp = tx.clone();
                            tokio::spawn(async move {
                                let res = tx_tmp.send(Ok(reply)).await;
                                match res {
                                    Ok(_) =>println!("download_image: sent a download_image"),
                                    Err(e) =>println!(" download_image ERROR: {}", e)
                                }
                            });
                        }
                    } else {
                        let reply = DownloadImageResponse {
                            response_code: -1,
                            file_chunk: vec![]
                        };
                        let mut tx_tmp = tx.clone();
                        tokio::spawn(async move {
                            let res = tx_tmp.send(Ok(reply)).await;
                            match res {
                                Ok(_) =>println!("download_image: sent a download_image"),
                                Err(e) =>println!(" download_image ERROR: {}", e)
                            }
                        });
                    }
                } else {
                    println!("download_image: error while opening file");
                }
            } else {
                println!("download_image: no image_name");
            }
        } else {
            println!("download_image: no connected_client");
        }

        return Ok(Response::new(rx));
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
    let addr = CHAT_SERVER_ADDRESS.parse()?;
    let chat = HABChat::default();
    //chat.searching_peers = HashMap::new();
    //chat.connected_peers_to_peers = Vec::new();
    //chat.connected_clients = HashMap::new();
    //chat.pending_messages = HashMap::new();

    println!("ChatServer listening on {}", addr);

    Server::builder()
        .add_service(ChatServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}
