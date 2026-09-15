use core::ffi::c_void;
use core::ptr;
use core::ptr::NonNull;
use core::sync::atomic::{
    Atomic, AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr,
    AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize,
};
use core::time::Duration;
use crate::error::error;
use crate::{multipar, x4};
use crate::types::C_VOID;

pub const INFINITE: u32 = 4294967295u32;

// atomic for >= 32 bit futex
pub type Futex = Atomic<Primitive>;
pub type Primitive = u32;

// atomic for >= 8 bit futex
pub type SmallFutex = Atomic<SmallPrimitive>;
pub type SmallPrimitive = u8;

pub unsafe trait Futexable {
    unsafe fn lock(&self);
    unsafe fn unlock(&self);
    unsafe fn wait(&self);
    unsafe fn wait_timeout(&self, timeout: Duration) -> bool;
    unsafe fn mem_map(&self, addr: C_VOID, len: usize);
    unsafe fn mem_unmap(&self, addr: C_VOID, len: usize);
    unsafe fn renew_at(&self, ptr: NonNull<AtomicU64>);
}
pub unsafe trait Waitable {
    type Futex;
}
macro_rules! unsafe_waitable_int {
    ($(($int:ty, $atomic:ty)),*$(,)?) => {
        $(
            unsafe impl Waitable for $int {
                type Futex = $atomic;
            }
            unsafe impl Futexable for $atomic {
                unsafe fn lock(&self) {  todo!() }
                unsafe fn unlock(&self) { todo!() }
                unsafe fn wait(&self) { todo!() }
                unsafe fn wait_timeout(&self, timeout: Duration) -> bool { todo!() }
                unsafe fn mem_map(&self, addr: C_VOID, len: usize) { todo!() }
                unsafe fn mem_unmap(&self, addr: C_VOID, len: usize) { todo!() }
                unsafe fn renew_at(&self, ptr: NonNull<AtomicU64>) { todo!() }
            }
        )*
    };
}
unsafe_waitable_int! {
    (bool, AtomicBool),
    (i8, AtomicI8),
    (i16, AtomicI16),
    (i32, AtomicI32),
    (i64, AtomicI64),
    (isize, AtomicIsize),
    (u8, AtomicU8),
    (u16, AtomicU16),
    (u32, AtomicU32),
    (u64, AtomicU64),
    (usize, AtomicUsize),
}
unsafe impl<T> Waitable for *const T {
    type Futex = Atomic<*mut T>;
}
unsafe impl<T> Waitable for *mut T {
    type Futex = Atomic<*mut T>;
}
unsafe impl<T> Futexable for AtomicPtr<T> {
    unsafe fn lock(&self) {  todo!() }
    unsafe fn unlock(&self) { todo!() }
    unsafe fn wait(&self) { todo!() }
    unsafe fn wait_timeout(&self, timeout: Duration) -> bool { todo!() }
    unsafe fn mem_map(&self, addr: C_VOID, len: usize) { todo!() }
    unsafe fn mem_unmap(&self, addr: C_VOID, len: usize) { todo!() }
    unsafe fn renew_at(&self, ptr: NonNull<AtomicU64>) { todo!() }
}

pub fn wait_on_address<W: Waitable>(
    address: &W::Futex,
    compare: W,
    timeout: Option<Duration>,
) -> bool {
    unsafe {
        let addr = ptr::from_ref(address).cast::<c_void>();
        let size = size_of::<W>();
        let compare_addr = (&raw const compare).cast::<c_void>();
        let timeout = timeout.map(x4::counter::ClockTime::dur2timeout).unwrap_or(INFINITE);
        // wait_on_addr(addr, compare_addr, size, timeout) == 1
        todo!()
    }
}

pub fn wake_by_address_single<T: Futexable>(address: &T) {
    unsafe {
        let addr = ptr::from_ref(address).cast::<c_void>();
        // todo: use multipar
        todo!()
    }
}

pub fn wake_by_address_all<T: Futexable>(address: &T) {
    unsafe {
        let addr = ptr::from_ref(address).cast::<c_void>();
        // c::WakeByAddressAll(addr);
        //todo: implement wake all futexes by addr
        todo!()
    }
}

pub fn futex_wait<W: Waitable>(futex: &W::Futex, expected: W, timeout: Option<Duration>) -> bool {
    // return false only on timeout
    wait_on_address(futex, expected, timeout) || 0 != 0 //todo: poll futex timeout
}

pub fn futex_wake<T: Futexable>(futex: &T) -> bool {
    wake_by_address_single(futex);
    false
}

pub fn futex_wake_all<T: Futexable>(futex: &T) {
    wake_by_address_all(futex)
}