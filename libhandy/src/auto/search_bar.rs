// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v0_0_6", feature = "dox"))]
use gdk;
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

glib_wrapper! {
    pub struct SearchBar(Object<handy_sys::HdySearchBar, handy_sys::HdySearchBarClass, SearchBarClass>) @extends gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        get_type => || handy_sys::hdy_search_bar_get_type(),
    }
}

impl SearchBar {
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn new() -> SearchBar {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(handy_sys::hdy_search_bar_new()).unsafe_cast()
        }
    }
}

#[cfg(any(feature = "v0_0_6", feature = "dox"))]
impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SEARCH_BAR: Option<&SearchBar> = None;

pub trait SearchBarExt: 'static {
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn connect_entry<P: IsA<gtk::Entry>>(&self, entry: &P);

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn get_search_mode(&self) -> bool;

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn get_show_close_button(&self) -> bool;

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn handle_event(&self, event: &mut gdk::Event) -> bool;

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn set_search_mode(&self, search_mode: bool);

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn set_show_close_button(&self, visible: bool);

    fn get_property_search_mode_enabled(&self) -> bool;

    fn set_property_search_mode_enabled(&self, search_mode_enabled: bool);

    fn get_property_show_close_button(&self) -> bool;

    fn set_property_show_close_button(&self, show_close_button: bool);

    fn connect_property_search_mode_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SearchBar>> SearchBarExt for O {
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn connect_entry<P: IsA<gtk::Entry>>(&self, entry: &P) {
        unsafe {
            handy_sys::hdy_search_bar_connect_entry(self.as_ref().to_glib_none().0, entry.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn get_search_mode(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_search_bar_get_search_mode(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_search_bar_get_show_close_button(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn handle_event(&self, event: &mut gdk::Event) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_search_bar_handle_event(self.as_ref().to_glib_none().0, event.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn set_search_mode(&self, search_mode: bool) {
        unsafe {
            handy_sys::hdy_search_bar_set_search_mode(self.as_ref().to_glib_none().0, search_mode.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn set_show_close_button(&self, visible: bool) {
        unsafe {
            handy_sys::hdy_search_bar_set_show_close_button(self.as_ref().to_glib_none().0, visible.to_glib());
        }
    }

    fn get_property_search_mode_enabled(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"search-mode-enabled\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `search-mode-enabled` getter").unwrap()
        }
    }

    fn set_property_search_mode_enabled(&self, search_mode_enabled: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"search-mode-enabled\0".as_ptr() as *const _, Value::from(&search_mode_enabled).to_glib_none().0);
        }
    }

    fn get_property_show_close_button(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-close-button\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `show-close-button` getter").unwrap()
        }
    }

    fn set_property_show_close_button(&self, show_close_button: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-close-button\0".as_ptr() as *const _, Value::from(&show_close_button).to_glib_none().0);
        }
    }

    fn connect_property_search_mode_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_mode_enabled_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdySearchBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SearchBar>
        {
            let f: &F = &*(f as *const F);
            f(&SearchBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::search-mode-enabled\0".as_ptr() as *const _,
                Some(transmute(notify_search_mode_enabled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_close_button_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdySearchBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SearchBar>
        {
            let f: &F = &*(f as *const F);
            f(&SearchBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-close-button\0".as_ptr() as *const _,
                Some(transmute(notify_show_close_button_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for SearchBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SearchBar")
    }
}
