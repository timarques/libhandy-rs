// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum ArrowsDirection {
    Up,
    Down,
    Left,
    Right,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ArrowsDirection {
    type GlibType = ffi::HdyArrowsDirection;

    fn to_glib(&self) -> ffi::HdyArrowsDirection {
        match *self {
            ArrowsDirection::Up => ffi::HDY_ARROWS_DIRECTION_UP,
            ArrowsDirection::Down => ffi::HDY_ARROWS_DIRECTION_DOWN,
            ArrowsDirection::Left => ffi::HDY_ARROWS_DIRECTION_LEFT,
            ArrowsDirection::Right => ffi::HDY_ARROWS_DIRECTION_RIGHT,
            ArrowsDirection::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyArrowsDirection> for ArrowsDirection {
    fn from_glib(value: ffi::HdyArrowsDirection) -> Self {
        skip_assert_initialized!();
        match value {
            0 => ArrowsDirection::Up,
            1 => ArrowsDirection::Down,
            2 => ArrowsDirection::Left,
            3 => ArrowsDirection::Right,
            value => ArrowsDirection::__Unknown(value),
        }
    }
}

impl StaticType for ArrowsDirection {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_arrows_direction_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ArrowsDirection {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ArrowsDirection {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ArrowsDirection {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum Fold {
    Unfolded,
    Folded,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for Fold {
    type GlibType = ffi::HdyFold;

    fn to_glib(&self) -> ffi::HdyFold {
        match *self {
            Fold::Unfolded => ffi::HDY_FOLD_UNFOLDED,
            Fold::Folded => ffi::HDY_FOLD_FOLDED,
            Fold::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyFold> for Fold {
    fn from_glib(value: ffi::HdyFold) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Fold::Unfolded,
            1 => Fold::Folded,
            value => Fold::__Unknown(value),
        }
    }
}

impl StaticType for Fold {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_fold_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Fold {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for Fold {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for Fold {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum LeafletChildTransitionType {
    None,
    Crossfade,
    Slide,
    Over,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for LeafletChildTransitionType {
    type GlibType = ffi::HdyLeafletChildTransitionType;

    fn to_glib(&self) -> ffi::HdyLeafletChildTransitionType {
        match *self {
            LeafletChildTransitionType::None => ffi::HDY_LEAFLET_CHILD_TRANSITION_TYPE_NONE,
            LeafletChildTransitionType::Crossfade => ffi::HDY_LEAFLET_CHILD_TRANSITION_TYPE_CROSSFADE,
            LeafletChildTransitionType::Slide => ffi::HDY_LEAFLET_CHILD_TRANSITION_TYPE_SLIDE,
            LeafletChildTransitionType::Over => ffi::HDY_LEAFLET_CHILD_TRANSITION_TYPE_OVER,
            LeafletChildTransitionType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyLeafletChildTransitionType> for LeafletChildTransitionType {
    fn from_glib(value: ffi::HdyLeafletChildTransitionType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => LeafletChildTransitionType::None,
            1 => LeafletChildTransitionType::Crossfade,
            2 => LeafletChildTransitionType::Slide,
            3 => LeafletChildTransitionType::Over,
            value => LeafletChildTransitionType::__Unknown(value),
        }
    }
}

impl StaticType for LeafletChildTransitionType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_leaflet_child_transition_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for LeafletChildTransitionType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for LeafletChildTransitionType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for LeafletChildTransitionType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum LeafletModeTransitionType {
    None,
    Slide,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for LeafletModeTransitionType {
    type GlibType = ffi::HdyLeafletModeTransitionType;

    fn to_glib(&self) -> ffi::HdyLeafletModeTransitionType {
        match *self {
            LeafletModeTransitionType::None => ffi::HDY_LEAFLET_MODE_TRANSITION_TYPE_NONE,
            LeafletModeTransitionType::Slide => ffi::HDY_LEAFLET_MODE_TRANSITION_TYPE_SLIDE,
            LeafletModeTransitionType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HdyLeafletModeTransitionType> for LeafletModeTransitionType {
    fn from_glib(value: ffi::HdyLeafletModeTransitionType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => LeafletModeTransitionType::None,
            1 => LeafletModeTransitionType::Slide,
            value => LeafletModeTransitionType::__Unknown(value),
        }
    }
}

impl StaticType for LeafletModeTransitionType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hdy_leaflet_mode_transition_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for LeafletModeTransitionType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for LeafletModeTransitionType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for LeafletModeTransitionType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
