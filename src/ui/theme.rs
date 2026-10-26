//! Dark IDE Theme & Styling for LocalSQL Studio
//!
//! Provides a modern, high-contrast, clean dark theme tailored for data analysis.

use egui::{Color32, Context, FontFamily, FontId, Margin, Rounding, Stroke, TextStyle, Visuals};

pub const COLOR_BG: Color32 = Color32::from_rgb(18, 20, 24);
pub const COLOR_PANEL_BG: Color32 = Color32::from_rgb(26, 29, 36);
pub const COLOR_SURFACE: Color32 = Color32::from_rgb(33, 37, 46);
pub const COLOR_SURFACE_HOVER: Color32 = Color32::from_rgb(42, 47, 58);
pub const COLOR_BORDER: Color32 = Color32::from_rgb(48, 54, 66);
pub const COLOR_PRIMARY: Color32 = Color32::from_rgb(59, 130, 246);
pub const COLOR_PRIMARY_HOVER: Color32 = Color32::from_rgb(37, 99, 235);
pub const COLOR_SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);
pub const COLOR_WARNING: Color32 = Color32::from_rgb(234, 179, 8);
pub const COLOR_ERROR: Color32 = Color32::from_rgb(239, 68, 68);
pub const COLOR_TEXT_PRIMARY: Color32 = Color32::from_rgb(243, 244, 246);
pub const COLOR_TEXT_MUTED: Color32 = Color32::from_rgb(156, 163, 175);
pub const COLOR_CODE_BG: Color32 = Color32::from_rgb(15, 17, 21);

/// Configures egui context with our custom dark IDE theme.
pub fn apply_theme(ctx: &Context) {
    let mut visuals = Visuals::dark();

    // Panel backgrounds
    visuals.panel_fill = COLOR_PANEL_BG;
    visuals.window_fill = COLOR_PANEL_BG;
    visuals.faint_bg_color = COLOR_SURFACE;
    visuals.extreme_bg_color = COLOR_CODE_BG;

    // Widgets (buttons, text fields, checkboxes)
    visuals.widgets.noninteractive.bg_fill = COLOR_PANEL_BG;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, COLOR_BORDER);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, COLOR_TEXT_MUTED);
    visuals.widgets.noninteractive.rounding = Rounding::same(4.0);

    visuals.widgets.inactive.bg_fill = COLOR_SURFACE;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, COLOR_BORDER);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, COLOR_TEXT_PRIMARY);
    visuals.widgets.inactive.rounding = Rounding::same(4.0);

    visuals.widgets.hovered.bg_fill = COLOR_SURFACE_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, COLOR_PRIMARY);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    visuals.widgets.hovered.rounding = Rounding::same(4.0);

    visuals.widgets.active.bg_fill = COLOR_PRIMARY_HOVER;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, COLOR_PRIMARY);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    visuals.widgets.active.rounding = Rounding::same(4.0);

    visuals.widgets.open.bg_fill = COLOR_SURFACE;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0_f32, COLOR_BORDER);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0_f32, COLOR_TEXT_PRIMARY);

    visuals.selection.bg_fill = COLOR_PRIMARY;
    visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);

    ctx.set_visuals(visuals);

    // Typography & Spacing configuration
    ctx.style_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.window_margin = Margin::same(8.0);
        style.spacing.button_padding = egui::vec2(10.0, 5.0);

        // Crisp typography
        style.text_styles = [
            (TextStyle::Small, FontId::new(11.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(13.0, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(13.0, FontFamily::Proportional)),
            (TextStyle::Heading, FontId::new(16.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace)),
        ]
        .into();
    });
}
