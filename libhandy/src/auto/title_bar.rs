// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gtk;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct TitleBar(Object<ffi::HdyTitleBar, ffi::HdyTitleBarClass, TitleBarClass>) @extends gtk::Widget;

    match fn {
        get_type => || ffi::hdy_title_bar_get_type(),
    }
}

impl TitleBar {
    pub fn new() -> TitleBar {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::hdy_title_bar_new())
        }
    }
}

impl Default for TitleBar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TITLE_BAR: Option<&TitleBar> = None;

pub trait TitleBarExt: 'static {
    fn get_selection_mode(&self) -> bool;

    fn set_selection_mode(&self, selection_mode: bool);

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TitleBar>> TitleBarExt for O {
    fn get_selection_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::hdy_title_bar_get_selection_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn set_selection_mode(&self, selection_mode: bool) {
        unsafe {
            ffi::hdy_title_bar_set_selection_mode(self.as_ref().to_glib_none().0, selection_mode.to_glib());
        }
    }

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::selection-mode\0".as_ptr() as *const _,
                Some(transmute(notify_selection_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_selection_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::HdyTitleBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TitleBar> {
    let f: &F = transmute(f);
    f(&TitleBar::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for TitleBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TitleBar")
    }
}
