// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![allow(dead_code)]

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk_sys as gtk_ffi;
extern crate libhandy_sys as ffi;
#[macro_use]
extern crate glib;
extern crate bitflags;
extern crate gdk;
extern crate gdk_sys as gdk_ffi;
extern crate gio;
extern crate gio_sys as gio_ffi;
extern crate gtk;
extern crate lazy_static;
extern crate libc;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("Libhandy may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using libhandy.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use glib::Error;
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
mod auto;
mod value_object;
pub use auto::*;
