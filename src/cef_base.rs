use libc::size_t;
use std::default::Default;
use std::mem;
use std::os::raw;
use std::sync::atomic;
use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(unused_variables)]
extern "C" fn nop_base(ptr: *mut CefBase) {
    return;
}

#[allow(unused_variables)]
extern "C" fn nop_base_int(ptr: *mut CefBase) -> i32 {
    return 1;
}

#[repr(C)]
pub struct CefBase {
    pub size: size_t,
    add_ref: extern "C" fn(this: *mut CefBase),
    release: extern "C" fn(this: *mut CefBase) -> i32,
    has_one_ref: extern "C" fn(this: *mut CefBase) -> i32,
    has_at_least_one_ref: extern "C" fn(this: *mut CefBase) -> i32,
}

impl Default for CefBase {
    fn default() -> Self {
        return CefBase::get::<CefBase>();
    }
}

impl CefBase {
    pub fn get<T>() -> CefBase {
        return CefBase {
            size: mem::size_of::<T>() as size_t,
            add_ref: nop_base,
            release: nop_base_int,
            has_one_ref: nop_base_int,
            has_at_least_one_ref: nop_base_int,
        };
    }
}

pub fn return_non_ref_counted_as_cef_sys<T, U>(obj: &T) -> *mut U {
    // Return a pointer to the vtable that cef dll expects.
    let cef_sys: *mut U = unsafe { std::mem::transmute(obj) };
    cef_sys
}

pub fn add_ref_and_return_as_cef_sys<T, U>(obj: &T) -> *mut U {
    // First add-ref the object because we're about to send over the dll boundary.
    let cef_base: *mut cef_sys::_cef_base_ref_counted_t = unsafe { std::mem::transmute(obj) };
    unsafe {
        ((*cef_base).add_ref.unwrap())(cef_base);
    }

    // Now return a pointer to the vtable that cef dll expects.
    let cef_sys: *mut U = unsafe { std::mem::transmute(obj) };
    cef_sys
}

pub fn make_empty_cef_base() -> cef_sys::cef_base_ref_counted_t {
    cef_sys::cef_base_ref_counted_t {
        size: 0,
        add_ref: None,
        release: None,
        has_one_ref: None,
        has_at_least_one_ref: None,
    }
}

// pub trait CefWithBase {
//   fn get_base(&self) -> &'static mut cef_sys::cef_base_ref_counted_t;
// }

#[repr(C)]
pub struct CefRefCounted<T> {
    // Cef objects typically have a 'base' member which describes the structure's size in memory
    // followed by reference counting implementation methods. base is always the first member in the object's
    // v-table. We (CefRefCounted) provides the reference counting implmentation.
    // base: cef_sys::cef_base_ref_counted_t,

    // Next up are the cef object's remaining v-table methods.
    sys: T,

    // The base and sys members above have fully completed the native v-table that Cef expects. It is safe
    // to add further members below here. We store the reference count in such a member. Cef doesn't know
    // about this directly, it is used to fulfill the reference counting implementation which Cef *does*
    // directly interact with.
    refs: AtomicUsize,
}

impl<T> CefRefCounted<T> {
    pub fn new(sys: T) -> CefRefCounted<T> {
        unsafe {
            let base: *mut cef_sys::cef_base_ref_counted_t = mem::transmute(&sys);
            (*base).size = mem::size_of::<T>();
            (*base).add_ref = Some(CefRefCounted::<T>::add_ref);
            (*base).release = Some(CefRefCounted::<T>::release);
            (*base).has_one_ref = Some(CefRefCounted::<T>::has_one_ref);
            (*base).has_at_least_one_ref = Some(CefRefCounted::<T>::has_at_least_one_ref);
        }

        CefRefCounted::<T> {
            sys,

            refs: AtomicUsize::new(1),
        }
    }

    fn from_raw(self_: *mut cef_sys::cef_base_ref_counted_t) -> *mut CefRefCounted<T> {
        unsafe {
            let v: *mut CefRefCounted<T> = mem::transmute(self_);
            v
        }
    }

    // Help from https://github.com/dylanede/cef-rs/blob/e9adc70485f592d0783ef65af6abc9a38bf049e0/src/lib.rs#L129

