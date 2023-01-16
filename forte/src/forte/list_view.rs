#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, ListView, *};
use gtkrs::{
    Accessible, AccessibleRole, Adjustment, Align, Buildable, ConstraintTarget, LayoutManager,
    ListBase, ListItemFactory, Orientable, Orientation, Overflow, Scrollable, ScrollablePolicy,
    SelectionModel, Widget,
};
#[derive(Clone)]
pub struct ListViewBuilder {
    obj: ListView,
}
impl Default for ListViewBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ListViewBuilder {
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn model(&mut self, value: &impl IsA<SelectionModel>) -> &mut Self {
        self.obj.set_property("model", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn hadjustment(&mut self, value: &impl IsA<Adjustment>) -> &mut Self {
        self.obj.set_property("hadjustment", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn factory(&mut self, value: &impl IsA<ListItemFactory>) -> &mut Self {
        self.obj.set_property("factory", value);
        self
    }
    pub fn orientation(&mut self, value: Orientation) -> &mut Self {
        self.obj.set_property("orientation", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn hscroll_policy(&mut self, value: ScrollablePolicy) -> &mut Self {
        self.obj.set_property("hscroll_policy", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn vadjustment(&mut self, value: &impl IsA<Adjustment>) -> &mut Self {
        self.obj.set_property("vadjustment", value);
        self
    }
    pub fn vscroll_policy(&mut self, value: ScrollablePolicy) -> &mut Self {
        self.obj.set_property("vscroll_policy", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn show_separators(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("show_separators", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn enable_rubberband(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("enable_rubberband", value);
        self
    }
    pub fn single_click_activate(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("single_click_activate", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn bind(&mut self) -> ListViewBinder {
        ListViewBinder { builder: self }
    }
    pub fn connect(&mut self) -> ListViewSignals {
        ListViewSignals { builder: self }
    }
}
impl crate::prelude::Builder for ListViewBuilder {
    type Target = ListView;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ListViewBuilder {
    type Target = ListView;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ListViewBinder<'builder> {
    builder: &'builder mut ListViewBuilder,
}
impl<'builder> ListViewBinder<'builder> {
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn model<T: IsA<SelectionModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("model", val));
        value.connect_update(move |value| obj.set_property("model", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn hadjustment<T: IsA<Adjustment>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hadjustment", val));
        value.connect_update(move |value| obj.set_property("hadjustment", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn factory<T: IsA<ListItemFactory>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("factory", val));
        value.connect_update(move |value| obj.set_property("factory", value));
        self.builder
    }
    pub fn orientation(
        &mut self,
        value: &(impl Prop<Orientation> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("orientation", val));
        value.connect_update(move |value| obj.set_property("orientation", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn hscroll_policy(
        &mut self,
        value: &(impl Prop<ScrollablePolicy> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hscroll_policy", val));
        value.connect_update(move |value| obj.set_property("hscroll_policy", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn vadjustment<T: IsA<Adjustment>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vadjustment", val));
        value.connect_update(move |value| obj.set_property("vadjustment", value));
        self.builder
    }
    pub fn vscroll_policy(
        &mut self,
        value: &(impl Prop<ScrollablePolicy> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vscroll_policy", val));
        value.connect_update(move |value| obj.set_property("vscroll_policy", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn show_separators(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("show_separators", val));
        value.connect_update(move |value| obj.set_property("show_separators", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn enable_rubberband(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("enable_rubberband", val));
        value.connect_update(move |value| obj.set_property("enable_rubberband", value));
        self.builder
    }
    pub fn single_click_activate(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("single_click_activate", val));
        value.connect_update(move |value| obj.set_property("single_click_activate", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ListViewBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
}
pub struct ListViewSignals<'builder> {
    builder: &'builder mut ListViewBuilder,
}
impl<'builder> ListViewSignals<'builder> {}
impl ForteExt for ListView {
    type Builder = ListViewBuilder;
}
#[macro_export]
macro_rules ! ListView { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: ListView :: forte ()) $ ($ rest) * } } }
