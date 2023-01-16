// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{BaselinePosition, LayoutManager, Orientable, Orientation};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkBoxLayout")]
    pub struct BoxLayout(Object<ffi::GtkBoxLayout, ffi::GtkBoxLayoutClass>) @extends LayoutManager, @implements Orientable;

    match fn {
        type_ => || ffi::gtk_box_layout_get_type(),
    }
}

impl BoxLayout {
    #[doc(alias = "gtk_box_layout_new")]
    pub fn new(orientation: Orientation) -> BoxLayout {
        assert_initialized_main_thread!();
        unsafe {
            LayoutManager::from_glib_full(ffi::gtk_box_layout_new(orientation.into_glib()))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`BoxLayout`] objects.
    ///
    /// This method returns an instance of [`BoxLayoutBuilder`](crate::builders::BoxLayoutBuilder) which can be used to create [`BoxLayout`] objects.
    pub fn builder() -> BoxLayoutBuilder {
        BoxLayoutBuilder::default()
    }

    #[doc(alias = "gtk_box_layout_get_baseline_position")]
    #[doc(alias = "get_baseline_position")]
    pub fn baseline_position(&self) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_box_layout_get_baseline_position(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_box_layout_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    pub fn is_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_box_layout_get_homogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_box_layout_get_spacing")]
    #[doc(alias = "get_spacing")]
    pub fn spacing(&self) -> u32 {
        unsafe { ffi::gtk_box_layout_get_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_box_layout_set_baseline_position")]
    pub fn set_baseline_position(&self, position: BaselinePosition) {
        unsafe {
            ffi::gtk_box_layout_set_baseline_position(self.to_glib_none().0, position.into_glib());
        }
    }

    #[doc(alias = "gtk_box_layout_set_homogeneous")]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_box_layout_set_homogeneous(self.to_glib_none().0, homogeneous.into_glib());
        }
    }

    #[doc(alias = "gtk_box_layout_set_spacing")]
    pub fn set_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_box_layout_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "baseline-position")]
    pub fn connect_baseline_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_baseline_position_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::baseline-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_baseline_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "homogeneous")]
    pub fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "spacing")]
    pub fn connect_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for BoxLayout {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BoxLayout`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BoxLayoutBuilder {
    baseline_position: Option<BaselinePosition>,
    homogeneous: Option<bool>,
    spacing: Option<i32>,
    orientation: Option<Orientation>,
}

impl BoxLayoutBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`BoxLayoutBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`BoxLayout`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BoxLayout {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref baseline_position) = self.baseline_position {
            properties.push(("baseline-position", baseline_position));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<BoxLayout>(&properties)
    }

    pub fn baseline_position(mut self, baseline_position: BaselinePosition) -> Self {
        self.baseline_position = Some(baseline_position);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for BoxLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BoxLayout")
    }
}
