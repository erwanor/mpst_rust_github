mod http;
mod tcp;

use ntest::timeout;

#[test]
fn tcp() {
    tcp::binary::main();
    tcp::binary_fail::main();
}

#[test]
#[timeout(60000)]
fn http() {
    http::simple_http_get::main();
    http::simple_http_post::main();
    http::simple_https_get::main();
    http::complex_https_get::main();
    http::binary_http_get::main();
    http::o_auth::main();
    http::o_auth_fail_too_true::main();
    http::o_auth_fail_too_false::main();
}
