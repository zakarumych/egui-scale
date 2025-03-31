//! This crate provides a trait for scaling various types in the `egui` library.
//! It includes implementations for primitive types, vectors, and various `egui` types.
//! The `EguiScale` trait allows for scaling values by a given factor, which can be useful for
//! creating responsive UIs that adapt to different screen sizes or user preferences.

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![deny(clippy::pedantic)]

use egui::{
    epaint::Shadow,
    style::{Interaction, ScrollStyle, Spacing, TextCursorStyle, WidgetVisuals, Widgets},
    CornerRadius, FontId, Frame, Margin, Stroke, Style, Vec2, Visuals,
};

/// A trait for scaling various types in the `egui` library.
pub trait EguiScale {
    /// Scales the value by the given factor.
    fn scale(&mut self, scale: f32);

    /// Scales the value by the given factor and return the modified value.
    #[inline]
    #[must_use]
    fn scaled(mut self, scale: f32) -> Self
    where
        Self: Sized,
    {
        self.scale(scale);
        self
    }
}

impl EguiScale for f32 {
    #[inline]
    fn scale(&mut self, scale: f32) {
        *self *= scale;
    }
}

impl EguiScale for u8 {
    #[inline]
    fn scale(&mut self, scale: f32) {
        #![allow(clippy::cast_possible_truncation)]
        #![allow(clippy::cast_sign_loss)]

        *self = (f32::from(*self) * scale) as u8;
    }
}

impl EguiScale for i8 {
    #[inline]
    fn scale(&mut self, scale: f32) {
        #![allow(clippy::cast_possible_truncation)]

        *self = (f32::from(*self) * scale) as i8;
    }
}

impl EguiScale for Vec2 {
    #[inline]
    fn scale(&mut self, scale: f32) {
        *self *= scale;
    }
}

impl EguiScale for CornerRadius {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.nw.scale(scale);
        self.ne.scale(scale);
        self.se.scale(scale);
        self.sw.scale(scale);
    }
}

impl EguiScale for Margin {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.left.scale(scale);
        self.right.scale(scale);
        self.top.scale(scale);
        self.bottom.scale(scale);
    }
}

impl<T: EguiScale> EguiScale for [T] {
    #[inline]
    fn scale(&mut self, scale: f32) {
        for value in self.iter_mut() {
            value.scale(scale);
        }
    }
}

impl EguiScale for Shadow {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.offset.scale(scale);
        self.blur.scale(scale);
        self.spread.scale(scale);
    }
}

impl EguiScale for Stroke {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.width *= scale;
        if self.width < 1.0 {
            self.color.gamma_multiply(self.width);
            self.width = 1.0;
        }
    }
}

impl EguiScale for WidgetVisuals {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.bg_stroke.scale(scale);
        self.corner_radius.scale(scale);
        self.fg_stroke.scale(scale);
        self.expansion.scale(scale);
    }
}

impl EguiScale for Interaction {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.resize_grab_radius_corner.scale(scale);
        self.resize_grab_radius_side.scale(scale);
    }
}

impl EguiScale for Widgets {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.noninteractive.scale(scale);
        self.inactive.scale(scale);
        self.hovered.scale(scale);
        self.active.scale(scale);
        self.open.scale(scale);
    }
}

impl EguiScale for TextCursorStyle {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.stroke.scale(scale);
    }
}

impl EguiScale for Visuals {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.clip_rect_margin.scale(scale);
        self.menu_corner_radius.scale(scale);
        self.popup_shadow.scale(scale);
        self.resize_corner_size.scale(scale);
        self.selection.stroke.scale(scale);
        self.text_cursor.scale(scale);
        self.widgets.scale(scale);
        self.window_corner_radius.scale(scale);
        self.window_shadow.scale(scale);
        self.window_stroke.scale(scale);
    }
}

impl EguiScale for ScrollStyle {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.bar_inner_margin.scale(scale);
        self.bar_outer_margin.scale(scale);
        self.bar_width.scale(scale);
        self.floating_allocated_width.scale(scale);
        self.floating_width.scale(scale);
        self.handle_min_length.scale(scale);
    }
}

impl EguiScale for Spacing {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.button_padding.scale(scale);
        self.combo_height.scale(scale);
        self.combo_width.scale(scale);
        self.icon_spacing.scale(scale);
        self.icon_width.scale(scale);
        self.icon_width_inner.scale(scale);
        self.indent.scale(scale);
        self.interact_size.scale(scale);
        self.item_spacing.scale(scale);
        self.menu_margin.scale(scale);
        self.scroll.scale(scale);
        self.slider_width.scale(scale);
        self.text_edit_width.scale(scale);
        self.tooltip_width.scale(scale);
        self.window_margin.scale(scale);
    }
}

impl EguiScale for FontId {
    fn scale(&mut self, scale: f32) {
        self.size.scale(scale);
    }
}

impl EguiScale for Style {
    #[inline]
    fn scale(&mut self, scale: f32) {
        if let Some(font_id) = &mut self.override_font_id {
            font_id.scale(scale);
        }
        for font_id in self.text_styles.values_mut() {
            font_id.scale(scale);
        }
        self.interaction.scale(scale);
        self.spacing.scale(scale);
        self.visuals.scale(scale);
    }
}

impl<T> EguiScale for Option<T>
where
    T: EguiScale,
{
    #[inline]
    fn scale(&mut self, scale: f32) {
        if let Some(value) = self {
            value.scale(scale);
        }
    }
}

impl EguiScale for Frame {
    #[inline]
    fn scale(&mut self, scale: f32) {
        self.inner_margin.scale(scale);
        self.outer_margin.scale(scale);
        self.corner_radius.scale(scale);
        self.shadow.scale(scale);
        self.stroke.scale(scale);
    }
}
