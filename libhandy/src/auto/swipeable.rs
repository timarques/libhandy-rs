// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v0_0_12", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v0_0_12", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v0_0_12", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v0_0_12", feature = "dox"))]
use glib_sys;
use gtk;
use handy_sys;
#[cfg(any(feature = "v0_0_12", feature = "dox"))]
use libc;
#[cfg(any(feature = "v0_0_12", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v0_0_12", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct Swipeable(Interface<handy_sys::HdySwipeable>) @requires gtk::Widget;

    match fn {
        get_type => || handy_sys::hdy_swipeable_get_type(),
    }
}

pub const NONE_SWIPEABLE: Option<&Swipeable> = None;

pub trait SwipeableExt: 'static {
    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_begin_swipe<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_end_swipe<F: Fn(&Self, i64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_switch_child<F: Fn(&Self, u32, i64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_update_swipe<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Swipeable>> SwipeableExt for O {
    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_begin_swipe<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn begin_swipe_trampoline<P, F: Fn(&P, i32) + 'static>(this: *mut handy_sys::HdySwipeable, direction: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<Swipeable>
        {
            let f: &F = &*(f as *const F);
            f(&Swipeable::from_glib_borrow(this).unsafe_cast(), direction)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"begin-swipe\0".as_ptr() as *const _,
                Some(transmute(begin_swipe_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_end_swipe<F: Fn(&Self, i64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn end_swipe_trampoline<P, F: Fn(&P, i64, f64) + 'static>(this: *mut handy_sys::HdySwipeable, duration: i64, to: libc::c_double, f: glib_sys::gpointer)
            where P: IsA<Swipeable>
        {
            let f: &F = &*(f as *const F);
            f(&Swipeable::from_glib_borrow(this).unsafe_cast(), duration, to)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"end-swipe\0".as_ptr() as *const _,
                Some(transmute(end_swipe_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_switch_child<F: Fn(&Self, u32, i64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn switch_child_trampoline<P, F: Fn(&P, u32, i64) + 'static>(this: *mut handy_sys::HdySwipeable, index: libc::c_uint, duration: i64, f: glib_sys::gpointer)
            where P: IsA<Swipeable>
        {
            let f: &F = &*(f as *const F);
            f(&Swipeable::from_glib_borrow(this).unsafe_cast(), index, duration)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"switch-child\0".as_ptr() as *const _,
                Some(transmute(switch_child_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_update_swipe<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn update_swipe_trampoline<P, F: Fn(&P, f64) + 'static>(this: *mut handy_sys::HdySwipeable, value: libc::c_double, f: glib_sys::gpointer)
            where P: IsA<Swipeable>
        {
            let f: &F = &*(f as *const F);
            f(&Swipeable::from_glib_borrow(this).unsafe_cast(), value)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"update-swipe\0".as_ptr() as *const _,
                Some(transmute(update_swipe_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Swipeable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Swipeable")
    }
}
