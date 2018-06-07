use ffi;
use Arrows;
use gtk;
use glib::translate::*;

impl Arrows {
    pub fn new() -> gtk::Widget {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::hdy_arrows_new())
        }
    }
}
