#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = fuzz(data);
});

fn fuzz(data: &[u8]) -> Option<()> {
    // load fuzz data in an in memory file
    let file = memfd_path::InMemFilePath::new(data).ok()?;

    // set SSL_CERT_FILE to /proc/self/fd/3 (or similar)
    std::env::set_var("SSL_CERT_FILE", file.as_ref());

    // load certs
    rustls_native_certs::load_native_certs().ok()?;

    Some(())
}
