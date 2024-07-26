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

async fn test_async_gcoap() {
    println!("test_async_gcoap():");

    if 0 == 1 { // NO auto-handle blockwise context (like alias [3])
        emulate_sync_gcoap_get("[::1]", "/.well-known/core");
        return;
    }

    if 1 == 1 { // WIP !!!! auto-handle blockwise context (like alias [3])
        gcoap_get_auto_wip("[::1]", "/.well-known/core").await;
        return;
    }

    {
        use super::gcoap::{gcoap_get, gcoap_post, gcoap_put};

        println!("-------- out-0:");
        println!("{:?}", gcoap_get("[::1]", "/.well-known/core").await);
        // !!!! FIXME wrong modality  <<<< --- blockwise start ---
        // TODO auto-handle blockwise context (like alias [3])

/*
        println!("-------- out-1:");
        println!("{:?}", gcoap_get("[::1]", "/cli/stats").await);

        let addr_self = "[::1]:5683";
        println!("-------- out-2:");
        let _ = gcoap_post(addr_self, "/cli/stats", b"3000").await;
        println!("{:?}", gcoap_get(addr_self, "/cli/stats").await);
        println!("-------- out-3:");
        let _ = gcoap_put(addr_self, "/cli/stats", b"1000").await;
        println!("{:?}", gcoap_get(addr_self, "/cli/stats").await);
*/
    }
}

async fn test_async_gcoap_blockwise() {
    todo!();
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

async fn gcoap_get_auto_wip(addr: &str, uri: &str) {
    use super::gcoap::gcoap_get_blockwise;
    use super::stream::StreamExt;

    let mut bs = gcoap_get_blockwise(addr, uri).unwrap();

    if let Some(req) = bs.next().await {
        let first = req.await;
        println!("@@ first: {:?}", first);

        if first.is_blockwise() {
            while let Some(req) = bs.next().await {
                println!("@@ cont: {:?}", req.await);
            }
        } else {
            bs.close();
        }
    }
}