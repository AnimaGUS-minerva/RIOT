#[macro_export]
macro_rules! static_borrow_mut { ($x:expr) => (unsafe { &mut *core::ptr::addr_of_mut!($x) }) }

pub fn to_raw<T>(x: &mut T) -> *mut () { x as *mut _ as _ }
pub fn static_from_raw<T>(p: *mut ()) -> &'static mut T { unsafe { core::mem::transmute(p) } }
pub fn get_static<T>(x: &mut T) -> &'static mut T { static_from_raw(to_raw(x)) }

pub async fn sleep_msec(ms: u32) {
    use riot_wrappers::ztimer::*;

    Clock::msec().sleep_async(Ticks(ms)).await;
}