#[cfg(not(docsrs))]
extern crate cc;

#[cfg(not(docsrs))]
fn main() {
    if std::env::var("HOST").unwrap().contains("gnu") { return; }
    cc::Build::new().file("src/stub.c").compile("sehstub");
}

#[cfg(docsrs)]
fn main() {}
