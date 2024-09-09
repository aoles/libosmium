fn main() {
    println!("cargo:rerun-if-changed=src/libosmium.cpp");
    cc::Build::new()
        .cpp(true)
        .std("c++14")
        .include("libosmium/include")
        .include("protozero/include")
        .file("src/libosmium.cpp")
        .compile("osmium");
    println!("cargo:rustc-link-lib=z");
}
