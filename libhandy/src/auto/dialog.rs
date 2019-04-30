// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gtk;
use std::fmt;

glib_wrapper! {
    pub struct Dialog(Object<ffi::HdyDialog, ffi::HdyDialogClass, DialogClass>) @extends gtk::Dialog, gtk::Window, gtk::Container, gtk::Widget;

    match fn {
        get_type => || ffi::hdy_dialog_get_type(),
    }
}

impl Dialog {
    #[cfg(any(feature = "v0_0_7", feature = "dox"))]
    pub fn new<P: IsA<gtk::Window>>(parent: &P) -> Dialog {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::hdy_dialog_new(parent.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_DIALOG: Option<&Dialog> = None;

impl fmt::Display for Dialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dialog")
    }
}
