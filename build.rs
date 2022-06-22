//! Emits linker flags depending on platforms and features.
//!
//! (iOS/macOS only right now... maybe tvOS one day?)

fn main() {
    println!("cargo:rustc-link-lib=framework=Network");
}
