fn main() {
    tonic_build::compile_protos("proto/hawkeye.proto").expect("Compiled protocol");
}