    ///
    // Called to increment the reference count for the object. Should be called
    // for every new copy of a pointer to a given object.
    ///
    unsafe extern "C" fn add_ref(self_: *mut cef_sys::cef_base_ref_counted_t) {
        let v = CefRefCounted::<T>::from_raw(self_);
        (*v).refs.fetch_add(1, Ordering::Relaxed);
        // println!("add_ref {:?}", (*v).refs);
    }

    ///
    // Called to decrement the reference count for the object. If the reference
    // count falls to 0 the object should self-delete. Returns true (1) if the
    // resulting reference count is 0.
    ///
    unsafe extern "C" fn release(self_: *mut cef_sys::cef_base_ref_counted_t) -> raw::c_int {
        let v = CefRefCounted::<T>::from_raw(self_);
        let old_count = (*v).refs.fetch_sub(1, Ordering::Release);

        if old_count == 1 {
            println!("release called. Dropping cef object");
            atomic::fence(Ordering::Acquire);
            let v: Box<CefRefCounted<T>> = mem::transmute(v);
            drop(v);
        }
        // println!("release {:?}", (*v).refs);
        if old_count == 1 {
            1
        } else {
            0
        }
    }

    ///
    // Returns true (1) if the current reference count is 1.
    ///
    unsafe extern "C" fn has_one_ref(self_: *mut cef_sys::cef_base_ref_counted_t) -> raw::c_int {
        if (*CefRefCounted::<T>::from_raw(self_))
            .refs
            .load(Ordering::SeqCst)
            == 1
        {
            1
        } else {
            0
        }
    }

    ///
    // Returns true (1) if the current reference count is at least 1.
    ///
    unsafe extern "C" fn has_at_least_one_ref(
        self_: *mut cef_sys::cef_base_ref_counted_t,
    ) -> raw::c_int {
        if (*CefRefCounted::<T>::from_raw(self_))
            .refs
            .load(Ordering::SeqCst)
            >= 1
        {
            1
        } else {
            0
        }
    }
}

// use std::mem::transmute;
// use std::ops;

// #[repr(C)]
// // #[unsafe_no_drop_flag]
// pub struct CefRc<T: Is<cef_sys::cef_base_ref_counted_t>> {
//   inner: *mut T
// }

// pub unsafe trait Is<T> {}
// pub unsafe trait Interface<T> {}

// unsafe impl Is<cef_sys::cef_base_ref_counted_t> for cef_sys::cef_base_ref_counted_t {}
// trait CefBase2 : Is<cef_sys::cef_base_ref_counted_t> {
//   fn add_ref(&mut self);
//   fn release(&mut self) -> libc::c_int;
// }

// impl<T: Is<cef_sys::cef_base_ref_counted_t>> CefBase2 for T {
//   fn add_ref(&mut self) {
//       let base: &mut cef_sys::cef_base_ref_counted_t = upcast_mut(self);
//       unsafe { base.add_ref.unwrap()(base as *mut _) }
//   }
//   fn release(&mut self) -> libc::c_int {
//       let base: &mut cef_sys::cef_base_ref_counted_t = upcast_mut(self);
//       unsafe { base.release.unwrap()(base as *mut _) }
//   }
// }

// impl<T: Is<cef_sys::cef_base_ref_counted_t>> CefRc<T> {
//   fn make<F: FnOnce(cef_sys::cef_base_ref_counted_t) -> T>(f: F) -> CefRc<T> {
//       use std::mem::size_of;
//       use std::sync::atomic::AtomicUsize;
//       use std::sync::atomic::Ordering;
//       use std::sync::atomic;

//       //println!("making {:?}", size_of::<T>());
//       #[repr(C)]
//       struct RefCounted<T> {
//           v: T,
//           count: AtomicUsize
//       }
//       unsafe impl<T> Is<cef_sys::cef_base_ref_counted_t> for RefCounted<T> {}

