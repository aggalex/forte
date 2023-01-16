// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkWaylandMonitor")]
    pub struct WaylandMonitor(Object<ffi::GdkWaylandMonitor, ffi::GdkWaylandMonitorClass>) @extends gdk::Monitor;

    match fn {
        type_ => || ffi::gdk_wayland_monitor_get_type(),
    }
}

impl WaylandMonitor {}

impl fmt::Display for WaylandMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WaylandMonitor")
    }
}
