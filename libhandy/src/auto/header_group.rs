// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk;
use handy_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct HeaderGroup(Object<handy_sys::HdyHeaderGroup, handy_sys::HdyHeaderGroupClass, HeaderGroupClass>);

    match fn {
        get_type => || handy_sys::hdy_header_group_get_type(),
    }
}

impl HeaderGroup {
    pub fn new() -> HeaderGroup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(handy_sys::hdy_header_group_new())
        }
    }
}

impl Default for HeaderGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_HEADER_GROUP: Option<&HeaderGroup> = None;

pub trait HeaderGroupExt: 'static {
    fn add_header_bar<P: IsA<gtk::HeaderBar>>(&self, header_bar: &P);

    fn get_focus(&self) -> Option<gtk::HeaderBar>;

    fn get_header_bars(&self) -> Vec<gtk::HeaderBar>;

    fn remove_header_bar<P: IsA<gtk::HeaderBar>>(&self, header_bar: &P);

    fn set_focus<P: IsA<gtk::HeaderBar>>(&self, header_bar: Option<&P>);

    fn connect_property_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<HeaderGroup>> HeaderGroupExt for O {
    fn add_header_bar<P: IsA<gtk::HeaderBar>>(&self, header_bar: &P) {
        unsafe {
            handy_sys::hdy_header_group_add_header_bar(self.as_ref().to_glib_none().0, header_bar.as_ref().to_glib_none().0);
        }
    }

    fn get_focus(&self) -> Option<gtk::HeaderBar> {
        unsafe {
            from_glib_none(handy_sys::hdy_header_group_get_focus(self.as_ref().to_glib_none().0))
        }
    }

    fn get_header_bars(&self) -> Vec<gtk::HeaderBar> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(handy_sys::hdy_header_group_get_header_bars(self.as_ref().to_glib_none().0))
        }
    }

    fn remove_header_bar<P: IsA<gtk::HeaderBar>>(&self, header_bar: &P) {
        unsafe {
            handy_sys::hdy_header_group_remove_header_bar(self.as_ref().to_glib_none().0, header_bar.as_ref().to_glib_none().0);
        }
    }

    fn set_focus<P: IsA<gtk::HeaderBar>>(&self, header_bar: Option<&P>) {
        unsafe {
            handy_sys::hdy_header_group_set_focus(self.as_ref().to_glib_none().0, header_bar.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn connect_property_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyHeaderGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<HeaderGroup>
        {
            let f: &F = &*(f as *const F);
            f(&HeaderGroup::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::focus\0".as_ptr() as *const _,
                Some(transmute(notify_focus_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for HeaderGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HeaderGroup")
    }
}
