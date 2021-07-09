#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPeerRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPeerResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchingPeerRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub latitude: f64,
    #[prost(double, tag = "4")]
    pub longitude: f64,
    #[prost(int32, tag = "5")]
    pub visible_in_radius_in_meters: i32,
    #[prost(string, tag = "6")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub status_color_id: i32,
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "9")]
    pub is_searching: bool,
    #[prost(string, tag = "10")]
    pub gender: ::prost::alloc::string::String,
    #[prost(int32, tag = "11")]
    pub age: i32,
    #[prost(string, tag = "12")]
    pub searching_gender: ::prost::alloc::string::String,
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
    pub user_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub radius_distance_in_meters: i32,
    #[prost(string, tag = "4")]
    pub status: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub status_color_id: i32,
    #[prost(string, tag = "6")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub is_admin_on: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCoordinatesRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
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
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to_user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewMessageResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCollectiveMessageRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub admin_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub is_admin: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCollectiveMessageResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypingMessageRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypingMessageResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatClosedRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_closed: bool,
    #[prost(string, tag = "3")]
    pub to_user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatClosedResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectiveChatClosedRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_closed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectiveChatClosedResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminStatusRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_admin_on: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminStatusResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdminStatusRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub admin_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdminStatusResponse {
    #[prost(bool, tag = "1")]
    pub is_admin_on: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerClosedRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerClosedResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockUserInCollectiveChatRequest {
    #[prost(string, tag = "1")]
    pub admin_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub blocked_user_id: ::prost::alloc::string::String,
    /// 1h or 3h or 5h or always
    #[prost(string, tag = "3")]
    pub blocking_time: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockUserInCollectiveChatResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub blocking_time: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub admin_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCollectiveChatRequest {
    #[prost(string, tag = "1")]
    pub admin_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub clear_chat: bool,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearCollectiveChatResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockUserInPersonalChatRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub blocked_user_id: ::prost::alloc::string::String,
    /// 1h or 3h or 5h or always
    #[prost(string, tag = "3")]
    pub blocking_time: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockUserInPersonalChatResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(string, tag = "2")]
    pub blocking_time: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearPersonalChatRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub clear_chat: bool,
    #[prost(string, tag = "3")]
    pub admin_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearPersonalChatResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportUserRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub reported_user_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub report_user: bool,
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportUserResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadImageRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub file_chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub image_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadImageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadImageRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadImageResponse {
    #[prost(int32, tag = "1")]
    pub response_code: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub file_chunk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
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
    #[derive(Debug, Clone)]
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
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> ChatClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ChatClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn new_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::NewPeerRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::NewPeerResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/NewPeer");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn searching_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchingPeerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SearchingPeerResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/SearchingPeer");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn new_coordinates(
            &mut self,
            request: impl tonic::IntoRequest<super::NewCoordinatesRequest>,
        ) -> Result<tonic::Response<super::NewCoordinatesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/NewCoordinates");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_message(
            &mut self,
            request: impl tonic::IntoRequest<super::NewMessageRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::NewMessageResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/NewMessage");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn new_group_message(
            &mut self,
            request: impl tonic::IntoRequest<super::NewCollectiveMessageRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::NewCollectiveMessageResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/NewGroupMessage");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn typing_message(
            &mut self,
            request: impl tonic::IntoRequest<super::TypingMessageRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TypingMessageResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/TypingMessage");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn typing_group_message(
            &mut self,
            request: impl tonic::IntoRequest<super::TypingMessageRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TypingMessageResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/TypingGroupMessage");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn chat_closed(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatClosedRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ChatClosedResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/ChatClosed");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn group_chat_closed(
            &mut self,
            request: impl tonic::IntoRequest<super::CollectiveChatClosedRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CollectiveChatClosedResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/GroupChatClosed");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn peer_closed(
            &mut self,
            request: impl tonic::IntoRequest<super::PeerClosedRequest>,
        ) -> Result<tonic::Response<super::PeerClosedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/PeerClosed");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn admin_status(
            &mut self,
            request: impl tonic::IntoRequest<super::AdminStatusRequest>,
        ) -> Result<tonic::Response<super::AdminStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/AdminStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_admin_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdminStatusRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GetAdminStatusResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/GetAdminStatus");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn block_user_in_group_chat(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockUserInCollectiveChatRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::BlockUserInCollectiveChatResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/chatservice.Chat/BlockUserInGroupChat");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn clear_group_chat(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearCollectiveChatRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ClearCollectiveChatResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/ClearGroupChat");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn block_user_in_personal_chat(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockUserInPersonalChatRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::BlockUserInPersonalChatResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/chatservice.Chat/BlockUserInPersonalChat");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn clear_personal_chat(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearPersonalChatRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ClearPersonalChatResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/ClearPersonalChat");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn report_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportUserRequest>,
        ) -> Result<tonic::Response<super::ReportUserResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/ReportUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn upload_image(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::UploadImageRequest>,
        ) -> Result<tonic::Response<super::UploadImageResponse>, tonic::Status>
        where
            T: std::fmt::Debug,
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/UploadImage");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn download_image(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadImageRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::DownloadImageResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/DownloadImage");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn remove_image(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveImageRequest>,
        ) -> Result<tonic::Response<super::RemoveImageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chatservice.Chat/RemoveImage");
            self.inner.unary(request.into_request(), path, codec).await
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
        type NewPeerStream: futures_core::Stream<Item = Result<super::NewPeerResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn new_peer(
            &mut self,
            request: tonic::Request<super::NewPeerRequest>,
        ) -> Result<tonic::Response<Self::NewPeerStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SearchingPeer method."]
        type SearchingPeerStream: futures_core::Stream<Item = Result<super::SearchingPeerResponse, tonic::Status>>
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
        type NewMessageStream: futures_core::Stream<Item = Result<super::NewMessageResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn new_message(
            &mut self,
            request: tonic::Request<super::NewMessageRequest>,
        ) -> Result<tonic::Response<Self::NewMessageStream>, tonic::Status>;
        #[doc = "Server streaming response type for the NewGroupMessage method."]
        type NewGroupMessageStream: futures_core::Stream<Item = Result<super::NewCollectiveMessageResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn new_group_message(
            &mut self,
            request: tonic::Request<super::NewCollectiveMessageRequest>,
        ) -> Result<tonic::Response<Self::NewGroupMessageStream>, tonic::Status>;
        #[doc = "Server streaming response type for the TypingMessage method."]
        type TypingMessageStream: futures_core::Stream<Item = Result<super::TypingMessageResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn typing_message(
            &mut self,
            request: tonic::Request<super::TypingMessageRequest>,
        ) -> Result<tonic::Response<Self::TypingMessageStream>, tonic::Status>;
        #[doc = "Server streaming response type for the TypingGroupMessage method."]
        type TypingGroupMessageStream: futures_core::Stream<Item = Result<super::TypingMessageResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn typing_group_message(
            &mut self,
            request: tonic::Request<super::TypingMessageRequest>,
        ) -> Result<tonic::Response<Self::TypingGroupMessageStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ChatClosed method."]
        type ChatClosedStream: futures_core::Stream<Item = Result<super::ChatClosedResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn chat_closed(
            &mut self,
            request: tonic::Request<super::ChatClosedRequest>,
        ) -> Result<tonic::Response<Self::ChatClosedStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GroupChatClosed method."]
        type GroupChatClosedStream: futures_core::Stream<Item = Result<super::CollectiveChatClosedResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn group_chat_closed(
            &mut self,
            request: tonic::Request<super::CollectiveChatClosedRequest>,
        ) -> Result<tonic::Response<Self::GroupChatClosedStream>, tonic::Status>;
        async fn peer_closed(
            &mut self,
            request: tonic::Request<super::PeerClosedRequest>,
        ) -> Result<tonic::Response<super::PeerClosedResponse>, tonic::Status>;
        async fn admin_status(
            &mut self,
            request: tonic::Request<super::AdminStatusRequest>,
        ) -> Result<tonic::Response<super::AdminStatusResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the GetAdminStatus method."]
        type GetAdminStatusStream: futures_core::Stream<Item = Result<super::GetAdminStatusResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn get_admin_status(
            &mut self,
            request: tonic::Request<super::GetAdminStatusRequest>,
        ) -> Result<tonic::Response<Self::GetAdminStatusStream>, tonic::Status>;
        #[doc = "Server streaming response type for the BlockUserInGroupChat method."]
        type BlockUserInGroupChatStream: futures_core::Stream<
                Item = Result<super::BlockUserInCollectiveChatResponse, tonic::Status>,
            > + Send
            + Sync
            + 'static;
        async fn block_user_in_group_chat(
            &mut self,
            request: tonic::Request<super::BlockUserInCollectiveChatRequest>,
        ) -> Result<tonic::Response<Self::BlockUserInGroupChatStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ClearGroupChat method."]
        type ClearGroupChatStream: futures_core::Stream<Item = Result<super::ClearCollectiveChatResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn clear_group_chat(
            &mut self,
            request: tonic::Request<super::ClearCollectiveChatRequest>,
        ) -> Result<tonic::Response<Self::ClearGroupChatStream>, tonic::Status>;
        #[doc = "Server streaming response type for the BlockUserInPersonalChat method."]
        type BlockUserInPersonalChatStream: futures_core::Stream<
                Item = Result<super::BlockUserInPersonalChatResponse, tonic::Status>,
            > + Send
            + Sync
            + 'static;
        async fn block_user_in_personal_chat(
            &mut self,
            request: tonic::Request<super::BlockUserInPersonalChatRequest>,
        ) -> Result<tonic::Response<Self::BlockUserInPersonalChatStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ClearPersonalChat method."]
        type ClearPersonalChatStream: futures_core::Stream<Item = Result<super::ClearPersonalChatResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn clear_personal_chat(
            &mut self,
            request: tonic::Request<super::ClearPersonalChatRequest>,
        ) -> Result<tonic::Response<Self::ClearPersonalChatStream>, tonic::Status>;
        async fn report_user(
            &self,
            request: tonic::Request<super::ReportUserRequest>,
        ) -> Result<tonic::Response<super::ReportUserResponse>, tonic::Status>;
        async fn upload_image(
            &mut self,
            request: tonic::Request<tonic::Streaming<super::UploadImageRequest>>,
        ) -> Result<tonic::Response<super::UploadImageResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the DownloadImage method."]
        type DownloadImageStream: futures_core::Stream<Item = Result<super::DownloadImageResponse, tonic::Status>>
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
    pub struct ChatServer<T: Chat> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<Mutex<T>>);
    impl<T: Chat> ChatServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(Mutex::new(inner));
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> Service<http::Request<B>> for ChatServer<T>
    where
        T: Chat,
        B: Body + Send + Sync + 'static,
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
                                //(*inner).new_peer(request).await
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.new_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/SearchingPeer" => {
                    #[allow(non_camel_case_types)]
                    struct SearchingPeerSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::ServerStreamingService<super::SearchingPeerRequest>
                        for SearchingPeerSvc<T>
                    {
                        type Response = super::SearchingPeerResponse;
                        type ResponseStream = T::SearchingPeerStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchingPeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                //(*inner).searching_peer(request).await 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.searching_peer(request).await 
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchingPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).new_coordinates(request).await 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.new_coordinates(request).await 
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewCoordinatesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                //(*inner).new_message(request).await 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.new_message(request).await 
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/NewGroupMessage" => {
                    #[allow(non_camel_case_types)]
                    struct NewGroupMessageSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<super::NewCollectiveMessageRequest>
                        for NewGroupMessageSvc<T>
                    {
                        type Response = super::NewCollectiveMessageResponse;
                        type ResponseStream = T::NewGroupMessageStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewCollectiveMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                //(*inner).new_group_message(request).await 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.new_group_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewGroupMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).typing_message(request).await 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.typing_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TypingMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/TypingGroupMessage" => {
                    #[allow(non_camel_case_types)]
                    struct TypingGroupMessageSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat> tonic::server::ServerStreamingService<super::TypingMessageRequest>
                        for TypingGroupMessageSvc<T>
                    {
                        type Response = super::TypingMessageResponse;
                        type ResponseStream = T::TypingGroupMessageStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypingMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                //(*inner).typing_group_message(request).await 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.typing_group_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TypingGroupMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).chat_closed(request).await
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.chat_closed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChatClosedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/GroupChatClosed" => {
                    #[allow(non_camel_case_types)]
                    struct GroupChatClosedSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<super::CollectiveChatClosedRequest>
                        for GroupChatClosedSvc<T>
                    {
                        type Response = super::CollectiveChatClosedResponse;
                        type ResponseStream = T::GroupChatClosedStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CollectiveChatClosedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                //(*inner).group_chat_closed(request).await
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.group_chat_closed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GroupChatClosedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).peer_closed(request).await
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.peer_closed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PeerClosedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).admin_status(request).await
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.admin_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AdminStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/GetAdminStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetAdminStatusSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<super::GetAdminStatusRequest>
                        for GetAdminStatusSvc<T>
                    {
                        type Response = super::GetAdminStatusResponse;
                        type ResponseStream = T::GetAdminStatusStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAdminStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                //(*inner).get_admin_status(request).await 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.get_admin_status(request).await 
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAdminStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/BlockUserInGroupChat" => {
                    #[allow(non_camel_case_types)]
                    struct BlockUserInGroupChatSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<
                            super::BlockUserInCollectiveChatRequest,
                        > for BlockUserInGroupChatSvc<T>
                    {
                        type Response = super::BlockUserInCollectiveChatResponse;
                        type ResponseStream = T::BlockUserInGroupChatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockUserInCollectiveChatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { 
                                    //(*inner).block_user_in_group_chat(request).await
                                    let mut tmp_inner = inner.lock().await;
                                    tmp_inner.block_user_in_group_chat(request).await
                                };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlockUserInGroupChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chatservice.Chat/ClearGroupChat" => {
                    #[allow(non_camel_case_types)]
                    struct ClearGroupChatSvc<T: Chat>(pub Arc<Mutex<T>>);
                    impl<T: Chat>
                        tonic::server::ServerStreamingService<super::ClearCollectiveChatRequest>
                        for ClearGroupChatSvc<T>
                    {
                        type Response = super::ClearCollectiveChatResponse;
                        type ResponseStream = T::ClearGroupChatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearCollectiveChatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { 
                                //(*inner).clear_group_chat(request).await
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.clear_group_chat(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearGroupChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                    //(*inner).block_user_in_personal_chat(request).await
                                    let mut tmp_inner = inner.lock().await;
                                    tmp_inner.block_user_in_personal_chat(request).await
                                };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlockUserInPersonalChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).clear_personal_chat(request).await 
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.clear_personal_chat(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearPersonalChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).report_user(request).await
                                let tmp_inner = inner.lock().await;
                                tmp_inner.report_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReportUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).upload_image(request).await
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.upload_image(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadImageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).download_image(request).await
                                let tmp_inner = inner.lock().await;
                                tmp_inner.download_image(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DownloadImageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                                //(*inner).remove_image(request).await
                                let mut tmp_inner = inner.lock().await;
                                tmp_inner.remove_image(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveImageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Chat> Clone for ChatServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Chat> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
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
