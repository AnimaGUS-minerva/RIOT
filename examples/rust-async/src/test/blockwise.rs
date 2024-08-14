use riot_wrappers::println;
use crate::runtime::USE_FIXTURE_SERVER;

use crate::blockwise::*;
use crate::gcoap::*;
use crate::stream::StreamExt;
use super::gcoap::{test_gcoap_get_auto, assert_memo_resp_payload};

pub async fn test_async_blockwise() {
    println!("test_async_blockwise(): 🧪");

    if USE_FIXTURE_SERVER {
        //test_async_blockwise_fixture().await; // uses 'gcoap_c/server.c'
    } else {
        test_async_blockwise_rs().await.unwrap(); // uses 'server.rs'
    }

    println!("test_async_blockwise(): ✅");
}

async fn test_async_blockwise_rs() -> Result<(), BlockwiseError> {
    println!("test_async_blockwise_rs(): 🧪");
    assert!(!USE_FIXTURE_SERVER);

    let addr = "[::1]";
    let uri = "/.well-known/core";

    test_blockwise_payload(addr, uri).await?;
    test_blockwise_nested(addr, uri).await?;
    test_blockwise_close(addr, uri).await?;
    test_blockwise_none().await?;
    test_blockwise_timeout().await?;

    println!("test_async_blockwise_rs(): ✅");
    Ok(())
}

async fn test_blockwise_payload(addr: &str, uri: &str) -> Result<(), BlockwiseError> {
    println!("test_blockwise_payload(): 🧪");

    let (memo, blockwise) = test_gcoap_get_auto(addr, uri).await;
    assert!(blockwise);
    assert_memo_resp_payload(&memo);

    println!("test_blockwise_payload(): ✅");
    Ok(())
}


async fn test_blockwise_nested(addr: &str, uri: &str) -> Result<(), BlockwiseError> {
    println!("test_blockwise_nested(): 🧪");

    let mut bs = gcoap_get_blockwise(addr, uri)?;
    assert!(blockwise_states_debug()[0].is_some(), "debug");

    let mut count = 0;
    while let Some(req) = bs.next().await {
        let _out = req.await;
        //println!("@@ _out_nested_0: {:?}", _out);
        count += 1;

        if count == 1 {
            let mut bs = gcoap_get_blockwise(addr, uri)?;
            assert!(blockwise_states_debug()[1].is_some(), "debug");

            while let Some(req) = bs.next().await {
                let _out = req.await;
                //println!("@@ _out_nested_1: {:?}", _out);
            }

            blockwise_states_print();
            assert!(blockwise_states_debug()[1].is_none(), "debug");
        }
    }

    assert!(count > 2); // assume multiple blocks for this test endpoint

    blockwise_states_print();
    assert!(blockwise_states_debug()[0].is_none(), "debug");

    println!("test_blockwise_nested(): ✅");
    Ok(())
}

async fn test_blockwise_close(addr: &str, uri: &str) -> Result<(), BlockwiseError> {
    println!("test_blockwise_close(): 🧪");

    let mut bss = heapless::Vec::<_, BLOCKWISE_STATES_MAX>::new();
    for _ in 0..BLOCKWISE_STATES_MAX {
        bss.push(gcoap_get_blockwise(addr, uri)?).unwrap();
    }
    assert_eq!(gcoap_get_blockwise(addr, uri).err(), Some(BlockwiseError::StateNotAvailable));

    let req0 = bss[0].next().await.unwrap();
    let req1 = bss[1].next().await.unwrap();

    // before `.close()`

    assert!(match req0.await {
        GcoapMemoState::Resp(true, Some(x)) => x.len() > 0,
        _ => false,
    });

    bss.iter_mut().for_each(|bs| bs.close());
    blockwise_states_debug().iter().for_each(|x| assert!(x.is_none()));

    // after `.close()`

    assert!(bss[0].next().await.is_none());
    assert!(bss[1].next().await.is_none());
    assert_eq!(req1.await, GcoapMemoState::Err(false));

    println!("test_blockwise_close(): ✅");
    Ok(())
}

async fn test_blockwise_none() -> Result<(), BlockwiseError> {
    println!("test_blockwise_none(): 🧪");

    let addr = "[::1]";
    let uri = "/.well-known/core__"; // induce `Resp(None)`

    let mut bs = gcoap_get_blockwise(addr, uri)?;
    while let Some(req) = bs.next().await {
        match req.await {
            GcoapMemoState::Resp(false, None) => bs.close(),
            _ => panic!(),
        };
    }

    println!("test_blockwise_none(): ✅");
    Ok(())
}

async fn test_blockwise_timeout() -> Result<(), BlockwiseError> {
    println!("test_blockwise_timeout(): 🧪");

    let addr = "[::1]:5680"; // induce `Timeout`, not 5683
    let uri = "/.well-known/core";

    let mut bs = gcoap_get_blockwise(addr, uri)?;
    while let Some(req) = bs.next().await {
        match req.await {
            GcoapMemoState::Timeout(false) => bs.close(),
            _ => panic!(),
        };
    }

    println!("test_blockwise_timeout(): ✅");
    Ok(())
}