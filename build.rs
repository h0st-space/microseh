#[cfg(all(windows, not(docsrs)))]
extern crate cc;

#[cfg(all(windows, not(docsrs)))]
fn main() {
    // TODO: this is a hack to allow this crate to build on docs.rs.
    //       https://github.com/sonodima/microseh/pull/11#issuecomment-2385633164
    if !std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default().contains("gnu") {
        cc::Build::new().file("src/stub.c").compile("sehstub");
    }
}

#[cfg(any(not(windows), docsrs))]
fn main() {
    println!("cargo:warning=building for a non-supported platform, exception handling will not be available");
}
