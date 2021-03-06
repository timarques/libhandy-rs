// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk;
use handy_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use SqueezerTransitionType;

glib_wrapper! {
    pub struct Squeezer(Object<handy_sys::HdySqueezer, handy_sys::HdySqueezerClass, SqueezerClass>) @extends gtk::Container, gtk::Widget, @implements gtk::Orientable;

    match fn {
        get_type => || handy_sys::hdy_squeezer_get_type(),
    }
}

impl Squeezer {
    pub fn new() -> Squeezer {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(handy_sys::hdy_squeezer_new())
        }
    }
}

impl Default for Squeezer {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SQUEEZER: Option<&Squeezer> = None;

pub trait SqueezerExt: 'static {
    fn get_child_enabled<P: IsA<gtk::Widget>>(&self, child: &P) -> bool;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_homogeneous(&self) -> bool;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_interpolate_size(&self) -> bool;

    fn get_transition_duration(&self) -> u32;

    fn get_transition_running(&self) -> bool;

    fn get_transition_type(&self) -> SqueezerTransitionType;

    fn get_visible_child(&self) -> Option<gtk::Widget>;

    fn set_child_enabled<P: IsA<gtk::Widget>>(&self, child: &P, enabled: bool);

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_homogeneous(&self, homogeneous: bool);

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_interpolate_size(&self, interpolate_size: bool);

    fn set_transition_duration(&self, duration: u32);

    fn set_transition_type(&self, transition: SqueezerTransitionType);

    fn get_property_homogeneous(&self) -> bool;

    fn set_property_homogeneous(&self, homogeneous: bool);

    fn get_property_interpolate_size(&self) -> bool;

    fn set_property_interpolate_size(&self, interpolate_size: bool);

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_running_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Squeezer>> SqueezerExt for O {
    fn get_child_enabled<P: IsA<gtk::Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_squeezer_get_child_enabled(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_squeezer_get_homogeneous(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_interpolate_size(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_squeezer_get_interpolate_size(self.as_ref().to_glib_none().0))
        }
    }

    fn get_transition_duration(&self) -> u32 {
        unsafe {
            handy_sys::hdy_squeezer_get_transition_duration(self.as_ref().to_glib_none().0)
        }
    }

    fn get_transition_running(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_squeezer_get_transition_running(self.as_ref().to_glib_none().0))
        }
    }

    fn get_transition_type(&self) -> SqueezerTransitionType {
        unsafe {
            from_glib(handy_sys::hdy_squeezer_get_transition_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_visible_child(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(handy_sys::hdy_squeezer_get_visible_child(self.as_ref().to_glib_none().0))
        }
    }

    fn set_child_enabled<P: IsA<gtk::Widget>>(&self, child: &P, enabled: bool) {
        unsafe {
            handy_sys::hdy_squeezer_set_child_enabled(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, enabled.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            handy_sys::hdy_squeezer_set_homogeneous(self.as_ref().to_glib_none().0, homogeneous.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            handy_sys::hdy_squeezer_set_interpolate_size(self.as_ref().to_glib_none().0, interpolate_size.to_glib());
        }
    }

    fn set_transition_duration(&self, duration: u32) {
        unsafe {
            handy_sys::hdy_squeezer_set_transition_duration(self.as_ref().to_glib_none().0, duration);
        }
    }

    fn set_transition_type(&self, transition: SqueezerTransitionType) {
        unsafe {
            handy_sys::hdy_squeezer_set_transition_type(self.as_ref().to_glib_none().0, transition.to_glib());
        }
    }

    fn get_property_homogeneous(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"homogeneous\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `homogeneous` getter").unwrap()
        }
    }

    fn set_property_homogeneous(&self, homogeneous: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"homogeneous\0".as_ptr() as *const _, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    fn get_property_interpolate_size(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"interpolate-size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `interpolate-size` getter").unwrap()
        }
    }

    fn set_property_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"interpolate-size\0".as_ptr() as *const _, Value::from(&interpolate_size).to_glib_none().0);
        }
    }

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdySqueezer, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Squeezer>
        {
            let f: &F = &*(f as *const F);
            f(&Squeezer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::homogeneous\0".as_ptr() as *const _,
                Some(transmute(notify_homogeneous_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interpolate_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdySqueezer, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Squeezer>
        {
            let f: &F = &*(f as *const F);
            f(&Squeezer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::interpolate-size\0".as_ptr() as *const _,
                Some(transmute(notify_interpolate_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_duration_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdySqueezer, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Squeezer>
        {
            let f: &F = &*(f as *const F);
            f(&Squeezer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::transition-duration\0".as_ptr() as *const _,
                Some(transmute(notify_transition_duration_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_transition_running_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_running_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdySqueezer, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Squeezer>
        {
            let f: &F = &*(f as *const F);
            f(&Squeezer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::transition-running\0".as_ptr() as *const _,
                Some(transmute(notify_transition_running_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdySqueezer, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Squeezer>
        {
            let f: &F = &*(f as *const F);
            f(&Squeezer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::transition-type\0".as_ptr() as *const _,
                Some(transmute(notify_transition_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdySqueezer, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Squeezer>
        {
            let f: &F = &*(f as *const F);
            f(&Squeezer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible-child\0".as_ptr() as *const _,
                Some(transmute(notify_visible_child_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Squeezer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Squeezer")
    }
}
