extern crate cc;

#[cfg(not(docsrs))]
fn main() {
    cc::Build::new().file("src/stub.c").compile("sehstub");
}

#[cfg(docsrs)]
fn main() {}
