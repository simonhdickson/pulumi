fn main() {
    tower_grpc_build::Config::new()
        .enable_server(true)
        .build(
            &[
                "../proto/analyzer.proto",
                "../proto/engine.proto",
                "../proto/errors.proto",
                "../proto/language.proto",
                "../proto/plugin.proto",
                "../proto/provider.proto",
                "../proto/resource.proto",
                "../proto/status.proto",
            ],
            &["../proto/"],
        )
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
    println!("cargo:rerun-if-changed=../proto/");
}
