// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ 62f3bf0)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![cfg_attr(feature = "cargo-clippy", allow(approx_constant, type_complexity, unreadable_literal))]

extern crate libc;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gio_sys as gio;
extern crate gtk_sys as gtk;
extern crate gdk_sys as gdk;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type HdyArrowsDirection = c_int;
pub const HDY_ARROWS_DIRECTION_UP: HdyArrowsDirection = 0;
pub const HDY_ARROWS_DIRECTION_DOWN: HdyArrowsDirection = 1;
pub const HDY_ARROWS_DIRECTION_LEFT: HdyArrowsDirection = 2;
pub const HDY_ARROWS_DIRECTION_RIGHT: HdyArrowsDirection = 3;

pub type HdyFold = c_int;
pub const HDY_FOLD_UNFOLDED: HdyFold = 0;
pub const HDY_FOLD_FOLDED: HdyFold = 1;

pub type HdyLeafletChildTransitionType = c_int;
pub const HDY_LEAFLET_CHILD_TRANSITION_TYPE_NONE: HdyLeafletChildTransitionType = 0;
pub const HDY_LEAFLET_CHILD_TRANSITION_TYPE_CROSSFADE: HdyLeafletChildTransitionType = 1;
pub const HDY_LEAFLET_CHILD_TRANSITION_TYPE_SLIDE: HdyLeafletChildTransitionType = 2;
pub const HDY_LEAFLET_CHILD_TRANSITION_TYPE_OVER: HdyLeafletChildTransitionType = 3;

pub type HdyLeafletModeTransitionType = c_int;
pub const HDY_LEAFLET_MODE_TRANSITION_TYPE_NONE: HdyLeafletModeTransitionType = 0;
pub const HDY_LEAFLET_MODE_TRANSITION_TYPE_SLIDE: HdyLeafletModeTransitionType = 1;

// Callbacks
pub type HdyComboRowGetEnumValueNameFunc = Option<unsafe extern "C" fn(*mut HdyEnumValueObject, gpointer) -> *mut c_char>;
pub type HdyComboRowGetNameFunc = Option<unsafe extern "C" fn(*mut gobject::GObject, gpointer) -> *mut c_char>;

// Records
#[repr(C)]
pub struct HdyActionRowClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdyActionRowClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyActionRowClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyArrowsClass {
    pub parent_class: gtk::GtkDrawingAreaClass,
}

impl ::std::fmt::Debug for HdyArrowsClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyArrowsClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct HdyColumnClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdyColumnClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyColumnClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct HdyComboRowClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdyComboRowClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyComboRowClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct HdyDialerButtonClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdyDialerButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyDialerButtonClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct HdyDialerClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdyDialerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyDialerClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct HdyDialerCycleButtonClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdyDialerCycleButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyDialerCycleButtonClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyEnumValueObjectClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for HdyEnumValueObjectClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyEnumValueObjectClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct HdyExpanderRowClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdyExpanderRowClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyExpanderRowClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyHeaderGroupClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for HdyHeaderGroupClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyHeaderGroupClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct HdyLeafletClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdyLeafletClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyLeafletClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct HdySearchBarClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdySearchBarClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdySearchBarClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct HdyTitleBarClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for HdyTitleBarClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyTitleBarClass @ {:?}", self as *const _))
         .finish()
    }
}

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyActionRow {
    pub parent_instance: gtk::GtkListBoxRow,
}

impl ::std::fmt::Debug for HdyActionRow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyActionRow @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyArrows {
    pub parent_instance: gtk::GtkDrawingArea,
}

impl ::std::fmt::Debug for HdyArrows {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyArrows @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct HdyColumn(c_void);

impl ::std::fmt::Debug for HdyColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyColumn @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyComboRow {
    pub parent_instance: HdyActionRow,
}

impl ::std::fmt::Debug for HdyComboRow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyComboRow @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyDialer {
    pub parent_instance: gtk::GtkBin,
}

impl ::std::fmt::Debug for HdyDialer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyDialer @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyDialerButton {
    pub parent_instance: gtk::GtkButton,
}

impl ::std::fmt::Debug for HdyDialerButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyDialerButton @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyDialerCycleButton {
    pub parent_instance: HdyDialerButton,
}

impl ::std::fmt::Debug for HdyDialerCycleButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyDialerCycleButton @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct HdyEnumValueObject(c_void);

impl ::std::fmt::Debug for HdyEnumValueObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyEnumValueObject @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyExpanderRow {
    pub parent_instance: HdyActionRow,
}

impl ::std::fmt::Debug for HdyExpanderRow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyExpanderRow @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyHeaderGroup {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for HdyHeaderGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyHeaderGroup @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdyLeaflet {
    pub parent_instance: gtk::GtkContainer,
}

impl ::std::fmt::Debug for HdyLeaflet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyLeaflet @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdySearchBar {
    pub parent_instance: gtk::GtkBin,
}

impl ::std::fmt::Debug for HdySearchBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdySearchBar @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct HdyTitleBar(c_void);

impl ::std::fmt::Debug for HdyTitleBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HdyTitleBar @ {:?}", self as *const _))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // HdyArrowsDirection
    //=========================================================================
    pub fn hdy_arrows_direction_get_type() -> GType;