//       // #[stdcall_win]
//       extern "C" fn add_ref<T>(_self: *mut cef_sys::cef_base_ref_counted_t) {
//           //println!("add {:?}", size_of::<T>());
//           let cell: &mut RefCounted<T> = unsafe{ unsafe_downcast_mut(&mut *_self) };
//           cell.count.fetch_add(1, Ordering::Relaxed);
//       }
//       // #[stdcall_win]
//       extern "C" fn release<T>(_self: *mut cef_sys::cef_base_ref_counted_t) -> libc::c_int {
//           //println!("release {:?}", size_of::<T>());
//           unsafe {
//               let cell: *mut RefCounted<T> = transmute(_self);
//               let old_count = (*cell).count.fetch_sub(1, Ordering::Release);
//               if old_count == 1 {
//                   //println!("dropping {:?}", size_of::<T>());
//                   atomic::fence(Ordering::Acquire);
//                   let cell: Box<RefCounted<T>> = transmute(cell);
//                   drop(cell);
//               }
//               if old_count == 1 { 1 } else { 0 }
//           }
//       }
//       // #[stdcall_win]
//       extern "C" fn has_one_ref<T>(_self: *mut cef_sys::cef_base_ref_counted_t) -> libc::c_int {
//           let cell: &mut RefCounted<T> = unsafe{ unsafe_downcast_mut(&mut *_self) };
//           if cell.count.load(Ordering::SeqCst) == 1 { 1 } else { 0 }
//       }
//       extern "C" fn has_at_least_one_ref<T>(_self: *mut cef_sys::cef_base_ref_counted_t) -> libc::c_int {
//           let cell: &mut RefCounted<T> = unsafe{ unsafe_downcast_mut(&mut *_self) };
//           if cell.count.load(Ordering::SeqCst) >= 1 { 1 } else { 0 }
//       }
//       CefRc {
//           inner: unsafe { transmute(Box::new(RefCounted {
//               v: f(cef_sys::cef_base_ref_counted_t {
//                   size: size_of::<RefCounted<T>>() as libc::size_t,
//                   add_ref: Some(add_ref::<T>),
//                   release: Some(release::<T>),
//                   has_one_ref: Some(has_one_ref::<T>),
//                   has_at_least_one_ref: Some(has_at_least_one_ref::<T>)
//               }),
//               count: AtomicUsize::new(1)
//           }))}
//       }
//   }
//   pub fn from_existing(ptr: *mut T) -> CefRc<T> {
//       CefRc { inner: ptr }
//   }
// }

// impl<T: Is<cef_sys::cef_base_ref_counted_t>> ops::Deref for CefRc<T> {
//   type Target = T;

//   fn deref<'a>(&'a self) -> &'a T {
//       unsafe{ &*self.inner }
//   }
// }

// impl<T: Is<cef_sys::cef_base_ref_counted_t>> ops::DerefMut for CefRc<T> {
//   fn deref_mut<'a>(&'a mut self) -> &'a mut T {
//       unsafe{ &mut *self.inner }
//   }
// }

// unsafe fn unsafe_downcast_mut<'a, T1, T2 : Is<T1>>(x: &'a mut T1) -> &'a mut T2 {
//   transmute(x)
// }
// fn upcast_mut<'a, T1 : Is<T2>, T2>(x: &'a mut T1) -> &'a mut T2 {
//   unsafe{ transmute(x) }
// }
// fn upcast<'a, T1 : Is<T2>, T2>(x: &'a T1) -> &'a T2 {
//   unsafe{ transmute(x) }
// }

// fn upcast_ptr<T1 : Is<T2>, T2>(x: CefRc<T1>) -> *mut T2 where T1 : Is<cef_sys::cef_base_ref_counted_t> {
//   unsafe { transmute(x) }
// }

// unsafe fn unsafe_downcast_ptr<T1, T2 : Is<T1>>(x: *mut T1) -> CefRc<T2> where T2 : Is<cef_sys::cef_base_ref_counted_t> {
//   transmute(x)
// }

// fn cast_ref<'a, T1, T2 : Interface<T1>>(x: &'a T1) -> &'a T2 {
//   unsafe{ transmute(x) }
// }

// fn cast_mut_ref<'a, T1, T2 : Interface<T1>>(x: &'a mut T1) -> &'a mut T2 {
//   unsafe{ transmute(x) }
// }

// fn cast_to_interface<T1, T2 : Interface<T1>>(x: *mut T1) -> CefRc<T2> where T2 : Is<cef_sys::cef_base_ref_counted_t> {
//   unsafe{ transmute(x) }
// }

// fn cast_from_interface<T1, T2 : Interface<T1>>(x: CefRc<T2>) -> *mut T1 where T2 : Is<cef_sys::cef_base_ref_counted_t> {
//   unsafe { transmute(x) }
// }
