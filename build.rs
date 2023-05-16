fn main() {
    cc::Build::new()
        .file("src/LyricDecoder/LyricDecoder.c")
        .file("src/LyricDecoder/QQMusicDES/des.c")
        .include("src/LyricDecoder")
        .include("src/LyricDecoder/QQMusicDES")
        .include("src/LyricDecoder/zlib")
        .compile("LyricDecoder");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=LyricDecoder");
}