    //=========================================================================
    // HdyFold
    //=========================================================================
    pub fn hdy_fold_get_type() -> GType;

    //=========================================================================
    // HdyLeafletChildTransitionType
    //=========================================================================
    pub fn hdy_leaflet_child_transition_type_get_type() -> GType;

    //=========================================================================
    // HdyLeafletModeTransitionType
    //=========================================================================
    pub fn hdy_leaflet_mode_transition_type_get_type() -> GType;

    //=========================================================================
    // HdyActionRow
    //=========================================================================
    pub fn hdy_action_row_get_type() -> GType;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_new() -> *mut HdyActionRow;
    pub fn hdy_action_row_activate(self_: *mut HdyActionRow);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_add_action(self_: *mut HdyActionRow, widget: *mut gtk::GtkWidget);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_add_prefix(self_: *mut HdyActionRow, widget: *mut gtk::GtkWidget);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_get_icon_name(self_: *mut HdyActionRow) -> *const c_char;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_get_subtitle(self_: *mut HdyActionRow) -> *const c_char;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_get_title(self_: *mut HdyActionRow) -> *const c_char;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_get_use_underline(self_: *mut HdyActionRow) -> gboolean;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_set_icon_name(self_: *mut HdyActionRow, icon_name: *const c_char);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_set_subtitle(self_: *mut HdyActionRow, subtitle: *const c_char);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_set_title(self_: *mut HdyActionRow, title: *const c_char);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_action_row_set_use_underline(self_: *mut HdyActionRow, use_underline: gboolean);

    //=========================================================================
    // HdyArrows
    //=========================================================================
    pub fn hdy_arrows_get_type() -> GType;
    pub fn hdy_arrows_new() -> *mut gtk::GtkWidget;
    pub fn hdy_arrows_animate(self_: *mut HdyArrows);
    pub fn hdy_arrows_get_count(self_: *mut HdyArrows) -> c_uint;
    pub fn hdy_arrows_get_direction(self_: *mut HdyArrows) -> HdyArrowsDirection;
    pub fn hdy_arrows_get_duration(self_: *mut HdyArrows) -> c_uint;
    pub fn hdy_arrows_set_count(self_: *mut HdyArrows, count: c_uint);
    pub fn hdy_arrows_set_direction(self_: *mut HdyArrows, direction: HdyArrowsDirection);
    pub fn hdy_arrows_set_duration(self_: *mut HdyArrows, duration: c_uint);

    //=========================================================================
    // HdyColumn
    //=========================================================================
    pub fn hdy_column_get_type() -> GType;
    pub fn hdy_column_new() -> *mut HdyColumn;
    pub fn hdy_column_get_linear_growth_width(self_: *mut HdyColumn) -> c_int;
    pub fn hdy_column_get_maximum_width(self_: *mut HdyColumn) -> c_int;
    pub fn hdy_column_set_linear_growth_width(self_: *mut HdyColumn, linear_growth_width: c_int);
    pub fn hdy_column_set_maximum_width(self_: *mut HdyColumn, maximum_width: c_int);

    //=========================================================================
    // HdyComboRow
    //=========================================================================
    pub fn hdy_combo_row_get_type() -> GType;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_combo_row_new() -> *mut HdyComboRow;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_combo_row_bind_model(self_: *mut HdyComboRow, model: *mut gio::GListModel, create_list_widget_func: gtk::GtkListBoxCreateWidgetFunc, create_current_widget_func: gtk::GtkListBoxCreateWidgetFunc, user_data: gpointer, user_data_free_func: glib::GDestroyNotify);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_combo_row_bind_name_model(self_: *mut HdyComboRow, model: *mut gio::GListModel, get_name_func: HdyComboRowGetNameFunc, user_data: gpointer, user_data_free_func: glib::GDestroyNotify);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_combo_row_get_model(self_: *mut HdyComboRow) -> *mut gio::GListModel;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_combo_row_set_for_enum(self_: *mut HdyComboRow, enum_type: GType, get_name_func: HdyComboRowGetEnumValueNameFunc, user_data: gpointer, user_data_free_func: glib::GDestroyNotify);

