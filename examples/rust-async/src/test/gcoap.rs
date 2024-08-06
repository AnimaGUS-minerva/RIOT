use riot_wrappers::println;


async fn test_async_gcoap_fixture() { // per 'gcoap_c/server.c'
    println!("test_async_gcoap_fixture(): 🧪");

    assert!(crate::runtime::USE_FIXTURE_SERVER);

    //

    let assert_memo_resp_payload = |memo| if let GcoapMemoState::Resp(_, Some(payload)) = memo {
        assert!(payload.len() > 0);
    } else { panic!(); };

    let (memo, blockwise) = test_gcoap_get_auto("[::1]", "/.well-known/core").await; // non-blockwise
    assert_eq!(blockwise, false);
    assert_memo_resp_payload(memo);

    let (memo, blockwise) = test_gcoap_get_auto("[::1]", "/cli/stats").await; // COAP_GET | COAP_PUT
    assert_eq!(blockwise, false);
    assert_memo_resp_payload(memo);

    let (memo, blockwise) = test_gcoap_get_auto("[::1]", "/riot/board").await; // COAP_GET
    assert_eq!(blockwise, false);
    assert_memo_resp_payload(memo);

    //

    use crate::gcoap::{gcoap_get, gcoap_post, gcoap_put};
    let gcoap_get_cli_stats = || gcoap_get("[::1]", "/cli/stats");

    let assert_cli_stats = |memo, expected: &[u8]| if let GcoapMemoState::Resp(_, Some(payload)) = memo {
        assert_eq!(payload, expected);
    } else { panic!(); };

    println!("----: (orig)");
    assert_cli_stats(gcoap_get_cli_stats().await, b"0");

    println!("----: (after COAP_POST)");
    let _ = gcoap_post("[::1]", "/cli/stats", b"3000").await; // NOP (endpoint is for COAP_GET | COAP_PUT)
    assert_cli_stats(gcoap_get_cli_stats().await, b"0");

    println!("----: (after COAP_PUT)");
    let _ = gcoap_put("[::1]", "/cli/stats", b"1000").await;
    assert_cli_stats(gcoap_get_cli_stats().await, b"1000");

    println!("test_async_gcoap_fixture(): ✅");
}

pub async fn test_async_gcoap() {
    println!("test_async_gcoap(): 🧪");

    if 0 == 1 { // debug; NO auto-handle blockwise context unlike `gcoap_get_auto()`
        emulate_sync_gcoap_get("[::1]", "/.well-known/core");
        return;
    }

    if crate::runtime::USE_FIXTURE_SERVER {
        test_async_gcoap_fixture().await;
    } else { // per 'server.rs'
        let (memo, blockwise) = test_gcoap_get_auto("[::1]", "/.well-known/core").await;

        assert!(blockwise);
        if let GcoapMemoState::Resp(_, Some(payload)) = memo {
            assert!(payload.len() > 0);
        } else { panic!(); }
    }

    println!("test_async_gcoap(): ✅");
}

use crate::gcoap::{gcoap_get_auto, GcoapMemoState};
use crate::stream::StreamExt;
async fn test_gcoap_get_auto(addr: &str, uri: &str) -> (GcoapMemoState, bool) {

    let (memo, mut bs) = gcoap_get_auto(addr, uri).await.unwrap();
    println!("memo: {:?}", memo);
    println!("bs: {:?}", bs);

    if let Some(ref mut bs) = bs {
        while let Some(req) = bs.next().await {
            println!("@@ memo cont: {:?}", req.await);
        }
        println!("blockwise done");
    } else {
        println!("non-blockwise done");
    }

    (memo, bs.is_some())
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