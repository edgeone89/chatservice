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
    NewMessageRequest, NewMessageResponse, PeerClosedRequest, PeerClosedResponse, NewCollectiveMessageRequest,
    NewCollectiveMessageResponse, TypingMessageRequest, TypingMessageResponse};

struct ConnectedClient {
    user_id: String,
    tx: Sender<Result<NewMessageResponse, Status>>
}

struct SearchingPeer {
    radius_distance_in_meters: i32,
    status: String,
    tx: Sender<Result<SearchingPeerResponse, Status>>
}

struct ConnectedPeerToPeer {//1-to-1 relation for personal chat
    user_id1: String,
    user_id2: String,
}

//todo: need 1-to-many relation for collective chat in specific radius

#[derive(Default)]
pub struct MyChat {
    searching_peers: HashMap<String, SearchingPeer>,//Vec<SearchingPeer>,
    connected_peers_to_peers: Vec<ConnectedPeerToPeer>,//todo: convert Vect to HashMap
    connected_clients: HashMap<String, Sender<Result<NewMessageResponse, Status>>>,
    typing_messages: HashMap<String, Sender<Result<TypingMessageResponse, Status>>>
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
                    let new_connected_peer_to_peer = ConnectedPeerToPeer{
                        user_id1: user_id_from_request.clone(),
                        user_id2: (*key).clone()
                    };
                    self.connected_peers_to_peers.push(new_connected_peer_to_peer);

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
            self.searching_peers.remove(&user_id_from_request);
            self.searching_peers.remove(&found_peer_id);
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
    
    type NewCollectiveMessageStream=mpsc::Receiver<Result<NewCollectiveMessageResponse, Status>>;
    async fn new_collective_message(
        &mut self,
        request: Request<NewCollectiveMessageRequest>,
    ) -> Result<Response<Self::NewCollectiveMessageStream>, Status>{
        println!("Got a new_collective_message request from {:?}", request.remote_addr());

        let (mut tx, rx) = mpsc::channel(4);

        //todo: send message to many users in the same radius
        return Ok(Response::new(rx));
    }

    type TypingMessageStream=mpsc::Receiver<Result<TypingMessageResponse, Status>>;
    async fn typing_message(
        &mut self,
        request: Request<TypingMessageRequest>,
    ) -> Result<Response<Self::TypingMessageStream>, Status>{
        println!("Got a typing_message request from {:?}", request.remote_addr());

        let (mut tx, rx) = mpsc::channel(4);

        let user_id_from_request = request.get_ref().user_id.clone();
        self.typing_messages.entry(user_id_from_request.clone()).or_insert(tx.clone());

        if self.typing_messages.len() > 1 {
            // send "typing message" to many users in the same radius
            for connected_peer_to_peer in &self.connected_peers_to_peers {
                if connected_peer_to_peer.user_id1 == user_id_from_request {
                    let user_id2 = &(connected_peer_to_peer.user_id2);
                    let tx_tmp_ref = self.typing_messages.get(user_id2).unwrap();
                    let mut tx_tmp = (*tx_tmp_ref).clone();
                    let reply = chatservice::TypingMessageResponse {
                        response_code: 1,
                    };
                    tokio::spawn(async move {
                        tx_tmp.send(Ok(reply)).await;
                        println!("sent a typing_messages ");
                    });
                    break;
                } else if connected_peer_to_peer.user_id2 == user_id_from_request {
                    let user_id1 = &(connected_peer_to_peer.user_id1);
                    let tx_tmp_ref = self.typing_messages.get(user_id1).unwrap();
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

        // удалить подключённого клиента
        self.connected_clients.remove(&user_id_from_request);
        
        let reply = chatservice::PeerClosedResponse {
            response_code: 1
        };
        return Ok(Response::new(reply));
    }
}

// compute distance in meters between 2 geopoints
fn compute_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64)-> f64 {
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
    let aSqMinusBSqOverBSq = (a * a - b * b) / (b * b);

    let L:f64 = tmp_lon2 - tmp_lon1;
    let mut A:f64 = 0.0;
    let U1:f64 = ((1.0 - f) * tmp_lat1.tan()).atan();
    let U2:f64 = ((1.0 - f) * tmp_lat2.tan()).atan();

    let cosU1:f64 = U1.cos();
    let cosU2:f64 = U2.cos();
    let sinU1:f64 = U1.sin();
    let sinU2:f64 = U2.sin();
    let cosU1cosU2:f64 = cosU1 * cosU2;
    let sinU1sinU2:f64 = sinU1 * sinU2;

    let mut sigma:f64 = 0.0;
    let mut deltaSigma:f64 = 0.0;
    let mut cosSqAlpha:f64 = 0.0;
    let mut cos2SM:f64 = 0.0;
    let mut cosSigma:f64 = 0.0;
    let mut sinSigma:f64 = 0.0;
    let mut cosLambda:f64 = 0.0;
    let mut sinLambda:f64 = 0.0;

    let mut lambda:f64 = L; // initial guess
    for iter in 0..MAXITERS {
        let lambdaOrig = lambda;
        cosLambda = lambda.cos();
        sinLambda = lambda.sin();
        let t1:f64 = cosU2 * sinLambda;
        let t2:f64 = cosU1 * sinU2 - sinU1 * cosU2 * cosLambda;
        let sinSqSigma:f64 = t1 * t1 + t2 * t2; // (14)
        sinSigma = sinSqSigma.sqrt();
        cosSigma = sinU1sinU2 + cosU1cosU2 * cosLambda; // (15)
        sigma = sinSigma.atan2(cosSigma); // (16)
        let sinAlpha = if (sinSigma == 0.0) {0.0} else
            {cosU1cosU2 * sinLambda / sinSigma}; // (17)
        cosSqAlpha = 1.0 - sinAlpha * sinAlpha;
        cos2SM = if(cosSqAlpha == 0.0) {0.0} else
            {cosSigma - 2.0 * sinU1sinU2 / cosSqAlpha}; // (18)

        let uSquared = cosSqAlpha * aSqMinusBSqOverBSq; // defn
        A = 1.0 + (uSquared / 16384.0) * // (3)
            (4096.0 + uSquared *
             (-768.0 + uSquared * (320.0 - 175.0 * uSquared)));
        let B = (uSquared / 1024.0) * // (4)
            (256.0 + uSquared *
             (-128.0 + uSquared * (74.0 - 47.0 * uSquared)));
        let C = (f / 16.0) *
            cosSqAlpha *
            (4.0 + f * (4.0 - 3.0 * cosSqAlpha)); // (10)
        let cos2SMSq = cos2SM * cos2SM;
        deltaSigma = B * sinSigma * // (6)
            (cos2SM + (B / 4.0) *
             (cosSigma * (-1.0 + 2.0 * cos2SMSq) -
              (B / 6.0) * cos2SM *
              (-3.0 + 4.0 * sinSigma * sinSigma) *
              (-3.0 + 4.0 * cos2SMSq)));

        lambda = L +
            (1.0 - C) * f * sinAlpha *
            (sigma + C * sinSigma *
             (cos2SM + C * cosSigma *
              (-1.0 + 2.0 * cos2SM * cos2SM))); // (11)

        let delta = (lambda - lambdaOrig) / lambda;
        if (delta.abs() < 1.0e-12) {
            break;
        }
    }

    let distance = b * A * (sigma - deltaSigma);
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
