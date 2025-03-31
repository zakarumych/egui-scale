//! This crate provides a trait for zooming various types in the `egui` library.
//! It includes implementations for primitive types, vectors, and various `egui` types.
//! The `Zoom` trait allows for scaling values by a given factor, which can be useful for
//! creating responsive UIs that adapt to different screen sizes or user preferences.

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![deny(clippy::pedantic)]

use egui::{
    epaint::Shadow,
    style::{Interaction, ScrollStyle, Spacing, TextCursorStyle, WidgetVisuals, Widgets},
    CornerRadius, FontId, Frame, Margin, Stroke, Style, Vec2, Visuals,
};

/// A trait for zooming various types in the `egui` library.
pub trait Zoom {
    /// Zooms the value by the given factor.
    fn zoom(&mut self, zoom: f32);

    /// Zoom the value by the given factor and return the modified value.
    #[inline]
    #[must_use]
    fn zoomed(mut self, zoom: f32) -> Self
    where
        Self: Sized,
    {
        self.zoom(zoom);
        self
    }
}

impl Zoom for f32 {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        *self *= zoom;
    }
}

impl Zoom for u8 {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        #![allow(clippy::cast_possible_truncation)]
        #![allow(clippy::cast_sign_loss)]

        *self = (f32::from(*self) * zoom) as u8;
    }
}

impl Zoom for i8 {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        #![allow(clippy::cast_possible_truncation)]

        *self = (f32::from(*self) * zoom) as i8;
    }
}

impl Zoom for Vec2 {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        *self *= zoom;
    }
}

impl Zoom for CornerRadius {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.nw.zoom(zoom);
        self.ne.zoom(zoom);
        self.se.zoom(zoom);
        self.sw.zoom(zoom);
    }
}

impl Zoom for Margin {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.left.zoom(zoom);
        self.right.zoom(zoom);
        self.top.zoom(zoom);
        self.bottom.zoom(zoom);
    }
}

impl<T: Zoom> Zoom for [T] {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        for value in self.iter_mut() {
            value.zoom(zoom);
        }
    }
}

impl Zoom for Shadow {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.offset.zoom(zoom);
        self.blur.zoom(zoom);
        self.spread.zoom(zoom);
    }
}

impl Zoom for Stroke {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.width *= zoom;
        if self.width < 1.0 {
            self.color.gamma_multiply(self.width);
            self.width = 1.0;
        }
    }
}

impl Zoom for WidgetVisuals {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.bg_stroke.zoom(zoom);
        self.corner_radius.zoom(zoom);
        self.fg_stroke.zoom(zoom);
        self.expansion.zoom(zoom);
    }
}

impl Zoom for Interaction {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.resize_grab_radius_corner.zoom(zoom);
        self.resize_grab_radius_side.zoom(zoom);
    }
}

impl Zoom for Widgets {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.noninteractive.zoom(zoom);
        self.inactive.zoom(zoom);
        self.hovered.zoom(zoom);
        self.active.zoom(zoom);
        self.open.zoom(zoom);
    }
}

impl Zoom for TextCursorStyle {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.stroke.zoom(zoom);
    }
}

impl Zoom for Visuals {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.clip_rect_margin.zoom(zoom);
        self.menu_corner_radius.zoom(zoom);
        self.popup_shadow.zoom(zoom);
        self.resize_corner_size.zoom(zoom);
        self.selection.stroke.zoom(zoom);
        self.text_cursor.zoom(zoom);
        self.widgets.zoom(zoom);
        self.window_corner_radius.zoom(zoom);
        self.window_shadow.zoom(zoom);
        self.window_stroke.zoom(zoom);
    }
}

impl Zoom for ScrollStyle {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.bar_inner_margin.zoom(zoom);
        self.bar_outer_margin.zoom(zoom);
        self.bar_width.zoom(zoom);
        self.floating_allocated_width.zoom(zoom);
        self.floating_width.zoom(zoom);
        self.handle_min_length.zoom(zoom);
    }
}

impl Zoom for Spacing {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.button_padding.zoom(zoom);
        self.combo_height.zoom(zoom);
        self.combo_width.zoom(zoom);
        self.icon_spacing.zoom(zoom);
        self.icon_width.zoom(zoom);
        self.icon_width_inner.zoom(zoom);
        self.indent.zoom(zoom);
        self.interact_size.zoom(zoom);
        self.item_spacing.zoom(zoom);
        self.menu_margin.zoom(zoom);
        self.scroll.zoom(zoom);
        self.slider_width.zoom(zoom);
        self.text_edit_width.zoom(zoom);
        self.tooltip_width.zoom(zoom);
        self.window_margin.zoom(zoom);
    }
}

impl Zoom for FontId {
    fn zoom(&mut self, zoom: f32) {
        self.size.zoom(zoom);
    }
}

impl Zoom for Style {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        if let Some(font_id) = &mut self.override_font_id {
            font_id.zoom(zoom);
        }
        for font_id in self.text_styles.values_mut() {
            font_id.zoom(zoom);
        }
        self.interaction.zoom(zoom);
        self.spacing.zoom(zoom);
        self.visuals.zoom(zoom);
    }
}

impl<T> Zoom for Option<T>
where
    T: Zoom,
{
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        if let Some(value) = self {
            value.zoom(zoom);
        }
    }
}

impl Zoom for Frame {
    #[inline]
    fn zoom(&mut self, zoom: f32) {
        self.inner_margin.zoom(zoom);
        self.outer_margin.zoom(zoom);
        self.corner_radius.zoom(zoom);
        self.shadow.zoom(zoom);
        self.stroke.zoom(zoom);
    }
}
