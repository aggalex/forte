#![allow(dead_code, unused_imports)]
use crate::prelude::*;
use gtkrs::glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(any(feature = "v4_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
use gtkrs::NaturalWrapMode;
use gtkrs::{prelude::*, Label, *};
use gtkrs::{
    Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, Justification, LayoutManager,
    MovementStep, Overflow, Widget,
};
#[derive(Clone, Default)]
pub struct LabelBuilder {
    builder: gtkrs::LabelBuilder,
    on_build: Vec<Box<dyn FnOnce(&gtkrs::Label) + 'static>>,
    object: Option<gtkrs::Label>,
}
impl LabelBuilder {
    pub fn focus_on_click(&mut self, value: bool) -> &mut Self {
        self.builder.focus_on_click(value);
        self
    }
    pub fn has_tooltip(&mut self, value: bool) -> &mut Self {
        self.builder.has_tooltip(value);
        self
    }
    pub fn width_chars(&mut self, value: i32) -> &mut Self {
        self.builder.width_chars(value);
        self
    }
    pub fn sensitive(&mut self, value: bool) -> &mut Self {
        self.builder.sensitive(value);
        self
    }
    pub fn tooltip_text(&mut self, value: &str) -> &mut Self {
        self.builder.tooltip_text(value);
        self
    }
    pub fn attributes(&mut self, value: &pango::AttrList) -> &mut Self {
        self.builder.attributes(value);
        self
    }
    pub fn focusable(&mut self, value: bool) -> &mut Self {
        self.builder.focusable(value);
        self
    }
    pub fn accessible_role(&mut self, value: AccessibleRole) -> &mut Self {
        self.builder.accessible_role(value);
        self
    }
    pub fn can_focus(&mut self, value: bool) -> &mut Self {
        self.builder.can_focus(value);
        self
    }
    pub fn margin_top(&mut self, value: i32) -> &mut Self {
        self.builder.margin_top(value);
        self
    }
    pub fn ellipsize(&mut self, value: pango::EllipsizeMode) -> &mut Self {
        self.builder.ellipsize(value);
        self
    }
    pub fn max_width_chars(&mut self, value: i32) -> &mut Self {
        self.builder.max_width_chars(value);
        self
    }
    pub fn height_request(&mut self, value: i32) -> &mut Self {
        self.builder.height_request(value);
        self
    }
    pub fn wrap_mode(&mut self, value: pango::WrapMode) -> &mut Self {
        self.builder.wrap_mode(value);
        self
    }
    pub fn overflow(&mut self, value: Overflow) -> &mut Self {
        self.builder.overflow(value);
        self
    }
    pub fn halign(&mut self, value: Align) -> &mut Self {
        self.builder.halign(value);
        self
    }
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.builder.label(value);
        self
    }
    pub fn vexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder.vexpand_set(value);
        self
    }
    pub fn css_classes(&mut self, value: Vec<String>) -> &mut Self {
        self.builder.css_classes(value);
        self
    }
    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    pub fn tabs(&mut self, value: &pango::TabArray) -> &mut Self {
        self.builder.tabs(value);
        self
    }
    pub fn layout_manager(&mut self, value: &impl IsA<LayoutManager>) -> &mut Self {
        self.builder.layout_manager(value);
        self
    }
    pub fn width_request(&mut self, value: i32) -> &mut Self {
        self.builder.width_request(value);
        self
    }
    pub fn margin_start(&mut self, value: i32) -> &mut Self {
        self.builder.margin_start(value);
        self
    }
    pub fn xalign(&mut self, value: f32) -> &mut Self {
        self.builder.xalign(value);
        self
    }
    pub fn opacity(&mut self, value: f64) -> &mut Self {
        self.builder.opacity(value);
        self
    }
    pub fn lines(&mut self, value: i32) -> &mut Self {
        self.builder.lines(value);
        self
    }
    pub fn yalign(&mut self, value: f32) -> &mut Self {
        self.builder.yalign(value);
        self
    }
    pub fn selectable(&mut self, value: bool) -> &mut Self {
        self.builder.selectable(value);
        self
    }
    pub fn tooltip_markup(&mut self, value: &str) -> &mut Self {
        self.builder.tooltip_markup(value);
        self
    }
    pub fn visible(&mut self, value: bool) -> &mut Self {
        self.builder.visible(value);
        self
    }
    pub fn vexpand(&mut self, value: bool) -> &mut Self {
        self.builder.vexpand(value);
        self
    }
    pub fn mnemonic_widget(&mut self, value: &impl IsA<Widget>) -> &mut Self {
        self.builder.mnemonic_widget(value);
        self
    }
    pub fn cursor(&mut self, value: &gdk::Cursor) -> &mut Self {
        self.builder.cursor(value);
        self
    }
    pub fn extra_menu(&mut self, value: &impl IsA<gio::MenuModel>) -> &mut Self {
        self.builder.extra_menu(value);
        self
    }
    pub fn wrap(&mut self, value: bool) -> &mut Self {
        self.builder.wrap(value);
        self
    }
    pub fn hexpand(&mut self, value: bool) -> &mut Self {
        self.builder.hexpand(value);
        self
    }
    pub fn css_name(&mut self, value: &str) -> &mut Self {
        self.builder.css_name(value);
        self
    }
    pub fn use_markup(&mut self, value: bool) -> &mut Self {
        self.builder.use_markup(value);
        self
    }
    pub fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.builder.margin_bottom(value);
        self
    }
    pub fn name(&mut self, value: &str) -> &mut Self {
        self.builder.name(value);
        self
    }
    pub fn receives_default(&mut self, value: bool) -> &mut Self {
        self.builder.receives_default(value);
        self
    }
    pub fn valign(&mut self, value: Align) -> &mut Self {
        self.builder.valign(value);
        self
    }
    pub fn single_line_mode(&mut self, value: bool) -> &mut Self {
        self.builder.single_line_mode(value);
        self
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn natural_wrap_mode(&mut self, value: NaturalWrapMode) -> &mut Self {
        self.builder.natural_wrap_mode(value);
        self
    }
    pub fn hexpand_set(&mut self, value: bool) -> &mut Self {
        self.builder.hexpand_set(value);
        self
    }
    pub fn use_underline(&mut self, value: bool) -> &mut Self {
        self.builder.use_underline(value);
        self
    }
    pub fn margin_end(&mut self, value: i32) -> &mut Self {
        self.builder.margin_end(value);
        self
    }
    pub fn can_target(&mut self, value: bool) -> &mut Self {
        self.builder.can_target(value);
        self
    }
    pub fn justify(&mut self, value: Justification) -> &mut Self {
        self.builder.justify(value);
        self
    }
    pub fn bind(&mut self) -> LabelBinder {
        LabelBinder { builder: self }
    }
    pub fn connect(&mut self) -> LabelSignals {
        LabelSignals { builder: self }
    }
}
impl crate::prelude::Builder for LabelBuilder {
    type Target = Label;
    fn build(&mut self, func: impl Fn(Self::Target)) -> &mut Self {
        func(self.create());
        self
    }
    fn create(&mut self) -> Self::Target {
        self.obj = self.obj.or_else(|| {
            let obj = self.builder.build();
            self.on_build.iter().for_each(|f| f(&obj));
            Some(obj)
        });
        self.obj.unwrap().clone()
    }
}
impl std::ops::Deref for LabelBuilder {
    type Target = Label;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
pub struct LabelBinder<'builder> {
    builder: &'builder mut LabelBuilder,
}
impl<'builder> LabelBinder<'builder> {
    pub fn focus_on_click(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focus_on_click", val));
        value.connect_update(move |value| obj.set_property("focus_on_click", value));
        self.builder
    }
    pub fn has_tooltip(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("has_tooltip", val));
        value.connect_update(move |value| obj.set_property("has_tooltip", value));
        self.builder
    }
    pub fn width_chars(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_chars", val));
        value.connect_update(move |value| obj.set_property("width_chars", value));
        self.builder
    }
    pub fn sensitive(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("sensitive", val));
        value.connect_update(move |value| obj.set_property("sensitive", value));
        self.builder
    }
    pub fn tooltip_text(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_text", val));
        value.connect_update(move |value| obj.set_property("tooltip_text", value));
        self.builder
    }
    pub fn attributes(
        &mut self,
        value: &(impl Prop<pango::AttrList> + ?Sized),
    ) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("attributes", val));
        value.connect_update(move |value| obj.set_property("attributes", value));
        self.builder
    }
    pub fn focusable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("focusable", val));
        value.connect_update(move |value| obj.set_property("focusable", value));
        self.builder
    }
    pub fn accessible_role(
        &mut self,
        value: &(impl Prop<AccessibleRole> + ?Sized),
    ) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("accessible_role", val));
        value.connect_update(move |value| obj.set_property("accessible_role", value));
        self.builder
    }
    pub fn can_focus(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_focus", val));
        value.connect_update(move |value| obj.set_property("can_focus", value));
        self.builder
    }
    pub fn margin_top(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_top", val));
        value.connect_update(move |value| obj.set_property("margin_top", value));
        self.builder
    }
    pub fn ellipsize(
        &mut self,
        value: &(impl Prop<pango::EllipsizeMode> + ?Sized),
    ) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("ellipsize", val));
        value.connect_update(move |value| obj.set_property("ellipsize", value));
        self.builder
    }
    pub fn max_width_chars(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("max_width_chars", val));
        value.connect_update(move |value| obj.set_property("max_width_chars", value));
        self.builder
    }
    pub fn height_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("height_request", val));
        value.connect_update(move |value| obj.set_property("height_request", value));
        self.builder
    }
    pub fn wrap_mode(
        &mut self,
        value: &(impl Prop<pango::WrapMode> + ?Sized),
    ) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("wrap_mode", val));
        value.connect_update(move |value| obj.set_property("wrap_mode", value));
        self.builder
    }
    pub fn overflow(&mut self, value: &(impl Prop<Overflow> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("overflow", val));
        value.connect_update(move |value| obj.set_property("overflow", value));
        self.builder
    }
    pub fn halign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("halign", val));
        value.connect_update(move |value| obj.set_property("halign", value));
        self.builder
    }
    pub fn label(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("label", val));
        value.connect_update(move |value| obj.set_property("label", value));
        self.builder
    }
    pub fn vexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand_set", val));
        value.connect_update(move |value| obj.set_property("vexpand_set", value));
        self.builder
    }
    pub fn css_classes(&mut self, value: &(impl Prop<Vec<String>> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_classes", val));
        value.connect_update(move |value| obj.set_property("css_classes", value));
        self.builder
    }
    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    pub fn tabs(&mut self, value: &(impl Prop<pango::TabArray> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tabs", val));
        value.connect_update(move |value| obj.set_property("tabs", value));
        self.builder
    }
    pub fn layout_manager<T: IsA<LayoutManager>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("layout_manager", val));
        value.connect_update(move |value| obj.set_property("layout_manager", value));
        self.builder
    }
    pub fn width_request(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("width_request", val));
        value.connect_update(move |value| obj.set_property("width_request", value));
        self.builder
    }
    pub fn margin_start(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_start", val));
        value.connect_update(move |value| obj.set_property("margin_start", value));
        self.builder
    }
    pub fn xalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("xalign", val));
        value.connect_update(move |value| obj.set_property("xalign", value));
        self.builder
    }
    pub fn opacity(&mut self, value: &(impl Prop<f64> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("opacity", val));
        value.connect_update(move |value| obj.set_property("opacity", value));
        self.builder
    }
    pub fn lines(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("lines", val));
        value.connect_update(move |value| obj.set_property("lines", value));
        self.builder
    }
    pub fn yalign(&mut self, value: &(impl Prop<f32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("yalign", val));
        value.connect_update(move |value| obj.set_property("yalign", value));
        self.builder
    }
    pub fn selectable(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("selectable", val));
        value.connect_update(move |value| obj.set_property("selectable", value));
        self.builder
    }
    pub fn tooltip_markup(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("tooltip_markup", val));
        value.connect_update(move |value| obj.set_property("tooltip_markup", value));
        self.builder
    }
    pub fn visible(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("visible", val));
        value.connect_update(move |value| obj.set_property("visible", value));
        self.builder
    }
    pub fn vexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("vexpand", val));
        value.connect_update(move |value| obj.set_property("vexpand", value));
        self.builder
    }
    pub fn mnemonic_widget<T: IsA<Widget>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("mnemonic_widget", val));
        value.connect_update(move |value| obj.set_property("mnemonic_widget", value));
        self.builder
    }
    pub fn cursor(&mut self, value: &(impl Prop<gdk::Cursor> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("cursor", val));
        value.connect_update(move |value| obj.set_property("cursor", value));
        self.builder
    }
    pub fn extra_menu<T: IsA<gio::MenuModel>>(
        &mut self,
        value: &(impl Prop<T> + ?Sized),
    ) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("extra_menu", val));
        value.connect_update(move |value| obj.set_property("extra_menu", value));
        self.builder
    }
    pub fn wrap(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("wrap", val));
        value.connect_update(move |value| obj.set_property("wrap", value));
        self.builder
    }
    pub fn hexpand(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand", val));
        value.connect_update(move |value| obj.set_property("hexpand", value));
        self.builder
    }
    pub fn css_name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("css_name", val));
        value.connect_update(move |value| obj.set_property("css_name", value));
        self.builder
    }
    pub fn use_markup(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("use_markup", val));
        value.connect_update(move |value| obj.set_property("use_markup", value));
        self.builder
    }
    pub fn margin_bottom(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_bottom", val));
        value.connect_update(move |value| obj.set_property("margin_bottom", value));
        self.builder
    }
    pub fn name(&mut self, value: &(impl Prop<str> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("name", val));
        value.connect_update(move |value| obj.set_property("name", value));
        self.builder
    }
    pub fn receives_default(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("receives_default", val));
        value.connect_update(move |value| obj.set_property("receives_default", value));
        self.builder
    }
    pub fn valign(&mut self, value: &(impl Prop<Align> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("valign", val));
        value.connect_update(move |value| obj.set_property("valign", value));
        self.builder
    }
    pub fn single_line_mode(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("single_line_mode", val));
        value.connect_update(move |value| obj.set_property("single_line_mode", value));
        self.builder
    }
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub fn natural_wrap_mode(
        &mut self,
        value: &(impl Prop<NaturalWrapMode> + ?Sized),
    ) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("natural_wrap_mode", val));
        value.connect_update(move |value| obj.set_property("natural_wrap_mode", value));
        self.builder
    }
    pub fn hexpand_set(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("hexpand_set", val));
        value.connect_update(move |value| obj.set_property("hexpand_set", value));
        self.builder
    }
    pub fn use_underline(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("use_underline", val));
        value.connect_update(move |value| obj.set_property("use_underline", value));
        self.builder
    }
    pub fn margin_end(&mut self, value: &(impl Prop<i32> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("margin_end", val));
        value.connect_update(move |value| obj.set_property("margin_end", value));
        self.builder
    }
    pub fn can_target(&mut self, value: &(impl Prop<bool> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("can_target", val));
        value.connect_update(move |value| obj.set_property("can_target", value));
        self.builder
    }
    pub fn justify(&mut self, value: &(impl Prop<Justification> + ?Sized)) -> &mut LabelBuilder {
        let obj = self.builder.obj.clone();
        value.with(|val| obj.set_property("justify", val));
        value.connect_update(move |value| obj.set_property("justify", value));
        self.builder
    }
}
pub struct LabelSignals<'builder> {
    builder: &'builder mut LabelBuilder,
}
impl<'builder> LabelSignals<'builder> {}
impl ForteExt for Label {
    type Builder = LabelBuilder;
}
#[macro_export]
macro_rules ! Label { { $ ($ rest : tt) * } => { forte ! { (gtkrs :: Label :: forte ()) $ ($ rest) * } } }
