// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ShortcutAction;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkNamedAction")]
    pub struct NamedAction(Object<ffi::GtkNamedAction, ffi::GtkNamedActionClass>) @extends ShortcutAction;

    match fn {
        type_ => || ffi::gtk_named_action_get_type(),
    }
}

impl NamedAction {
    #[doc(alias = "gtk_named_action_new")]
    pub fn new(name: &str) -> NamedAction {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_named_action_new(name.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_named_action_get_action_name")]
    #[doc(alias = "get_action_name")]
    pub fn action_name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_named_action_get_action_name(self.to_glib_none().0)) }
    }
}

impl fmt::Display for NamedAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NamedAction")
    }
}
