fn main() {
    tonic_build::compile_protos("proto/chat.proto").unwrap();
    tonic_build::compile_protos("proto/pushnotifications.proto").unwrap();
}
