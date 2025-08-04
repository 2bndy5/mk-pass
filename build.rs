#[cfg(feature = "node-binding")]
extern crate napi_build;

fn main() {
    #[cfg(feature = "node-binding")]
    napi_build::setup();
}
