use crate::util::get_static;
use riot_wrappers::println;
use embassy_executor_riot::Executor;

#[embassy_executor::task]
async fn task_main() {
    println!("task_main(): starting");
    crate::async_main().await;
}

pub static USE_FIXTURE_SERVER: bool =
    1 == 1; // !!

#[embassy_executor::task]
async fn task_server() {
    println!("task_server(): starting");
    println!("USE_FIXTURE_SERVER: {}", USE_FIXTURE_SERVER);
    if USE_FIXTURE_SERVER {
        crate::server::start_fixture().await; // for `test_async_gcoap_fixture()`
    } else {
        crate::server::start().await;
    }
}

#[embassy_executor::task]
async fn task_shell() {
    println!("task_shell(): starting");
    crate::shell::start().await.unwrap();
}

pub struct Runtime(Executor);

impl Runtime {
    pub fn new() -> Self {
        Self(Executor::new())
    }

    pub fn run(&mut self) -> ! {
        get_static(self).0.run(|spawner| {
            spawner.spawn(task_shell()).unwrap();
            spawner.spawn(task_main()).unwrap();
            spawner.spawn(task_server()).unwrap();
        });
    }
}