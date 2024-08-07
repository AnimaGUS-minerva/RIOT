use riot_wrappers::println;
use crate::runtime::USE_FIXTURE_SERVER;

use crate::blockwise::*;
use crate::gcoap::*;
use crate::stream::StreamExt;
use super::gcoap::{test_gcoap_get_auto, assert_memo_resp_payload};

pub async fn test_async_blockwise() {
    println!("test_async_blockwise(): 🧪");

    if USE_FIXTURE_SERVER {
//        test_async_blockwise_fixture().await; // uses 'gcoap_c/server.c'
    } else {
        test_async_blockwise_rs().await; // uses 'server.rs'
    }

    println!("test_async_blockwise(): ✅");
}

async fn test_async_blockwise_rs() {
    println!("test_async_blockwise_rs(): 🧪");
    assert!(!USE_FIXTURE_SERVER);

    //

    let (memo, blockwise) = test_gcoap_get_auto("[::1]", "/.well-known/core").await;
    assert!(blockwise);
    assert_memo_resp_payload(&memo);

    //

    test_blockwise_nested("[::1]", "/.well-known/core").await.unwrap();

    //

    println!("test_async_blockwise_rs(): ✅");
}

async fn test_blockwise_nested(addr: &str, uri: &str) -> Result<(), BlockwiseError> {
    println!("🧪 debug NEW [blockwise-1]");
    let mut bs = gcoap_get_blockwise(addr, uri)?;
    assert!(blockwise_states_debug()[0].is_some(), "debug");

    let mut debug_count = 0;
    while let Some(req) = bs.next().await {
        println!("req: {:?}", req);

        let out = req.await;
        println!("@@ out_1: {:?}", out);
        debug_count += 1;

        if debug_count == 2 {
            println!("🧪 debug NEW [blockwise-2]");
            let mut bs = gcoap_get_blockwise(addr, uri)?;
            assert!(blockwise_states_debug()[1].is_some(), "debug");
/* FIXME *** RIOT kernel panic: RUST PANIC
            while let Some(req) = bs.next().await {
                let out = req.await;
                println!("@@ out_2: {:?}", out);
            }
*/
            blockwise_states_print();
//            assert!(blockwise_states_debug()[1].is_none(), "debug");
        }
    }

    blockwise_states_print();
    assert!(blockwise_states_debug()[0].is_none(), "debug");

    Ok(())
}