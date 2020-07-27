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
    #[prost(int32, tag = "2")]
    pub radius_distance_in_meters: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchingPeerResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub user_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewMessageRequest {
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewMessageResponse {
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
        type SearchingPeerStream: Stream<Item = Result<super::SearchingPeerResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn new_peer(
            &mut self,
            request: tonic::Request<super::NewPeerRequest>,
        ) -> Result<tonic::Response<super::NewPeerResponse>, tonic::Status>;
        async fn searching_peer(
            &mut self,
            request: tonic::Request<super::SearchingPeerRequest>,
        ) -> Result<tonic::Response<Self::SearchingPeerStream>, tonic::Status>;
        async fn new_message(
            &self,
            request: tonic::Request<super::NewMessageRequest>,
        ) -> Result<tonic::Response<super::NewMessageResponse>, tonic::Status>;
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
                    impl<T: Chat> tonic::server::UnaryService<super::NewPeerRequest> for NewPeerSvc<T> {
                        type Response = super::NewPeerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
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
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewPeerSvc(inner);
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
                "/chatservice.Chat/NewMessage" => {
                    #[allow(non_camel_case_types)]
                    struct NewMessageSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::UnaryService<super::NewMessageRequest> for NewMessageSvc<T> {
                        type Response = super::NewMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
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
