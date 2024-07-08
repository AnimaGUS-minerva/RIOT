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
    emulate_sync_gcoap_get(); // !!!!
}

fn emulate_sync_gcoap_get() {
    use core::ffi::c_void;
    extern "C" {
        fn gcoap_req_send_async(
            addr: *const u8, uri: *const u8, method: u8,
            payload: *const u8, payload_len: usize,
            blockwise: bool, idx: usize,
            context: *const c_void, resp_handler: *const c_void);
    }

    //
    // emulate "gcoap get [::1] /.well-known/core"
    //

    const REQ_ADDR_MAX: usize = 64;
    const REQ_URI_MAX: usize = 64;

    let mut addr_cstr = heapless::String::<{ REQ_ADDR_MAX + 1 }>::new();
    addr_cstr.push_str("[::1]").unwrap();
    addr_cstr.push('\0').unwrap();

    let mut uri_cstr = heapless::String::<{ REQ_URI_MAX + 1 }>::new();
    uri_cstr.push_str("/.well-known/core").unwrap();
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