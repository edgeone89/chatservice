#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPeerRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPeerResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchingPeerRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(string, tag = "2")]
    pub status: std::string::String,
    #[prost(double, tag = "3")]
    pub latitude: f64,
    #[prost(double, tag = "4")]
    pub longitude: f64,
    #[prost(int32, tag = "5")]
    pub visible_in_radius_in_meters: i32,
    #[prost(string, tag = "6")]
    pub user_name: std::string::String,
    #[prost(int32, tag = "7")]
    pub status_color_id: i32,
    #[prost(string, tag = "8")]
    pub description: std::string::String,
    #[prost(bool, tag = "9")]
    pub is_searching: bool,
    #[prost(string, tag = "10")]
    pub gender: std::string::String,
    #[prost(int32, tag = "11")]
    pub age: i32,
    #[prost(string, tag = "12")]
    pub searching_gender: std::string::String,
    #[prost(int32, tag = "13")]
    pub searching_min_age: i32,
    #[prost(int32, tag = "14")]
    pub searching_max_age: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchingPeerResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub user_id: std::string::String,
    #[prost(int32, tag = "3")]
    pub radius_distance_in_meters: i32,
    #[prost(string, tag = "4")]
    pub status: std::string::String,
    #[prost(int32, tag = "5")]
    pub status_color_id: i32,
    #[prost(string, tag = "6")]
    pub user_name: std::string::String,
    #[prost(string, tag = "7")]
    pub description: std::string::String,
    #[prost(bool, tag = "8")]
    pub is_admin_on: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCoordinatesRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(double, tag = "2")]
    pub latitude: f64,
    #[prost(double, tag = "3")]
    pub longitude: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCoordinatesResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewMessageRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
    #[prost(string, tag = "3")]
    pub to_user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewMessageResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCollectiveMessageRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
    #[prost(string, tag = "3")]
    pub admin_id: std::string::String,
    #[prost(string, tag = "4")]
    pub user_name: std::string::String,
    #[prost(bool, tag = "5")]
    pub is_admin: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCollectiveMessageResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
    #[prost(string, tag = "3")]
    pub user_name: std::string::String,
    #[prost(string, tag = "4")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypingMessageRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(string, tag = "2")]
    pub to_user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypingMessageResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatClosedRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(bool, tag = "2")]
    pub is_closed: bool,
    #[prost(string, tag = "3")]
    pub to_user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatClosedResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectiveChatClosedRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(bool, tag = "2")]
    pub is_closed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectiveChatClosedResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminStatusRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(bool, tag = "2")]
    pub is_admin_on: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminStatusResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerClosedRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerClosedResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockUserInCollectiveChatRequest {
    #[prost(string, tag = "1")]
    pub admin_id: std::string::String,
    #[prost(string, tag = "2")]
    pub blocked_user_id: std::string::String,
    /// 1h or 3h or 5h or always
    #[prost(string, tag = "3")]
    pub blocking_time: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockUserInCollectiveChatResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub blocking_time: std::string::String,
    #[prost(string, tag = "3")]
    pub admin_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCollectiveChatRequest {
    #[prost(string, tag = "1")]
    pub admin_id: std::string::String,
    #[prost(bool, tag = "2")]
    pub clear_chat: bool,
    #[prost(string, tag = "3")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCollectiveChatResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockUserInPersonalChatRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(string, tag = "2")]
    pub blocked_user_id: std::string::String,
    /// 1h or 3h or 5h or always
    #[prost(string, tag = "3")]
    pub blocking_time: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockUserInPersonalChatResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub blocking_time: std::string::String,
    #[prost(string, tag = "3")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearPersonalChatRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(bool, tag = "2")]
    pub clear_chat: bool,
    #[prost(string, tag = "3")]
    pub admin_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearPersonalChatResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportUserRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(string, tag = "2")]
    pub reported_user_id: std::string::String,
    #[prost(bool, tag = "3")]
    pub report_user: bool,
    #[prost(string, tag = "4")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportUserResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadImageRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(bytes, tag = "2")]
    pub file_chunk: std::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub image_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadImageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadImageRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadImageResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(bytes, tag = "2")]
    pub file_chunk: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[doc = r" Generated client implementations."]
