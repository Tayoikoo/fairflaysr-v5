pub fn main() {
    let net_protocol_file = "StarRail.proto";
    if std::path::Path::new(net_protocol_file).exists() {
        println!("cargo:rerun-if-changed={net_protocol_file}");

        prost_build::Config::new()
            .out_dir("out/")
            .compile_protos(&[net_protocol_file], &["."])
            .unwrap();
    }

    let bin_protocol_file = "bin.server.proto";
    if std::path::Path::new(bin_protocol_file).exists() {
        println!("cargo:rerun-if-changed={bin_protocol_file}");

        prost_build::Config::new()
            .out_dir("out/")
            .compile_protos(&[bin_protocol_file], &["bin"])
            .unwrap();
    }
}
