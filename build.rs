fn main() {
    cc::Build::new()
        .file("LyricDecoder/LyricDecoder.c")
        .file("LyricDecoder/QQMusicDES/des.c")
        .include("LyricDecoder")
        .include("LyricDecoder/QQMusicDES")
        .include("LyricDecoder/zlib")
        .compile("LyricDecoder");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=LyricDecoder");
}