pub mod chat_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ChatClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChatClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ChatClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn new_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::NewPeerRequest>,
        ) -> Result<tonic::Response<super::NewPeerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/NewPeer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn searching_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchingPeerRequest>,
        ) -> Result<tonic::Response<super::SearchingPeerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/SearchingPeer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_message(
            &mut self,
            request: impl tonic::IntoRequest<super::NewMessageRequest>,
        ) -> Result<tonic::Response<super::NewMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/NewMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ChatClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ChatClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ChatClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod chat_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    use tokio::sync::Mutex;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ChatServer."]
    #[async_trait]
    pub trait Chat: Send + Sync + 'static {
        #[doc = "Server streaming response type for the NewPeer method."]
        type NewPeerStream: Stream<Item = Result<super::NewPeerResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn new_peer(
            &mut self,
            request: tonic::Request<super::NewPeerRequest>,
        ) -> Result<tonic::Response<Self::NewPeerStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SearchingPeer method."]
        type SearchingPeerStream: Stream<Item = Result<super::SearchingPeerResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn searching_peer(
            &mut self,
            request: tonic::Request<super::SearchingPeerRequest>,
        ) -> Result<tonic::Response<Self::SearchingPeerStream>, tonic::Status>;
        async fn new_coordinates(
            &mut self,
            request: tonic::Request<super::NewCoordinatesRequest>,
        ) -> Result<tonic::Response<super::NewCoordinatesResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the NewMessage method."]
        type NewMessageStream: Stream<Item = Result<super::NewMessageResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn new_message(
            &mut self,
            request: tonic::Request<super::NewMessageRequest>,
        ) -> Result<tonic::Response<Self::NewMessageStream>, tonic::Status>;
        #[doc = "Server streaming response type for the NewCollectiveMessage method."]
        type NewCollectiveMessageStream: Stream<Item = Result<super::NewCollectiveMessageResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn new_collective_message(
            &mut self,
            request: tonic::Request<super::NewCollectiveMessageRequest>,
        ) -> Result<tonic::Response<Self::NewCollectiveMessageStream>, tonic::Status>;
        #[doc = "Server streaming response type for the TypingMessage method."]
        type TypingMessageStream: Stream<Item = Result<super::TypingMessageResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn typing_message(
            &mut self,
            request: tonic::Request<super::TypingMessageRequest>,
        ) -> Result<tonic::Response<Self::TypingMessageStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ChatClosed method."]
        type ChatClosedStream: Stream<Item = Result<super::ChatClosedResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn chat_closed(
            &mut self,
            request: tonic::Request<super::ChatClosedRequest>,
        ) -> Result<tonic::Response<Self::ChatClosedStream>, tonic::Status>;
        type CollectiveChatClosedStream: Stream<Item = Result<super::CollectiveChatClosedResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn collective_chat_closed(
            &mut self,
            request: tonic::Request<super::CollectiveChatClosedRequest>,
        ) -> Result<tonic::Response<Self::CollectiveChatClosedStream>, tonic::Status>;
        async fn peer_closed(
            &mut self,
            request: tonic::Request<super::PeerClosedRequest>,
        ) -> Result<tonic::Response<super::PeerClosedResponse>, tonic::Status>;
        async fn admin_status(
            &mut self,
            request: tonic::Request<super::AdminStatusRequest>,
        ) -> Result<tonic::Response<super::AdminStatusResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the BlockUserInCollectiveChat method."]
        type BlockUserInCollectiveChatStream: Stream<Item = Result<super::BlockUserInCollectiveChatResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn block_user_in_collective_chat(
            &mut self,
            request: tonic::Request<super::BlockUserInCollectiveChatRequest>,
        ) -> Result<tonic::Response<Self::BlockUserInCollectiveChatStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ClearCollectiveChat method."]
        type ClearCollectiveChatStream: Stream<Item = Result<super::ClearCollectiveChatResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn clear_collective_chat(
            &mut self,
            request: tonic::Request<super::ClearCollectiveChatRequest>,
        ) -> Result<tonic::Response<Self::ClearCollectiveChatStream>, tonic::Status>;
        #[doc = "Server streaming response type for the BlockUserInPersonalChat method."]
        type BlockUserInPersonalChatStream: Stream<Item = Result<super::BlockUserInPersonalChatResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn block_user_in_personal_chat(
            &mut self,
            request: tonic::Request<super::BlockUserInPersonalChatRequest>,
        ) -> Result<tonic::Response<Self::BlockUserInPersonalChatStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ClearPersonalChat method."]
        type ClearPersonalChatStream: Stream<Item = Result<super::ClearPersonalChatResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn clear_personal_chat(
            &mut self,
            request: tonic::Request<super::ClearPersonalChatRequest>,
        ) -> Result<tonic::Response<Self::ClearPersonalChatStream>, tonic::Status>;
        #[doc = "Server response type for the ReportUserInProfileChat method."]
        async fn report_user(
            &self,
            request: tonic::Request<super::ReportUserRequest>,
        ) -> Result<tonic::Response<super::ReportUserResponse>, tonic::Status>;
        async fn upload_image(
            &mut self,
            request: tonic::Request<tonic::Streaming<super::UploadImageRequest>>,
        ) -> Result<tonic::Response<super::UploadImageResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the DownloadImage method."]
        type DownloadImageStream: Stream<Item = Result<super::DownloadImageResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn download_image(
            &self,
            request: tonic::Request<super::DownloadImageRequest>,
        ) -> Result<tonic::Response<Self::DownloadImageStream>, tonic::Status>;
        async fn remove_image(
            &mut self,
            request: tonic::Request<super::RemoveImageRequest>,
        ) -> Result<tonic::Response<super::RemoveImageResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ChatServer<T: Chat> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<Mutex<T>>, Option<tonic::Interceptor>);
    impl<T: Chat> ChatServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(Mutex::new(inner));
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(Mutex::new(inner));
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ChatServer<T>
    where
        T: Chat,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/chatservice.Chat/NewPeer" => {
                    #[allow(non_camel_case_types)]
                    struct NewPeerSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::ServerStreamingService<super::NewPeerRequest> for NewPeerSvc<T> {
                        type Response = super::NewPeerResponse;
                        type ResponseStream = T::NewPeerStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewPeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.new_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = NewPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/SearchingPeer" => {
                    #[allow(non_camel_case_types)]
                    struct SearchingPeerSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::ServerStreamingService<super::SearchingPeerRequest> for SearchingPeerSvc<T>
                    {
                        type Response = super::SearchingPeerResponse;
                        type ResponseStream = T::SearchingPeerStream;
                        type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchingPeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.searching_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SearchingPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/NewCoordinates" => {
                    #[allow(non_camel_case_types)]
                    struct NewCoordinatesSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::UnaryService<super::NewCoordinatesRequest> for NewCoordinatesSvc<T> {
                        type Response = super::NewCoordinatesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewCoordinatesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.new_coordinates(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewCoordinatesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/NewMessage" => {
                    #[allow(non_camel_case_types)]
                    struct NewMessageSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::ServerStreamingService<super::NewMessageRequest> for NewMessageSvc<T> {
                        type Response = super::NewMessageResponse;
                        type ResponseStream = T::NewMessageStream;
                        type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.new_message(request).await
                             };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/NewCollectiveMessage" => {
                    #[allow(non_camel_case_types)]
                    struct NewCollectiveMessageSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<super::NewCollectiveMessageRequest>
                        for NewCollectiveMessageSvc<T>
                    {
                        type Response = super::NewCollectiveMessageResponse;
                        type ResponseStream = T::NewCollectiveMessageStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewCollectiveMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.new_collective_message(request).await
                             };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = NewCollectiveMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/TypingMessage" => {
                    #[allow(non_camel_case_types)]
                    struct TypingMessageSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::ServerStreamingService<super::TypingMessageRequest>
                        for TypingMessageSvc<T>
                    {
                        type Response = super::TypingMessageResponse;
                        type ResponseStream = T::TypingMessageStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypingMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.typing_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = TypingMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/ChatClosed" => {
                    #[allow(non_camel_case_types)]
                    struct ChatClosedSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::ServerStreamingService<super::ChatClosedRequest> for ChatClosedSvc<T> {
                        type Response = super::ChatClosedResponse;
                        type ResponseStream = T::ChatClosedStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatClosedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.chat_closed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ChatClosedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/CollectiveChatClosed" => {
                    #[allow(non_camel_case_types)]
                    struct CollectiveChatClosedSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<super::CollectiveChatClosedRequest>
                        for CollectiveChatClosedSvc<T>
                    {
                        type Response = super::CollectiveChatClosedResponse;
                        type ResponseStream = T::CollectiveChatClosedStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CollectiveChatClosedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.collective_chat_closed(request).await
                             };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = CollectiveChatClosedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/PeerClosed" => {
                    #[allow(non_camel_case_types)]
                    struct PeerClosedSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::UnaryService<super::PeerClosedRequest> for PeerClosedSvc<T> {
                        type Response = super::PeerClosedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PeerClosedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.peer_closed(request).await
                             };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PeerClosedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/AdminStatus" => {
                    #[allow(non_camel_case_types)]
                    struct AdminStatusSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::UnaryService<super::AdminStatusRequest> for AdminStatusSvc<T> {
                        type Response = super::AdminStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AdminStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.admin_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AdminStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/BlockUserInCollectiveChat" => {
                    #[allow(non_camel_case_types)]
                    struct BlockUserInCollectiveChatSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<
                            super::BlockUserInCollectiveChatRequest,
                        > for BlockUserInCollectiveChatSvc<T>
                    {
                        type Response = super::BlockUserInCollectiveChatResponse;
                        type ResponseStream = T::BlockUserInCollectiveChatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockUserInCollectiveChatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move {
                                    let mut tmp_inner = inner.lock().await;
                                    tmp_inner.block_user_in_collective_chat(request).await
                                };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = BlockUserInCollectiveChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/ClearCollectiveChat" => {
                    #[allow(non_camel_case_types)]
                    struct ClearCollectiveChatSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<super::ClearCollectiveChatRequest>
                        for ClearCollectiveChatSvc<T>
                    {
                        type Response = super::ClearCollectiveChatResponse;
                        type ResponseStream = T::ClearCollectiveChatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearCollectiveChatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.clear_collective_chat(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ClearCollectiveChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/BlockUserInPersonalChat" => {
                    #[allow(non_camel_case_types)]
                    struct BlockUserInPersonalChatSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<super::BlockUserInPersonalChatRequest>
                        for BlockUserInPersonalChatSvc<T>
                    {
                        type Response = super::BlockUserInPersonalChatResponse;
                        type ResponseStream = T::BlockUserInPersonalChatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockUserInPersonalChatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { 
                                    let mut tmp_inner = inner.lock().await;
                                    tmp_inner.block_user_in_personal_chat(request).await
                                };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = BlockUserInPersonalChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/ClearPersonalChat" => {
                    #[allow(non_camel_case_types)]
                    struct ClearPersonalChatSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<super::ClearPersonalChatRequest>
                        for ClearPersonalChatSvc<T>
                    {
                        type Response = super::ClearPersonalChatResponse;
                        type ResponseStream = T::ClearPersonalChatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearPersonalChatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.clear_personal_chat(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ClearPersonalChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/ReportUser" => {
                    #[allow(non_camel_case_types)]
                    struct ReportUserSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::UnaryService<super::ReportUserRequest> for ReportUserSvc<T> {
                        type Response = super::ReportUserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReportUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.report_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReportUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/UploadImage" => {
                    #[allow(non_camel_case_types)]
                    struct UploadImageSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::ClientStreamingService<super::UploadImageRequest>
                        for UploadImageSvc<T>
                    {
                        type Response = super::UploadImageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::UploadImageRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.upload_image(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = UploadImageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/DownloadImage" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadImageSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::ServerStreamingService<super::DownloadImageRequest>
                        for DownloadImageSvc<T>
                    {
                        type Response = super::DownloadImageResponse;
                        type ResponseStream = T::DownloadImageStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadImageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.download_image(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = DownloadImageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/RemoveImage" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveImageSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::UnaryService<super::RemoveImageRequest> for RemoveImageSvc<T> {
                        type Response = super::RemoveImageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveImageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.remove_image(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveImageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Chat> Clone for ChatServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Chat> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Chat> tonic::transport::NamedService for ChatServer<T> {
        const NAME: &'static str = "chatservice.Chat";
    }
}
