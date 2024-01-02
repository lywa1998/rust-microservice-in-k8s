fn main() {
    // tonic_build::compile_protos("proto/echo.proto").unwrap();
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(
            &["proto/echo.proto"], 
            &["proto"]
        )
        .unwrap();
}