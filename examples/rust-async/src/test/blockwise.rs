use riot_wrappers::println;
use crate::runtime::USE_FIXTURE_SERVER;

pub async fn test_async_blockwise() {
    println!("test_async_blockwise(): 🧪");

    if USE_FIXTURE_SERVER {
//        test_async_blockwise_fixture().await; // uses 'gcoap_c/server.c'
    } else {
//        test_async_blockwise_rs().await; // uses 'server.rs'
    }

    println!("test_async_blockwise(): ✅");
}