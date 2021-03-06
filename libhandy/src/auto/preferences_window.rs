// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gtk;
use handy_sys;
use std::fmt;

glib_wrapper! {
    pub struct PreferencesWindow(Object<handy_sys::HdyPreferencesWindow, handy_sys::HdyPreferencesWindowClass, PreferencesWindowClass>) @extends gtk::Window, gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        get_type => || handy_sys::hdy_preferences_window_get_type(),
    }
}

impl PreferencesWindow {
    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    pub fn new() -> PreferencesWindow {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(handy_sys::hdy_preferences_window_new())
        }
    }
}

#[cfg(any(feature = "v0_0_10", feature = "dox"))]
impl Default for PreferencesWindow {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PREFERENCES_WINDOW: Option<&PreferencesWindow> = None;

impl fmt::Display for PreferencesWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PreferencesWindow")
    }
}
