// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Dialer(Object<ffi::HdyDialer, ffi::HdyDialerClass>): [
        gtk::Widget => gtk_ffi::GtkWidget,
    ];

    match fn {
        get_type => || ffi::hdy_dialer_get_type(),
    }
}

impl Dialer {
    pub fn new() -> Dialer {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::hdy_dialer_new()).downcast_unchecked()
        }
    }
}

impl Default for Dialer {
    fn default() -> Self {
        Self::new()
    }
}

pub trait DialerExt {
    fn clear_number(&self);

    fn get_number(&self) -> Option<String>;

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

impl<O: IsA<Dialer> + IsA<glib::object::Object>> DialerExt for O {
    fn clear_number(&self) {
        unsafe {
            ffi::hdy_dialer_clear_number(self.to_glib_none().0);
        }
    }

    fn get_number(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::hdy_dialer_get_number(self.to_glib_none().0))
        }
    }

    //fn get_relief(&self) -> /*Ignored*/gtk::ReliefStyle {
    //    unsafe { TODO: call ffi::hdy_dialer_get_relief() }
    //}

    fn get_show_action_buttons(&self) -> bool {
        unsafe {
            from_glib(ffi::hdy_dialer_get_show_action_buttons(self.to_glib_none().0))
        }
    }

    fn set_number(&self, number: &str) {
        unsafe {
            ffi::hdy_dialer_set_number(self.to_glib_none().0, number.to_glib_none().0);
        }
    }

    //fn set_relief(&self, relief: /*Ignored*/gtk::ReliefStyle) {
    //    unsafe { TODO: call ffi::hdy_dialer_set_relief() }
    //}

    fn set_show_action_buttons(&self, show: bool) {
        unsafe {
            ffi::hdy_dialer_set_show_action_buttons(self.to_glib_none().0, show.to_glib());
        }
    }

    fn get_property_column_spacing(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "column-spacing".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_column_spacing(&self, column_spacing: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "column-spacing".to_glib_none().0, Value::from(&column_spacing).to_glib_none().0);
        }
    }

    fn get_property_row_spacing(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "row-spacing".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_row_spacing(&self, row_spacing: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "row-spacing".to_glib_none().0, Value::from(&row_spacing).to_glib_none().0);
        }
    }

    fn connect_deleted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "deleted",
                transmute(deleted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_submitted<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "submitted",
                transmute(submitted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_symbol_clicked<F: Fn(&Self, glib::Char) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, glib::Char) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "symbol-clicked",
                transmute(symbol_clicked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::column-spacing",
                transmute(notify_column_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_number_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::number",
                transmute(notify_number_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::relief",
                transmute(notify_relief_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::row-spacing",
                transmute(notify_row_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_action_buttons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-action-buttons",
                transmute(notify_show_action_buttons_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn deleted_trampoline<P>(this: *mut ffi::HdyDialer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Dialer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn submitted_trampoline<P>(this: *mut ffi::HdyDialer, number: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&Dialer::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(number))
}

unsafe extern "C" fn symbol_clicked_trampoline<P>(this: *mut ffi::HdyDialer, button: libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &&(Fn(&P, glib::Char) + 'static) = transmute(f);
    f(&Dialer::from_glib_borrow(this).downcast_unchecked(), from_glib(button))
}

unsafe extern "C" fn notify_column_spacing_trampoline<P>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Dialer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_number_trampoline<P>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Dialer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_relief_trampoline<P>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Dialer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_row_spacing_trampoline<P>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Dialer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_action_buttons_trampoline<P>(this: *mut ffi::HdyDialer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialer> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Dialer::from_glib_borrow(this).downcast_unchecked())
}