// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ActionRow;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gtk;
use gtk_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ExpanderRow(Object<ffi::HdyExpanderRow, ffi::HdyExpanderRowClass>): [
        ActionRow,
        gtk::Widget => gtk_ffi::GtkWidget,
    ];

    match fn {
        get_type => || ffi::hdy_expander_row_get_type(),
    }
}

impl ExpanderRow {
    pub fn new() -> ExpanderRow {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::hdy_expander_row_new())
        }
    }
}

impl Default for ExpanderRow {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ExpanderRowExt: 'static {
    fn get_enable_expansion(&self) -> bool;

    fn get_show_enable_switch(&self) -> bool;

    fn set_enable_expansion(&self, enable_expansion: bool);

    fn set_show_enable_switch(&self, show_enable_switch: bool);

    fn connect_property_enable_expansion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_enable_switch_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ExpanderRow>> ExpanderRowExt for O {
    fn get_enable_expansion(&self) -> bool {
        unsafe {
            from_glib(ffi::hdy_expander_row_get_enable_expansion(self.to_glib_none().0))
        }
    }

    fn get_show_enable_switch(&self) -> bool {
        unsafe {
            from_glib(ffi::hdy_expander_row_get_show_enable_switch(self.to_glib_none().0))
        }
    }

    fn set_enable_expansion(&self, enable_expansion: bool) {
        unsafe {
            ffi::hdy_expander_row_set_enable_expansion(self.to_glib_none().0, enable_expansion.to_glib());
        }
    }

    fn set_show_enable_switch(&self, show_enable_switch: bool) {
        unsafe {
            ffi::hdy_expander_row_set_show_enable_switch(self.to_glib_none().0, show_enable_switch.to_glib());
        }
    }

    fn connect_property_enable_expansion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::enable-expansion\0".as_ptr() as *const _,
                transmute(notify_enable_expansion_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_enable_switch_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::show-enable-switch\0".as_ptr() as *const _,
                transmute(notify_show_enable_switch_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_enable_expansion_trampoline<P>(this: *mut ffi::HdyExpanderRow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ExpanderRow> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ExpanderRow::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_enable_switch_trampoline<P>(this: *mut ffi::HdyExpanderRow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ExpanderRow> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ExpanderRow::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for ExpanderRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExpanderRow")
    }
}
