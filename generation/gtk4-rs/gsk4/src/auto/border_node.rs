// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{RenderNode, RoundedRect};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskBorderNode")]
    pub struct BorderNode(Shared<ffi::GskBorderNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for BorderNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_border_node_get_type()) }
    }
}

impl BorderNode {
    #[doc(alias = "gsk_border_node_get_outline")]
    #[doc(alias = "get_outline")]
    pub fn outline(&self) -> RoundedRect {
        unsafe { from_glib_none(ffi::gsk_border_node_get_outline(self.to_glib_none().0)) }
    }
}

impl fmt::Display for BorderNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BorderNode")
    }
}
