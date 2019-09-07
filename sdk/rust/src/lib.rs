pub mod pulumi_rpc {
    include!(concat!(env!("OUT_DIR"), "/pulumirpc.rs"));
}

#[derive(Debug, Clone)]
struct LanguageHost {
    engineAddress: String,
}

impl pulumi_rpc::server::LanguageRuntime for LanguageHost {}
