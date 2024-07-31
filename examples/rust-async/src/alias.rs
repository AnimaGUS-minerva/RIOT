use riot_wrappers::println;

pub const TABLE_ALIAS_NAMED: &[(&str, &str)] = &[
    ("a", "alias"),
    ("h", "help"),
    ("p", "ps.rs"),
];

pub const TABLE_ALIAS_ENUMERATED: &[&str] = &[
    "version",
    "ifconfig",
    "ping ::1",
    "coap get coap://[::1]/.well-known/core", // !!
];

pub const TABLE_ALIAS_FUNCTION: &[&str] = &[
    "f0",
    "f1",
    "f2",
    "f", // !!
];

pub async fn run_function_alias(name: &str) {
    match name {
        "f0" => (|| println!("hello world!"))(),
        "f1" => test_async_sleep().await,
        "f2" => test_async_timeout().await,
        "f" => test_async_gcoap().await, // !!
        _ => println!("oops, code for function alias [{}] is missing!", name),
    }
}

async fn test_async_sleep() {
    println!("test_async_sleep():");

    for count in 1..=3 {
        println!("{}", count);
        crate::util::sleep_msec(1000).await;
    }
}

async fn test_async_timeout() {
    println!("test_async_timeout():");

    crate::util::set_timeout(2000, || println!("it works!")).await;
}

async fn test_async_gcoap_fixture() {
    println!("test_async_gcoap_fixture():");

    assert!(super::runtime::USE_FIXTURE_SERVER);

    //

    // per 'gcoap_c/server.c'
    test_gcoap_get_auto("[::1]", "/.well-known/core").await; // non-blockwise
    test_gcoap_get_auto("[::1]", "/cli/stats").await; // COAP_GET | COAP_PUT
    test_gcoap_get_auto("[::1]", "/riot/board").await; // COAP_GET

    //

    use super::gcoap::{gcoap_get, gcoap_post, gcoap_put};
    let gcoap_get_cli_stats = || gcoap_get("[::1]", "/cli/stats");

    println!("----:");
    println!("{:?}", gcoap_get_cli_stats().await);

    println!("----:");
    let _ = gcoap_post("[::1]", "/cli/stats", b"3000").await;
    println!("{:?} (after COAP_POST)", gcoap_get_cli_stats().await);

    println!("----:");
    let _ = gcoap_put("[::1]", "/cli/stats", b"1000").await;
    println!("{:?} (after COAP_PUT)", gcoap_get_cli_stats().await);

    //
}

async fn test_async_gcoap() {
    println!("test_async_gcoap():");

    if 0 == 1 { // NO auto-handle blockwise context unlike `gcoap_get_auto()`
        emulate_sync_gcoap_get("[::1]", "/.well-known/core");
        return;
    }

    if 1 == 1 {
        if super::runtime::USE_FIXTURE_SERVER {
            test_async_gcoap_fixture().await;
        } else { // per 'server.rs'
            test_gcoap_get_auto("[::1]", "/.well-known/core").await; // blockwise
        }
        return;
    }
}

async fn test_gcoap_get_auto(addr: &str, uri: &str) {
    use super::gcoap::gcoap_get_auto;
    use super::stream::StreamExt;

    let (memo, bs) = gcoap_get_auto(addr, uri).await.unwrap();
    println!("memo: {:?}", memo);
    println!("bs: {:?}", bs);

    if let Some(mut bs) = bs { // ok for e.g. crate::server::start()
        while let Some(req) = bs.next().await {
            println!("@@ memo cont: {:?}", req.await);
        }
        println!("blockwise done");
    } else { // ok for e.g. crate::server::start_fixture()
        println!("non-blockwise done");
    }
}

//

fn emulate_sync_gcoap_get(addr: &str, uri: &str) {
    use core::ffi::c_void;
    extern "C" {
        fn gcoap_req_send_async(
            addr: *const u8, uri: *const u8, method: u8,
            payload: *const u8, payload_len: usize,
            blockwise: bool, idx: usize,
            context: *const c_void, resp_handler: *const c_void);
    }

    const REQ_ADDR_MAX: usize = 64;
    const REQ_URI_MAX: usize = 64;

    let mut addr_cstr = heapless::String::<{ REQ_ADDR_MAX + 1 }>::new();
    addr_cstr.push_str(addr).unwrap();
    addr_cstr.push('\0').unwrap();

    let mut uri_cstr = heapless::String::<{ REQ_URI_MAX + 1 }>::new();
    uri_cstr.push_str(uri).unwrap();
    uri_cstr.push('\0').unwrap();

    let payload_ptr: *const u8 = core::ptr::null();
    let payload_len: usize = 0;

    let blockwise = false;
    let blockwise_state_index = None;

    unsafe { gcoap_req_send_async(
        addr_cstr.as_ptr(),
        uri_cstr.as_ptr(),
        0x01 /* COAP_METHOD_GET */, payload_ptr, payload_len,
        blockwise, blockwise_state_index.unwrap_or(0 /* to be ignored */),
        core::ptr::null(), // !!!! fstat_ptr as *const c_void, // context
        core::ptr::null()); // !!!! gcoap_req_resp_handler as *const c_void);
    }
}

//