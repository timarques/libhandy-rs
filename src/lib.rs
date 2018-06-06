// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![allow(dead_code)]

extern crate gtk_sys as gtk_ffi;
extern crate handy_sys as ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
#[macro_use]
extern crate glib;
extern crate bitflags;
extern crate lazy_static;
extern crate libc;
extern crate gio_sys as gio_ffi;
extern crate gdk_sys as gdk_ffi;
extern crate gio;
extern crate gdk;
extern crate gtk;


#[macro_use]
mod rt;

pub use glib::Error;
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]

mod auto;
mod manual;
pub use auto::*;
pub use rt::*;
pub use manual::*;