    //=========================================================================
    // HdyDialer
    //=========================================================================
    pub fn hdy_dialer_get_type() -> GType;
    pub fn hdy_dialer_new() -> *mut gtk::GtkWidget;
    pub fn hdy_dialer_clear_number(self_: *mut HdyDialer);
    pub fn hdy_dialer_get_number(self_: *mut HdyDialer) -> *const c_char;
    pub fn hdy_dialer_get_relief(self_: *mut HdyDialer) -> gtk::GtkReliefStyle;
    pub fn hdy_dialer_get_show_action_buttons(self_: *mut HdyDialer) -> gboolean;
    pub fn hdy_dialer_set_number(self_: *mut HdyDialer, number: *const c_char);
    pub fn hdy_dialer_set_relief(self_: *mut HdyDialer, relief: gtk::GtkReliefStyle);
    pub fn hdy_dialer_set_show_action_buttons(self_: *mut HdyDialer, show: gboolean);

    //=========================================================================
    // HdyDialerButton
    //=========================================================================
    pub fn hdy_dialer_button_get_type() -> GType;
    pub fn hdy_dialer_button_new(symbols: *const c_char) -> *mut gtk::GtkWidget;
    pub fn hdy_dialer_button_get_digit(self_: *mut HdyDialerButton) -> c_int;
    pub fn hdy_dialer_button_get_symbols(self_: *mut HdyDialerButton) -> *const c_char;

    //=========================================================================
    // HdyDialerCycleButton
    //=========================================================================
    pub fn hdy_dialer_cycle_button_get_type() -> GType;
    pub fn hdy_dialer_cycle_button_new(symbols: *const c_char) -> *mut gtk::GtkWidget;
    pub fn hdy_dialer_cycle_button_get_current_symbol(self_: *mut HdyDialerCycleButton) -> u32;
    pub fn hdy_dialer_cycle_button_get_cycle_timeout(self_: *mut HdyDialerCycleButton) -> c_int;
    pub fn hdy_dialer_cycle_button_is_cycling(self_: *mut HdyDialerCycleButton) -> gboolean;
    pub fn hdy_dialer_cycle_button_set_cycle_timeout(self_: *mut HdyDialerCycleButton, timeout: c_int);
    pub fn hdy_dialer_cycle_button_stop_cycle(self_: *mut HdyDialerCycleButton);

    //=========================================================================
    // HdyEnumValueObject
    //=========================================================================
    pub fn hdy_enum_value_object_get_type() -> GType;
    pub fn hdy_enum_value_object_new(enum_value: *mut gobject::GEnumValue) -> *mut HdyEnumValueObject;
    pub fn hdy_enum_value_object_get_name(self_: *mut HdyEnumValueObject) -> *const c_char;
    pub fn hdy_enum_value_object_get_nick(self_: *mut HdyEnumValueObject) -> *const c_char;
    pub fn hdy_enum_value_object_get_value(self_: *mut HdyEnumValueObject) -> c_int;

    //=========================================================================
    // HdyExpanderRow
    //=========================================================================
    pub fn hdy_expander_row_get_type() -> GType;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_expander_row_new() -> *mut HdyExpanderRow;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_expander_row_get_enable_expansion(self_: *mut HdyExpanderRow) -> gboolean;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_expander_row_get_show_enable_switch(self_: *mut HdyExpanderRow) -> gboolean;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_expander_row_set_enable_expansion(self_: *mut HdyExpanderRow, enable_expansion: gboolean);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_expander_row_set_show_enable_switch(self_: *mut HdyExpanderRow, show_enable_switch: gboolean);

    //=========================================================================
    // HdyHeaderGroup
    //=========================================================================
    pub fn hdy_header_group_get_type() -> GType;
    pub fn hdy_header_group_new() -> *mut HdyHeaderGroup;
    pub fn hdy_header_group_add_header_bar(self_: *mut HdyHeaderGroup, header_bar: *mut gtk::GtkHeaderBar);
    pub fn hdy_header_group_get_focus(self_: *mut HdyHeaderGroup) -> *mut gtk::GtkHeaderBar;
    pub fn hdy_header_group_get_header_bars(self_: *mut HdyHeaderGroup) -> *mut glib::GSList;
    pub fn hdy_header_group_remove_header_bar(self_: *mut HdyHeaderGroup, header_bar: *mut gtk::GtkHeaderBar);
    pub fn hdy_header_group_set_focus(self_: *mut HdyHeaderGroup, header_bar: *mut gtk::GtkHeaderBar);

