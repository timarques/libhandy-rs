// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
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
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DialerButton(Object<ffi::HdyDialerButton, ffi::HdyDialerButtonClass>): [
        gtk::Widget => gtk_ffi::GtkWidget,
    ];

    match fn {
        get_type => || ffi::hdy_dialer_button_get_type(),
    }
}

impl DialerButton {
    pub fn new<'a, P: Into<Option<&'a str>>>(digit: i32, letters: P) -> DialerButton {
        assert_initialized_main_thread!();
        let letters = letters.into();
        let letters = letters.to_glib_none();
        unsafe {
            gtk::Widget::from_glib_none(ffi::hdy_dialer_button_new(digit, letters.0)).downcast_unchecked()
        }
    }
}

pub trait DialerButtonExt {
    fn get_digit(&self) -> i32;

    fn get_letters(&self) -> Option<String>;

    fn set_property_digit(&self, digit: i32);

    fn set_property_letters(&self, letters: Option<&str>);

    fn connect_property_digit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_letters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DialerButton> + IsA<glib::object::Object>> DialerButtonExt for O {
    fn get_digit(&self) -> i32 {
        unsafe {
            ffi::hdy_dialer_button_get_digit(self.to_glib_none().0)
        }
    }

    fn get_letters(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::hdy_dialer_button_get_letters(self.to_glib_none().0))
        }
    }

    fn set_property_digit(&self, digit: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "digit".to_glib_none().0, Value::from(&digit).to_glib_none().0);
        }
    }

    fn set_property_letters(&self, letters: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "letters".to_glib_none().0, Value::from(letters).to_glib_none().0);
        }
    }

    fn connect_property_digit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::digit",
                transmute(notify_digit_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_letters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::letters",
                transmute(notify_letters_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_digit_trampoline<P>(this: *mut ffi::HdyDialerButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DialerButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DialerButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_letters_trampoline<P>(this: *mut ffi::HdyDialerButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DialerButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DialerButton::from_glib_borrow(this).downcast_unchecked())
}
