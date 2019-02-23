// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Dialer(Object<ffi::HdyDialer, ffi::HdyDialerClass, DialerClass>) @extends gtk::Widget;

    match fn {
        get_type => || ffi::hdy_dialer_get_type(),
    }
}

impl Dialer {
    pub fn new() -> Dialer {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::hdy_dialer_new()).unsafe_cast()
        }
    }
}

impl Default for Dialer {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DIALER: Option<&Dialer> = None;

pub trait DialerExt: 'static {
    fn clear_number(&self);

    fn get_number(&self) -> Option<GString>;

    //fn get_relief(&self) -> /*Ignored*/gtk::ReliefStyle;

    fn get_show_action_buttons(&self) -> bool;

    fn set_number(&self, number: &str);

    //fn set_relief(&self, relief: /*Ignored*/gtk::ReliefStyle);

    fn set_show_action_buttons(&self, show: bool);

    fn get_property_column_spacing(&self) -> u32;

    fn set_property_column_spacing(&self, column_spacing: u32);

    fn get_property_row_spacing(&self) -> u32;

    fn set_property_row_spacing(&self, row_spacing: u32);

    fn connect_deleted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_submitted<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_symbol_clicked<F: Fn(&Self, glib::Char) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_number_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_action_buttons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Dialer>> DialerExt for O {
    fn clear_number(&self) {
        unsafe {
            ffi::hdy_dialer_clear_number(self.as_ref().to_glib_none().0);
        }
    }

    fn get_number(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::hdy_dialer_get_number(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_relief(&self) -> /*Ignored*/gtk::ReliefStyle {
    //    unsafe { TODO: call ffi::hdy_dialer_get_relief() }
    //}

    fn get_show_action_buttons(&self) -> bool {
        unsafe {
            from_glib(ffi::hdy_dialer_get_show_action_buttons(self.as_ref().to_glib_none().0))
        }
    }

    fn set_number(&self, number: &str) {
        unsafe {
            ffi::hdy_dialer_set_number(self.as_ref().to_glib_none().0, number.to_glib_none().0);
        }
    }

    //fn set_relief(&self, relief: /*Ignored*/gtk::ReliefStyle) {
    //    unsafe { TODO: call ffi::hdy_dialer_set_relief() }
    //}

    fn set_show_action_buttons(&self, show: bool) {
        unsafe {
            ffi::hdy_dialer_set_show_action_buttons(self.as_ref().to_glib_none().0, show.to_glib());
        }
    }

    fn get_property_column_spacing(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"column-spacing\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_column_spacing(&self, column_spacing: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"column-spacing\0".as_ptr() as *const _, Value::from(&column_spacing).to_glib_none().0);
        }
    }

    fn get_property_row_spacing(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"row-spacing\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_row_spacing(&self, row_spacing: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"row-spacing\0".as_ptr() as *const _, Value::from(&row_spacing).to_glib_none().0);
        }
    }

    fn connect_deleted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"deleted\0".as_ptr() as *const _,
                Some(transmute(deleted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_submitted<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"submitted\0".as_ptr() as *const _,
                Some(transmute(submitted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_symbol_clicked<F: Fn(&Self, glib::Char) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"symbol-clicked\0".as_ptr() as *const _,
                Some(transmute(symbol_clicked_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::column-spacing\0".as_ptr() as *const _,
                Some(transmute(notify_column_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_number_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::number\0".as_ptr() as *const _,
                Some(transmute(notify_number_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::relief\0".as_ptr() as *const _,
                Some(transmute(notify_relief_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::row-spacing\0".as_ptr() as *const _,
                Some(transmute(notify_row_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_action_buttons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-action-buttons\0".as_ptr() as *const _,
                Some(transmute(notify_show_action_buttons_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn deleted_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::HdyDialer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &F = transmute(f);
    f(&Dialer::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn submitted_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut ffi::HdyDialer, number: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &F = transmute(f);
    f(&Dialer::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(number))
}

unsafe extern "C" fn symbol_clicked_trampoline<P, F: Fn(&P, glib::Char) + 'static>(this: *mut ffi::HdyDialer, button: libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &F = transmute(f);
    f(&Dialer::from_glib_borrow(this).unsafe_cast(), from_glib(button))
}

unsafe extern "C" fn notify_column_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &F = transmute(f);
    f(&Dialer::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_number_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &F = transmute(f);
    f(&Dialer::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_relief_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &F = transmute(f);
    f(&Dialer::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_row_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &F = transmute(f);
    f(&Dialer::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_action_buttons_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &F = transmute(f);
    f(&Dialer::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Dialer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dialer")
    }
}
