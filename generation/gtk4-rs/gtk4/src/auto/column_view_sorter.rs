// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ColumnViewColumn, SortType, Sorter};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkColumnViewSorter")]
    pub struct ColumnViewSorter(Object<ffi::GtkColumnViewSorter, ffi::GtkColumnViewSorterClass>) @extends Sorter;

    match fn {
        type_ => || ffi::gtk_column_view_sorter_get_type(),
    }
}

impl ColumnViewSorter {
    #[doc(alias = "gtk_column_view_sorter_get_n_sort_columns")]
    #[doc(alias = "get_n_sort_columns")]
    pub fn n_sort_columns(&self) -> u32 {
        unsafe { ffi::gtk_column_view_sorter_get_n_sort_columns(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_column_view_sorter_get_nth_sort_column")]
    #[doc(alias = "get_nth_sort_column")]
    pub fn nth_sort_column(&self, position: u32) -> (Option<ColumnViewColumn>, SortType) {
        unsafe {
            let mut sort_order = mem::MaybeUninit::uninit();
            let ret = from_glib_none(ffi::gtk_column_view_sorter_get_nth_sort_column(
                self.to_glib_none().0,
                position,
                sort_order.as_mut_ptr(),
            ));
            (ret, from_glib(sort_order.assume_init()))
        }
    }

    #[doc(alias = "gtk_column_view_sorter_get_primary_sort_column")]
    #[doc(alias = "get_primary_sort_column")]
    pub fn primary_sort_column(&self) -> Option<ColumnViewColumn> {
        unsafe {
            from_glib_none(ffi::gtk_column_view_sorter_get_primary_sort_column(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_sorter_get_primary_sort_order")]
    #[doc(alias = "get_primary_sort_order")]
    pub fn primary_sort_order(&self) -> SortType {
        unsafe {
            from_glib(ffi::gtk_column_view_sorter_get_primary_sort_order(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "primary-sort-column")]
    pub fn connect_primary_sort_column_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_primary_sort_column_trampoline<
            F: Fn(&ColumnViewSorter) + 'static,
        >(
            this: *mut ffi::GtkColumnViewSorter,
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
                b"notify::primary-sort-column\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_primary_sort_column_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "primary-sort-order")]
    pub fn connect_primary_sort_order_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_primary_sort_order_trampoline<
            F: Fn(&ColumnViewSorter) + 'static,
        >(
            this: *mut ffi::GtkColumnViewSorter,
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
                b"notify::primary-sort-order\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_primary_sort_order_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ColumnViewSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ColumnViewSorter")
    }
}
