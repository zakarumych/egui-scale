//! This crate provides a trait for scaling various types in the `egui` library.
//! It includes implementations for primitive types, vectors, and various `egui` types.
//! The `EguiScale` trait allows for scaling values by a given factor, which can be useful for
//! creating responsive UIs that adapt to different screen sizes or user preferences.

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![deny(clippy::pedantic)]

use egui::{
    CornerRadius, FontId, Frame, Margin, Stroke, Style, Vec2, Visuals,
    epaint::Shadow,
    style::{Interaction, ScrollStyle, Spacing, TextCursorStyle, WidgetVisuals, Widgets},
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
            self.color = self.color.gamma_multiply(self.width);
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
        self.interact_radius.scale(scale);
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
        #[allow(deprecated)]
        self.clip_rect_margin.scale(scale);
        self.menu_corner_radius.scale(scale);
        self.popup_shadow.scale(scale);
        self.resize_corner_size.scale(scale);
        self.ime_composition.active_underline_stroke.scale(scale);
        self.ime_composition.inactive_underline_stroke.scale(scale);
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
        self.content_margin.scale(scale);
        self.floating_allocated_width.scale(scale);
        self.floating_width.scale(scale);
        self.handle_min_length.scale(scale);
        self.fade.size.scale(scale);
        // `kinetic.decay_time` is a time constant and is deliberately left unscaled.
        self.kinetic.stop_distance.scale(scale);
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
        self.default_area_size.scale(scale);
        self.extra_text_line_spacing.scale(scale);
        self.menu_spacing.scale(scale);
        self.menu_width.scale(scale);
        self.slider_rail_height.scale(scale);
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

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Color32, Stroke, Style};

    #[test]
    fn sub_pixel_stroke_fades_colour_and_clamps_width() {
        let mut stroke = Stroke::new(0.5, Color32::from_rgba_unmultiplied(200, 100, 50, 255));
        stroke.scale(1.0);
        assert!(
            (stroke.width - 1.0).abs() < f32::EPSILON,
            "width is clamped to one pixel, got {}",
            stroke.width
        );
        assert!(
            stroke.color.a() < 255,
            "colour must fade with the sub-pixel width, got alpha {}",
            stroke.color.a()
        );
    }

    #[test]
    fn sub_pixel_stroke_colour_matches_gamma_multiply() {
        let colour = Color32::from_rgba_unmultiplied(200, 100, 50, 255);
        let mut stroke = Stroke::new(2.0, colour);
        stroke.scale(0.25);
        assert!(
            (stroke.width - 1.0).abs() < f32::EPSILON,
            "width is clamped to one pixel, got {}",
            stroke.width
        );
        assert_eq!(stroke.color, colour.gamma_multiply(0.5));
    }

    #[test]
    fn stroke_at_or_above_one_pixel_keeps_colour() {
        let colour = Color32::from_rgba_unmultiplied(200, 100, 50, 255);
        let mut stroke = Stroke::new(2.0, colour);
        stroke.scale(1.0);
        assert!((stroke.width - 2.0).abs() < f32::EPSILON);
        assert_eq!(stroke.color, colour);
    }

    #[test]
    fn scroll_style_scales_content_margin_fade_and_stop_distance() {
        let before = ScrollStyle::default();
        let mut after = before;
        after.scale(2.0);

        assert_eq!(after.content_margin.left, before.content_margin.left * 2);
        assert_eq!(after.content_margin.top, before.content_margin.top * 2);
        assert!((after.fade.size - before.fade.size * 2.0).abs() < 1e-4);
        assert!((after.kinetic.stop_distance - before.kinetic.stop_distance * 2.0).abs() < 1e-4);
    }

    #[test]
    fn scroll_style_keeps_kinetic_decay_time() {
        let before = ScrollStyle::default();
        let mut after = before;
        after.scale(2.0);
        assert!((after.kinetic.decay_time - before.kinetic.decay_time).abs() < f32::EPSILON);
    }

    #[test]
    fn style_scales_spacing_lengths() {
        let mut base = Style::default();
        base.spacing.extra_text_line_spacing = 3.0;
        let mut after = base.clone();
        after.scale(2.0);

        assert!(
            (after.spacing.slider_rail_height - base.spacing.slider_rail_height * 2.0).abs() < 1e-4
        );
        assert!((after.spacing.extra_text_line_spacing - 6.0).abs() < 1e-4);
        assert!(
            (after.spacing.default_area_size.x - base.spacing.default_area_size.x * 2.0).abs()
                < 1e-4
        );
        assert!(
            (after.spacing.default_area_size.y - base.spacing.default_area_size.y * 2.0).abs()
                < 1e-4
        );
        assert!((after.spacing.menu_width - base.spacing.menu_width * 2.0).abs() < 1e-4);
        assert!((after.spacing.menu_spacing - base.spacing.menu_spacing * 2.0).abs() < 1e-4);
    }

    #[test]
    fn style_scales_interact_radius() {
        let base = Style::default();
        let mut after = base.clone();
        after.scale(2.0);
        assert!(
            (after.interaction.interact_radius - base.interaction.interact_radius * 2.0).abs()
                < 1e-4
        );
    }

    #[test]
    fn style_scales_ime_underline_widths() {
        let base = Style::default();
        let mut after = base.clone();
        after.scale(2.0);
        let (ab, aa) = (
            &base.visuals.ime_composition,
            &after.visuals.ime_composition,
        );
        assert!(
            (aa.active_underline_stroke.width - ab.active_underline_stroke.width * 2.0).abs()
                < 1e-4
        );
        assert!(
            (aa.inactive_underline_stroke.width - ab.inactive_underline_stroke.width * 2.0).abs()
                < 1e-4
        );
    }
}
