// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v0_0_11", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v0_0_11", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v0_0_11", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v0_0_11", feature = "dox"))]
use glib_sys;
use gtk;
use handy_sys;
#[cfg(any(feature = "v0_0_12", feature = "dox"))]
use libc;
#[cfg(any(feature = "v0_0_11", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v0_0_11", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v0_0_11", feature = "dox"))]
use PaginatorIndicatorStyle;
use Swipeable;

glib_wrapper! {
    pub struct Paginator(Object<handy_sys::HdyPaginator, handy_sys::HdyPaginatorClass, PaginatorClass>) @extends gtk::EventBox, gtk::Bin, gtk::Container, gtk::Widget, @implements gtk::Orientable, Swipeable;

    match fn {
        get_type => || handy_sys::hdy_paginator_get_type(),
    }
}

impl Paginator {
    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    pub fn new() -> Paginator {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(handy_sys::hdy_paginator_new())
        }
    }
}

#[cfg(any(feature = "v0_0_11", feature = "dox"))]
impl Default for Paginator {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PAGINATOR: Option<&Paginator> = None;

pub trait PaginatorExt: 'static {
    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn get_allow_mouse_drag(&self) -> bool;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_animation_duration(&self) -> u32;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_center_content(&self) -> bool;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_indicator_spacing(&self) -> u32;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_indicator_style(&self) -> PaginatorIndicatorStyle;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_interactive(&self) -> bool;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_n_pages(&self) -> u32;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_position(&self) -> f64;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_spacing(&self) -> u32;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn insert<P: IsA<gtk::Widget>>(&self, child: &P, position: i32);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn prepend<P: IsA<gtk::Widget>>(&self, child: &P);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn reorder<P: IsA<gtk::Widget>>(&self, child: &P, position: i32);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn scroll_to<P: IsA<gtk::Widget>>(&self, widget: &P);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn scroll_to_full<P: IsA<gtk::Widget>>(&self, widget: &P, duration: i64);

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn set_allow_mouse_drag(&self, allow_mouse_drag: bool);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_animation_duration(&self, duration: u32);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_center_content(&self, center_content: bool);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_indicator_spacing(&self, spacing: u32);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_indicator_style(&self, style: PaginatorIndicatorStyle);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_interactive(&self, interactive: bool);

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_spacing(&self, spacing: u32);

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_page_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_property_allow_mouse_drag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_animation_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_center_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_indicator_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_indicator_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_interactive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_n_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Paginator>> PaginatorExt for O {
    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn get_allow_mouse_drag(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_paginator_get_allow_mouse_drag(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_animation_duration(&self) -> u32 {
        unsafe {
            handy_sys::hdy_paginator_get_animation_duration(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_center_content(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_paginator_get_center_content(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_indicator_spacing(&self) -> u32 {
        unsafe {
            handy_sys::hdy_paginator_get_indicator_spacing(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_indicator_style(&self) -> PaginatorIndicatorStyle {
        unsafe {
            from_glib(handy_sys::hdy_paginator_get_indicator_style(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_interactive(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_paginator_get_interactive(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_n_pages(&self) -> u32 {
        unsafe {
            handy_sys::hdy_paginator_get_n_pages(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_position(&self) -> f64 {
        unsafe {
            handy_sys::hdy_paginator_get_position(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn get_spacing(&self) -> u32 {
        unsafe {
            handy_sys::hdy_paginator_get_spacing(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn insert<P: IsA<gtk::Widget>>(&self, child: &P, position: i32) {
        unsafe {
            handy_sys::hdy_paginator_insert(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn prepend<P: IsA<gtk::Widget>>(&self, child: &P) {
        unsafe {
            handy_sys::hdy_paginator_prepend(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn reorder<P: IsA<gtk::Widget>>(&self, child: &P, position: i32) {
        unsafe {
            handy_sys::hdy_paginator_reorder(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn scroll_to<P: IsA<gtk::Widget>>(&self, widget: &P) {
        unsafe {
            handy_sys::hdy_paginator_scroll_to(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn scroll_to_full<P: IsA<gtk::Widget>>(&self, widget: &P, duration: i64) {
        unsafe {
            handy_sys::hdy_paginator_scroll_to_full(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, duration);
        }
    }

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn set_allow_mouse_drag(&self, allow_mouse_drag: bool) {
        unsafe {
            handy_sys::hdy_paginator_set_allow_mouse_drag(self.as_ref().to_glib_none().0, allow_mouse_drag.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_animation_duration(&self, duration: u32) {
        unsafe {
            handy_sys::hdy_paginator_set_animation_duration(self.as_ref().to_glib_none().0, duration);
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_center_content(&self, center_content: bool) {
        unsafe {
            handy_sys::hdy_paginator_set_center_content(self.as_ref().to_glib_none().0, center_content.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_indicator_spacing(&self, spacing: u32) {
        unsafe {
            handy_sys::hdy_paginator_set_indicator_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_indicator_style(&self, style: PaginatorIndicatorStyle) {
        unsafe {
            handy_sys::hdy_paginator_set_indicator_style(self.as_ref().to_glib_none().0, style.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_interactive(&self, interactive: bool) {
        unsafe {
            handy_sys::hdy_paginator_set_interactive(self.as_ref().to_glib_none().0, interactive.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn set_spacing(&self, spacing: u32) {
        unsafe {
            handy_sys::hdy_paginator_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_page_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_changed_trampoline<P, F: Fn(&P, u32) + 'static>(this: *mut handy_sys::HdyPaginator, index: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast(), index)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"page-changed\0".as_ptr() as *const _,
                Some(transmute(page_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_12", feature = "dox"))]
    fn connect_property_allow_mouse_drag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_allow_mouse_drag_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyPaginator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::allow-mouse-drag\0".as_ptr() as *const _,
                Some(transmute(notify_allow_mouse_drag_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_animation_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_animation_duration_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyPaginator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::animation-duration\0".as_ptr() as *const _,
                Some(transmute(notify_animation_duration_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_center_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_center_content_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyPaginator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::center-content\0".as_ptr() as *const _,
                Some(transmute(notify_center_content_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_indicator_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_indicator_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyPaginator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::indicator-spacing\0".as_ptr() as *const _,
                Some(transmute(notify_indicator_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_indicator_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_indicator_style_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyPaginator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::indicator-style\0".as_ptr() as *const _,
                Some(transmute(notify_indicator_style_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_interactive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interactive_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyPaginator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::interactive\0".as_ptr() as *const _,
                Some(transmute(notify_interactive_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_n_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_pages_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyPaginator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::n-pages\0".as_ptr() as *const _,
                Some(transmute(notify_n_pages_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyPaginator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::position\0".as_ptr() as *const _,
                Some(transmute(notify_position_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_11", feature = "dox"))]
    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyPaginator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Paginator>
        {
            let f: &F = &*(f as *const F);
            f(&Paginator::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute(notify_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Paginator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Paginator")
    }
}
