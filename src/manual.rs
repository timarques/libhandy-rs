/*
use ffi;
use Entry;
use gtk;
use glib::translate::*;

impl Entry {
    pub fn get_from_gtk_entry(gtk_entry: &gtk::Entry) -> Option<Entry> {
        unsafe {
            /* I'm not sure if return Some is ok here */
            Some(Entry::from_glib_none(ffi::gspell_entry_get_from_gtk_entry(gtk_entry.to_glib_none().0)))
        }
    }
}
*/
