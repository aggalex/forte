#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(any(feature = "v4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
use gtkrs::AccessibleRange;
use gtkrs::{prelude::*, Scale, *};
use gtkrs::{
    Accessible, AccessibleRole, Adjustment, Align, Buildable, ConstraintTarget, LayoutManager,
    Orientable, Orientation, Overflow, PositionType, Range, Widget,
};
#[derive(Clone)]
pub struct ScaleBuilder {
    obj: Scale,
}
impl Default for ScaleBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ScaleBuilder {
    pub fn fill_level(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("fill_level", value);
        self
    }
    pub fn round_digits(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("round_digits", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn value_pos(&mut self, value: PositionType) -> &mut Self {
        self.obj.set_property("value_pos", value);
        self
    }
    pub fn inverted(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("inverted", value);
        self
    }
    pub fn restrict_to_fill_level(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("restrict_to_fill_level", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn has_origin(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_origin", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn digits(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("digits", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn draw_value(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("draw_value", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn show_fill_level(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("show_fill_level", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn adjustment(&mut self, value: &impl IsA<Adjustment>) -> &mut Self {
        self.obj.set_property("adjustment", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn orientation(&mut self, value: Orientation) -> &mut Self {
        self.obj.set_property("orientation", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn bind(&mut self) -> ScaleBinder {
        ScaleBinder { builder: self }
    }
    pub fn connect(&mut self) -> ScaleSignals {
        ScaleSignals { builder: self }
    }
}
impl crate::prelude::Builder for ScaleBuilder {
    type Target = Scale;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ScaleBuilder {
    type Target = Scale;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ScaleBinder<'builder> {
    builder: &'builder mut ScaleBuilder,
}
impl<'builder> ScaleBinder<'builder> {
    pub fn fill_level(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("fill_level", val));
        value.connect_update(move |value| obj.set_property("fill_level", value));
        self.builder
    }
    pub fn round_digits(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("round_digits", val));
        value.connect_update(move |value| obj.set_property("round_digits", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn value_pos(&mut self, value: &(impl Prop<PositionType> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("value_pos", val));
        value.connect_update(move |value| obj.set_property("value_pos", value));
        self.builder
    }
    pub fn inverted(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("inverted", val));
        value.connect_update(move |value| obj.set_property("inverted", value));
        self.builder
    }
    pub fn restrict_to_fill_level(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("restrict_to_fill_level", val));
        value.connect_update(move |value| obj.set_property("restrict_to_fill_level", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn has_origin(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_origin", val));
        value.connect_update(move |value| obj.set_property("has_origin", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn digits(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("digits", val));
        value.connect_update(move |value| obj.set_property("digits", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn draw_value(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("draw_value", val));
        value.connect_update(move |value| obj.set_property("draw_value", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn show_fill_level(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("show_fill_level", val));
        value.connect_update(move |value| obj.set_property("show_fill_level", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn css_classes(&mut self, value: &(impl Prop<Vec<String>> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn adjustment<T: IsA<Adjustment>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("adjustment", val));
        value.connect_update(move |value| obj.set_property("adjustment", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn orientation(&mut self, value: &(impl Prop<Orientation> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("orientation", val));
        value.connect_update(move |value| obj.set_property("orientation", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
}
pub struct ScaleSignals<'builder> {
    builder: &'builder mut ScaleBuilder,
}
impl<'builder> ScaleSignals<'builder> {
    pub fn draw_value_notify(&mut self, f: impl Fn(&Scale) + 'static) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_draw_value_notify(f);
        &mut self.builder
    }
    pub fn value_pos_notify(&mut self, f: impl Fn(&Scale) + 'static) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_value_pos_notify(f);
        &mut self.builder
    }
    pub fn has_origin_notify(&mut self, f: impl Fn(&Scale) + 'static) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_has_origin_notify(f);
        &mut self.builder
    }
    pub fn digits_notify(&mut self, f: impl Fn(&Scale) + 'static) -> &mut ScaleBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_digits_notify(f);
        &mut self.builder
    }
}
impl ForteExt for Scale {
    type Builder = ScaleBuilder;
}
#[macro_export]
macro_rules ! Scale { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Scale :: forte ()) $ ($ rest) * } } }
