fn main() {
    println!("cargo:rerun-if-changed=src/stb/");
    cc::Build::new()
        .file("src/stb/stb_image_write.c")
        .compile("stb_image_write");
}
