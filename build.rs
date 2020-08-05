extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .customize(protobuf_codegen_pure::Customize {
            ..Default::default()
        })
        .out_dir("src/protos")
        .input("src/protos/telemetry.proto")
        .include("src/protos")
        .run()
        .expect("protoc");
}
