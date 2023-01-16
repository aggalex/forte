#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, ToggleButton, *};
use gtkrs::{
    Accessible, AccessibleRole, Actionable, Align, Buildable, Button, ConstraintTarget,
    LayoutManager, Overflow, Widget,
};
#[derive(Clone)]
pub struct ToggleButtonBuilder {
    obj: ToggleButton,
}
impl Default for ToggleButtonBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl ToggleButtonBuilder {
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn action_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("action_name", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn child(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("child", value);
        self
    }
    pub fn has_frame(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_frame", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn action_target(&mut self, value: &glib::Variant) -> &mut Self {
        self.obj.set_property("action_target", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn active(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("active", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("label", value);
        self
    }
    pub fn icon_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("icon_name", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn use_underline(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("use_underline", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn group(&mut self, value: &impl IsA<ToggleButton>) -> &mut Self {
        self.obj.set_property("group", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn bind(&mut self) -> ToggleButtonBinder {
        ToggleButtonBinder { builder: self }
    }
    pub fn connect(&mut self) -> ToggleButtonSignals {
        ToggleButtonSignals { builder: self }
    }
}
impl crate::prelude::Builder for ToggleButtonBuilder {
    type Target = ToggleButton;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for ToggleButtonBuilder {
    type Target = ToggleButton;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct ToggleButtonBinder<'builder> {
    builder: &'builder mut ToggleButtonBuilder,
}
impl<'builder> ToggleButtonBinder<'builder> {
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn action_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("action_name", val));
        value.connect_update(move |value| obj.set_property("action_name", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn focus_on_click(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn child<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("child", val));
        value.connect_update(move |value| obj.set_property("child", value));
        self.builder
    }
    pub fn has_frame(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_frame", val));
        value.connect_update(move |value| obj.set_property("has_frame", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn height_request(
        &mut self,
        value: &(impl Prop<i32> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn tooltip_markup(
        &mut self,
        value: &(impl Prop<str> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn cursor(
        &mut self,
        value: &(impl Prop<gdk::Cursor> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn action_target(
        &mut self,
        value: &(impl Prop<glib::Variant> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("action_target", val));
        value.connect_update(move |value| obj.set_property("action_target", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn active(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("active", val));
        value.connect_update(move |value| obj.set_property("active", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn receives_default(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn label(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("label", val));
        value.connect_update(move |value| obj.set_property("label", value));
        self.builder
    }
    pub fn icon_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("icon_name", val));
        value.connect_update(move |value| obj.set_property("icon_name", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn use_underline(
        &mut self,
        value: &(impl Prop<bool> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("use_underline", val));
        value.connect_update(move |value| obj.set_property("use_underline", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn group<T: IsA<ToggleButton>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("group", val));
        value.connect_update(move |value| obj.set_property("group", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
}
pub struct ToggleButtonSignals<'builder> {
    builder: &'builder mut ToggleButtonBuilder,
}
impl<'builder> ToggleButtonSignals<'builder> {
    pub fn active_notify(
        &mut self,
        f: impl Fn(&ToggleButton) + 'static,
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_active_notify(f);
        &mut self.builder
    }
    pub fn group_notify(
        &mut self,
        f: impl Fn(&ToggleButton) + 'static,
    ) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_group_notify(f);
        &mut self.builder
    }
    pub fn toggled(&mut self, f: impl Fn(&ToggleButton) + 'static) -> &mut ToggleButtonBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_toggled(f);
        &mut self.builder
    }
}
impl ForteExt for ToggleButton {
    type Builder = ToggleButtonBuilder;
}
#[macro_export]
macro_rules ! ToggleButton { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: ToggleButton :: forte ()) $ ($ rest) * } } }
