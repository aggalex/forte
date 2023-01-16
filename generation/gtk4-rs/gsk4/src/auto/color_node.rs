// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskColorNode")]
    pub struct ColorNode(Shared<ffi::GskColorNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for ColorNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_color_node_get_type()) }
    }
}

impl ColorNode {
    #[doc(alias = "gsk_color_node_new")]
    pub fn new(rgba: &gdk::RGBA, bounds: &graphene::Rect) -> ColorNode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_color_node_new(
                rgba.to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_color_node_get_color")]
    #[doc(alias = "get_color")]
    pub fn color(&self) -> gdk::RGBA {
        unsafe { from_glib_none(ffi::gsk_color_node_get_color(self.to_glib_none().0)) }
    }
}

impl fmt::Display for ColorNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ColorNode")
    }
}
