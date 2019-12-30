// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v0_0_10", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v0_0_10", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v0_0_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v0_0_10", feature = "dox"))]
use glib_sys;
use gtk;
use handy_sys;
#[cfg(any(feature = "v0_0_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v0_0_10", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v0_0_10", feature = "dox"))]
use ViewSwitcherPolicy;

glib_wrapper! {
    pub struct ViewSwitcherBar(Object<handy_sys::HdyViewSwitcherBar, handy_sys::HdyViewSwitcherBarClass, ViewSwitcherBarClass>) @extends gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        get_type => || handy_sys::hdy_view_switcher_bar_get_type(),
    }
}

impl ViewSwitcherBar {
    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    pub fn new() -> ViewSwitcherBar {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(handy_sys::hdy_view_switcher_bar_new())
        }
    }
}

#[cfg(any(feature = "v0_0_10", feature = "dox"))]
impl Default for ViewSwitcherBar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_VIEW_SWITCHER_BAR: Option<&ViewSwitcherBar> = None;

pub trait ViewSwitcherBarExt: 'static {
    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_icon_size(&self) -> gtk::IconSize;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_policy(&self) -> ViewSwitcherPolicy;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_reveal(&self) -> bool;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_stack(&self) -> Option<gtk::Stack>;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_icon_size(&self, icon_size: gtk::IconSize);

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_policy(&self, policy: ViewSwitcherPolicy);

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_reveal(&self, reveal: bool);

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_stack<P: IsA<gtk::Stack>>(&self, stack: Option<&P>);

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_reveal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ViewSwitcherBar>> ViewSwitcherBarExt for O {
    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_icon_size(&self) -> gtk::IconSize {
        unsafe {
            from_glib(handy_sys::hdy_view_switcher_bar_get_icon_size(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_policy(&self) -> ViewSwitcherPolicy {
        unsafe {
            from_glib(handy_sys::hdy_view_switcher_bar_get_policy(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_reveal(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_view_switcher_bar_get_reveal(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn get_stack(&self) -> Option<gtk::Stack> {
        unsafe {
            from_glib_none(handy_sys::hdy_view_switcher_bar_get_stack(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_icon_size(&self, icon_size: gtk::IconSize) {
        unsafe {
            handy_sys::hdy_view_switcher_bar_set_icon_size(self.as_ref().to_glib_none().0, icon_size.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_policy(&self, policy: ViewSwitcherPolicy) {
        unsafe {
            handy_sys::hdy_view_switcher_bar_set_policy(self.as_ref().to_glib_none().0, policy.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_reveal(&self, reveal: bool) {
        unsafe {
            handy_sys::hdy_view_switcher_bar_set_reveal(self.as_ref().to_glib_none().0, reveal.to_glib());
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn set_stack<P: IsA<gtk::Stack>>(&self, stack: Option<&P>) {
        unsafe {
            handy_sys::hdy_view_switcher_bar_set_stack(self.as_ref().to_glib_none().0, stack.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyViewSwitcherBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ViewSwitcherBar>
        {
            let f: &F = &*(f as *const F);
            f(&ViewSwitcherBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute(notify_icon_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_policy_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyViewSwitcherBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ViewSwitcherBar>
        {
            let f: &F = &*(f as *const F);
            f(&ViewSwitcherBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::policy\0".as_ptr() as *const _,
                Some(transmute(notify_policy_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_reveal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyViewSwitcherBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ViewSwitcherBar>
        {
            let f: &F = &*(f as *const F);
            f(&ViewSwitcherBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::reveal\0".as_ptr() as *const _,
                Some(transmute(notify_reveal_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_0_10", feature = "dox"))]
    fn connect_property_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stack_trampoline<P, F: Fn(&P) + 'static>(this: *mut handy_sys::HdyViewSwitcherBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ViewSwitcherBar>
        {
            let f: &F = &*(f as *const F);
            f(&ViewSwitcherBar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::stack\0".as_ptr() as *const _,
                Some(transmute(notify_stack_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for ViewSwitcherBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ViewSwitcherBar")
    }
}
