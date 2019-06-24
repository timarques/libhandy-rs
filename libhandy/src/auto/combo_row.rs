// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ActionRow;
use PreferencesRow;
#[cfg(any(feature = "v0_0_6", feature = "dox"))]
use gio;
#[cfg(any(feature = "v0_0_7", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v0_0_7", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v0_0_7", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v0_0_7", feature = "dox"))]
use glib_sys;
use gtk;
use handy_sys;
#[cfg(any(feature = "v0_0_7", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v0_0_7", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct ComboRow(Object<handy_sys::HdyComboRow, handy_sys::HdyComboRowClass, ComboRowClass>) @extends ActionRow, PreferencesRow, gtk::ListBoxRow, gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        get_type => || handy_sys::hdy_combo_row_get_type(),
    }
}

impl ComboRow {
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn new() -> ComboRow {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(handy_sys::hdy_combo_row_new())
        }
    }
}

#[cfg(any(feature = "v0_0_6", feature = "dox"))]
impl Default for ComboRow {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_COMBO_ROW: Option<&ComboRow> = None;

pub trait ComboRowExt: 'static {
    //#[cfg(any(feature = "v0_0_6", feature = "dox"))]
    //fn bind_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>, create_list_widget_func: /*Unimplemented*/FnMut(/*Ignored*/glib::Object) -> gtk::Widget, create_current_widget_func: /*Unimplemented*/Fn(/*Ignored*/glib::Object) -> gtk::Widget, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    //#[cfg(any(feature = "v0_0_6", feature = "dox"))]
    //fn bind_name_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>, get_name_func: /*Unimplemented*/Fn(/*Ignored*/glib::Object) -> String, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn get_model(&self) -> Option<gio::ListModel>;

    #[cfg(any(feature = "v0_0_7", feature = "dox"))]
    fn get_selected_index(&self) -> i32;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_use_subtitle(&self) -> bool;

    //#[cfg(any(feature = "v0_0_6", feature = "dox"))]
    //fn set_for_enum(&self, enum_type: glib::types::Type, get_name_func: /*Unimplemented*/Fn(/*Ignored*/EnumValueObject) -> String, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    //#[cfg(any(feature = "v0_0_10", feature = "dox"))]
    //fn set_get_name_func(&self, get_name_func: /*Unimplemented*/Fn(/*Ignored*/glib::Object) -> String, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    #[cfg(any(feature = "v0_0_7", feature = "dox"))]
    fn set_selected_index(&self, selected_index: i32);

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_use_subtitle(&self, use_subtitle: bool);

    #[cfg(any(feature = "v0_0_7", feature = "dox"))]
    fn connect_property_selected_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_use_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ComboRow>> ComboRowExt for O {
    //#[cfg(any(feature = "v0_0_6", feature = "dox"))]
    //fn bind_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>, create_list_widget_func: /*Unimplemented*/FnMut(/*Ignored*/glib::Object) -> gtk::Widget, create_current_widget_func: /*Unimplemented*/Fn(/*Ignored*/glib::Object) -> gtk::Widget, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call handy_sys:hdy_combo_row_bind_model() }
    //}

    //#[cfg(any(feature = "v0_0_6", feature = "dox"))]
    //fn bind_name_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>, get_name_func: /*Unimplemented*/Fn(/*Ignored*/glib::Object) -> String, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call handy_sys:hdy_combo_row_bind_name_model() }
    //}

    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    fn get_model(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_none(handy_sys::hdy_combo_row_get_model(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_7", feature = "dox"))]
    fn get_selected_index(&self) -> i32 {
        unsafe {
            handy_sys::hdy_combo_row_get_selected_index(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_use_subtitle(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_combo_row_get_use_subtitle(self.as_ref().to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v0_0_6", feature = "dox"))]
    //fn set_for_enum(&self, enum_type: glib::types::Type, get_name_func: /*Unimplemented*/Fn(/*Ignored*/EnumValueObject) -> String, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call handy_sys:hdy_combo_row_set_for_enum() }
    //}

    //#[cfg(any(feature = "v0_0_10", feature = "dox"))]
    //fn set_get_name_func(&self, get_name_func: /*Unimplemented*/Fn(/*Ignored*/glib::Object) -> String, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call handy_sys:hdy_combo_row_set_get_name_func() }
    //}

    #[cfg(any(feature = "v0_0_7", feature = "dox"))]
    fn set_selected_index(&self, selected_index: i32) {
        unsafe {
            handy_sys::hdy_combo_row_set_selected_index(self.as_ref().to_glib_none().0, selected_index);
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_use_subtitle(&self, use_subtitle: bool) {
        unsafe {
            handy_sys::hdy_combo_row_set_use_subtitle(self.as_ref().to_glib_none().0, use_subtitle.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_7", feature = "dox"))]
    fn connect_property_selected_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_index_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyComboRow, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ComboRow>
        {
            let f: &F = &*(f as *const F);
            f(&ComboRow::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::selected-index\0".as_ptr() as *const _,
                Some(transmute(notify_selected_index_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_use_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_subtitle_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyComboRow, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ComboRow>
        {
            let f: &F = &*(f as *const F);
            f(&ComboRow::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-subtitle\0".as_ptr() as *const _,
                Some(transmute(notify_use_subtitle_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for ComboRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComboRow")
    }
}
