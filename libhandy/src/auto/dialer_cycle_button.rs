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
use DialerButton;

glib_wrapper! {
    pub struct DialerCycleButton(Object<handy_sys::HdyDialerCycleButton, handy_sys::HdyDialerCycleButtonClass, DialerCycleButtonClass>) @extends DialerButton, gtk::Button, gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        get_type => || handy_sys::hdy_dialer_cycle_button_get_type(),
    }
}

impl DialerCycleButton {
    #[cfg_attr(feature = "v0_0_12", deprecated)]
    pub fn new(symbols: &str) -> DialerCycleButton {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(handy_sys::hdy_dialer_cycle_button_new(symbols.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_DIALER_CYCLE_BUTTON: Option<&DialerCycleButton> = None;

pub trait DialerCycleButtonExt: 'static {
    #[cfg_attr(feature = "v0_0_12", deprecated)]
    fn get_current_symbol(&self) -> char;

    #[cfg_attr(feature = "v0_0_12", deprecated)]
    fn get_cycle_timeout(&self) -> i32;

    #[cfg_attr(feature = "v0_0_12", deprecated)]
    fn is_cycling(&self) -> bool;

    #[cfg_attr(feature = "v0_0_12", deprecated)]
    fn set_cycle_timeout(&self, timeout: i32);

    #[cfg_attr(feature = "v0_0_12", deprecated)]
    fn stop_cycle(&self);

    #[cfg_attr(feature = "v0_0_12", deprecated)]
    fn connect_cycle_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v0_0_12", deprecated)]
    fn connect_cycle_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cycle_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DialerCycleButton>> DialerCycleButtonExt for O {
    fn get_current_symbol(&self) -> char {
        unsafe {
            from_glib(handy_sys::hdy_dialer_cycle_button_get_current_symbol(self.as_ref().to_glib_none().0))
        }
    }

    fn get_cycle_timeout(&self) -> i32 {
        unsafe {
            handy_sys::hdy_dialer_cycle_button_get_cycle_timeout(self.as_ref().to_glib_none().0)
        }
    }

    fn is_cycling(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_dialer_cycle_button_is_cycling(self.as_ref().to_glib_none().0))
        }
    }

    fn set_cycle_timeout(&self, timeout: i32) {
        unsafe {
            handy_sys::hdy_dialer_cycle_button_set_cycle_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    fn stop_cycle(&self) {
        unsafe {
            handy_sys::hdy_dialer_cycle_button_stop_cycle(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_cycle_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cycle_end_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyDialerCycleButton, f: glib_sys::gpointer)
            where P: IsA<DialerCycleButton>
        {
            let f: &F = &*(f as *const F);
            f(&DialerCycleButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cycle-end\0".as_ptr() as *const _,
                Some(transmute(cycle_end_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_cycle_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cycle_start_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyDialerCycleButton, f: glib_sys::gpointer)
            where P: IsA<DialerCycleButton>
        {
            let f: &F = &*(f as *const F);
            f(&DialerCycleButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cycle-start\0".as_ptr() as *const _,
                Some(transmute(cycle_start_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_cycle_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cycle_timeout_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyDialerCycleButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DialerCycleButton>
        {
            let f: &F = &*(f as *const F);
            f(&DialerCycleButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::cycle-timeout\0".as_ptr() as *const _,
                Some(transmute(notify_cycle_timeout_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DialerCycleButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DialerCycleButton")
    }
}