    //=========================================================================
    // HdyLeaflet
    //=========================================================================
    pub fn hdy_leaflet_get_type() -> GType;
    pub fn hdy_leaflet_new() -> *mut gtk::GtkWidget;
    pub fn hdy_leaflet_get_child_transition_duration(self_: *mut HdyLeaflet) -> c_uint;
    pub fn hdy_leaflet_get_child_transition_running(self_: *mut HdyLeaflet) -> gboolean;
    pub fn hdy_leaflet_get_child_transition_type(self_: *mut HdyLeaflet) -> HdyLeafletChildTransitionType;
    pub fn hdy_leaflet_get_fold(self_: *mut HdyLeaflet) -> HdyFold;
    pub fn hdy_leaflet_get_homogeneous(self_: *mut HdyLeaflet, fold: HdyFold, orientation: gtk::GtkOrientation) -> gboolean;
    pub fn hdy_leaflet_get_interpolate_size(self_: *mut HdyLeaflet) -> gboolean;
    pub fn hdy_leaflet_get_mode_transition_duration(self_: *mut HdyLeaflet) -> c_uint;
    pub fn hdy_leaflet_get_mode_transition_type(self_: *mut HdyLeaflet) -> HdyLeafletModeTransitionType;
    pub fn hdy_leaflet_get_visible_child(self_: *mut HdyLeaflet) -> *mut gtk::GtkWidget;
    pub fn hdy_leaflet_get_visible_child_name(self_: *mut HdyLeaflet) -> *const c_char;
    pub fn hdy_leaflet_set_child_transition_duration(self_: *mut HdyLeaflet, duration: c_uint);
    pub fn hdy_leaflet_set_child_transition_type(self_: *mut HdyLeaflet, transition: HdyLeafletChildTransitionType);
    pub fn hdy_leaflet_set_homogeneous(self_: *mut HdyLeaflet, fold: HdyFold, orientation: gtk::GtkOrientation, homogeneous: gboolean);
    pub fn hdy_leaflet_set_interpolate_size(self_: *mut HdyLeaflet, interpolate_size: gboolean);
    pub fn hdy_leaflet_set_mode_transition_duration(self_: *mut HdyLeaflet, duration: c_uint);
    pub fn hdy_leaflet_set_mode_transition_type(self_: *mut HdyLeaflet, transition: HdyLeafletModeTransitionType);
    pub fn hdy_leaflet_set_visible_child(self_: *mut HdyLeaflet, visible_child: *mut gtk::GtkWidget);
    pub fn hdy_leaflet_set_visible_child_name(self_: *mut HdyLeaflet, name: *const c_char);

    //=========================================================================
    // HdySearchBar
    //=========================================================================
    pub fn hdy_search_bar_get_type() -> GType;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_search_bar_new() -> *mut gtk::GtkWidget;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_search_bar_connect_entry(self_: *mut HdySearchBar, entry: *mut gtk::GtkEntry);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_search_bar_get_search_mode(self_: *mut HdySearchBar) -> gboolean;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_search_bar_get_show_close_button(self_: *mut HdySearchBar) -> gboolean;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_search_bar_handle_event(self_: *mut HdySearchBar, event: *mut gdk::GdkEvent) -> gboolean;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_search_bar_set_search_mode(self_: *mut HdySearchBar, search_mode: gboolean);
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_search_bar_set_show_close_button(self_: *mut HdySearchBar, visible: gboolean);

    //=========================================================================
    // HdyTitleBar
    //=========================================================================
    pub fn hdy_title_bar_get_type() -> GType;
    pub fn hdy_title_bar_new() -> *mut HdyTitleBar;
    pub fn hdy_title_bar_get_selection_mode(self_: *mut HdyTitleBar) -> gboolean;
    pub fn hdy_title_bar_set_selection_mode(self_: *mut HdyTitleBar, selection_mode: gboolean);

    //=========================================================================
    // Other functions
    //=========================================================================
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_enum_value_row_name(value: *mut HdyEnumValueObject, user_data: gpointer) -> *mut c_char;
    pub fn hdy_init(argc: *mut c_int, argv: *mut *mut *mut c_char) -> gboolean;
    #[cfg(any(feature = "v0_0_6", feature = "dox"))]
    pub fn hdy_list_box_separator_header(row: *mut gtk::GtkListBoxRow, before: *mut gtk::GtkListBoxRow, unused_user_data: gpointer);
    pub fn hdy_string_utf8_len(string: *mut glib::GString) -> c_long;
    pub fn hdy_string_utf8_truncate(string: *mut glib::GString, len: size_t) -> *mut glib::GString;

}