#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use gtkrs::{prelude::*, Popover, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Native,
    Overflow, PositionType, ShortcutManager, Widget,
};
#[derive(Clone)]
pub struct PopoverBuilder {
    obj: Popover,
}
impl Default for PopoverBuilder {
    fn default() -> Self {
        Self {
            obj: glib::Object::new(&[]),
        }
    }
}
impl PopoverBuilder {
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_markup", value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.obj.set_property("opacity", value);
        self
    }
    pub fn cascade_popdown(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("cascade_popdown", value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_start", value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("visible", value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_focus", value);
        self
    }
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focus_on_click", value);
        self
    }
    pub fn position(&mut self, value: PositionType) -> &mut Self {
        self.obj.set_property("position", value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_tooltip", value);
        self
    }
    pub fn default_widget(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("default_widget", value);
        self
    }
    pub fn child(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.obj.set_property("child", value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.obj.set_property("cursor", value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("focusable", value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("name", value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand_set", value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("width_request", value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("receives_default", value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("valign", value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand", value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.obj.set_property("css_classes", value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("vexpand", value);
        self
    }
    pub fn has_arrow(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("has_arrow", value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("sensitive", value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.obj.set_property("accessible_role", value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.obj.set_property("halign", value);
        self
    }
    pub fn pointing_to(&mut self, value: &gdk::Rectangle) -> &mut Self {
        self.obj.set_property("pointing_to", value);
        self
    }
    pub fn mnemonics_visible(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("mnemonics_visible", value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("tooltip_text", value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("height_request", value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("hexpand_set", value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("can_target", value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.obj.set_property("layout_manager", value);
        self
    }
    pub fn autohide(&mut self, value: bool) -> &mut Self {
        self.obj.set_property("autohide", value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.obj.set_property("overflow", value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_end", value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_top", value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.obj.set_property("margin_bottom", value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.obj.set_property("css_name", value);
        self
    }
    pub fn bind(&mut self) -> PopoverBinder {
        PopoverBinder { builder: self }
    }
    pub fn connect(&mut self) -> PopoverSignals {
        PopoverSignals { builder: self }
    }
}
impl crate::prelude::Builder for PopoverBuilder {
    type Target = Popover;
    fn build(&mut self, func: impl Fn(Self::Target) -> Self::Target) -> &mut Self {
        self.obj = func(self.obj.clone());
        self
    }
    fn create(&self) -> Self::Target {
        self.obj.clone()
    }
}
impl std::ops::Deref for PopoverBuilder {
    type Target = Popover;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct PopoverBinder<'builder> {
    builder: &'builder mut PopoverBuilder,
}
impl<'builder> PopoverBinder<'builder> {
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn cascade_popdown(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cascade_popdown", val));
        value.connect_update(move |value| obj.set_property("cascade_popdown", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn position(&mut self, value: &(impl Prop<PositionType> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("position", val));
        value.connect_update(move |value| obj.set_property("position", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn default_widget<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("default_widget", val));
        value.connect_update(move |value| obj.set_property("default_widget", value));
        self.builder
    }
    pub fn child<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("child", val));
        value.connect_update(move |value| obj.set_property("child", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn css_classes(
        &mut self,
        value: &(impl Prop<Vec<String>> + ?Sized),
    ) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn has_arrow(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_arrow", val));
        value.connect_update(move |value| obj.set_property("has_arrow", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn pointing_to(
        &mut self,
        value: &(impl Prop<gdk::Rectangle> + ?Sized),
    ) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("pointing_to", val));
        value.connect_update(move |value| obj.set_property("pointing_to", value));
        self.builder
    }
    pub fn mnemonics_visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("mnemonics_visible", val));
        value.connect_update(move |value| obj.set_property("mnemonics_visible", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn autohide(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("autohide", val));
        value.connect_update(move |value| obj.set_property("autohide", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
}
pub struct PopoverSignals<'builder> {
    builder: &'builder mut PopoverBuilder,
}
impl<'builder> PopoverSignals<'builder> {
    pub fn position_notify(&mut self, f: impl Fn(&Popover) + 'static) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_position_notify(f);
        &mut self.builder
    }
    pub fn child_notify(&mut self, f: impl Fn(&Popover) + 'static) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_child_notify(f);
        &mut self.builder
    }
    pub fn has_arrow_notify(&mut self, f: impl Fn(&Popover) + 'static) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_has_arrow_notify(f);
        &mut self.builder
    }
    pub fn default_widget_notify(&mut self, f: impl Fn(&Popover) + 'static) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_default_widget_notify(f);
        &mut self.builder
    }
    pub fn cascade_popdown_notify(
        &mut self,
        f: impl Fn(&Popover) + 'static,
    ) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_cascade_popdown_notify(f);
        &mut self.builder
    }
    pub fn mnemonics_visible_notify(
        &mut self,
        f: impl Fn(&Popover) + 'static,
    ) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_mnemonics_visible_notify(f);
        &mut self.builder
    }
    pub fn activate_default(&mut self, f: impl Fn(&Popover) + 'static) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_activate_default(f);
        &mut self.builder
    }
    pub fn autohide_notify(&mut self, f: impl Fn(&Popover) + 'static) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_autohide_notify(f);
        &mut self.builder
    }
    pub fn pointing_to_notify(&mut self, f: impl Fn(&Popover) + 'static) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_pointing_to_notify(f);
        &mut self.builder
    }
    pub fn closed(&mut self, f: impl Fn(&Popover) + 'static) -> &mut PopoverBuilder {
        let obj = self.builder.obj.clone();
        obj.connect_closed(f);
        &mut self.builder
    }
}
impl ForteExt for Popover {
    type Builder = PopoverBuilder;
}
#[macro_export]
macro_rules ! Popover { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Popover :: forte ()) $ ($ rest) * } } }
