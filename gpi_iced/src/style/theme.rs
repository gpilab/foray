use iced::{
    theme::{
        palette::{Background, Extended, Pair, Primary, Success},
        Palette,
    },
    Theme,
};
use serde::{Deserialize, Serialize};

use super::color::{mix, GuiColor};

#[derive(Clone, Serialize, Deserialize)]
pub struct AppTheme {
    pub background: GuiColor,
    pub text: GuiColor,
    pub primary: GuiColor,
    pub secondary: GuiColor,
    pub success: GuiColor,
    pub danger: GuiColor,
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme {
            background: GuiColor::new(16, 15, 15, 0.1, 0.4),
            text: GuiColor::new(206, 205, 195, 0.1, 0.4),
            primary: GuiColor::new(139, 126, 200, 0.1, 0.4),
            //secondary: GuiColor::new(203, 97, 32, 0.1, 0.4),
            secondary: GuiColor::new(206, 205, 195, 0.1, 0.4),
            success: GuiColor::new(135, 154, 57, 0.1, 0.4),
            danger: GuiColor::new(209, 77, 65, 0.1, 0.4),
        }
    }
}
impl From<AppTheme> for Theme {
    fn from(app_theme: AppTheme) -> Self {
        let palette = Palette {
            background: app_theme.background.base_color.into(),
            text: app_theme.text.base_color.into(),
            primary: app_theme.primary.base_color.into(),
            success: app_theme.success.base_color.into(),
            danger: app_theme.danger.base_color.into(),
        };
        iced::Theme::custom_with_fn("flexoki".into(), palette, move |palette| {
            extended(palette, app_theme)
        })
    }
}

pub fn extended(_palette: Palette, app_theme: AppTheme) -> Extended {
    let AppTheme {
        background,
        text,
        primary,
        secondary,
        success,
        danger,
    } = app_theme;

    let base = |gui_color: GuiColor| Pair::new(gui_color.base_color.into(), text.base_color.into());

    let weak = |gui_color: GuiColor| {
        Pair::new(
            mix(
                gui_color.base_color,
                text.base_color,
                gui_color.weak_modifier,
            )
            .into(),
            text.base_color.into(),
        )
    };
    let strong = |gui_color: GuiColor| {
        Pair::new(
            mix(
                gui_color.base_color,
                text.base_color,
                gui_color.strong_modifier,
            )
            .into(),
            text.base_color.into(),
        )
    };

    Extended {
        background: Background {
            base: base(background),
            weak: weak(background),
            strong: strong(background),
        },
        primary: Primary {
            base: base(primary),
            weak: weak(primary),
            strong: strong(primary),
        },
        secondary: iced::theme::palette::Secondary {
            base: base(secondary),
            weak: weak(secondary),
            strong: strong(secondary),
        },
        success: Success {
            base: base(success),
            weak: weak(success),
            strong: strong(success),
        },
        danger: iced::theme::palette::Danger {
            base: base(danger),
            weak: weak(danger),
            strong: strong(danger),
        },
        is_dark: true,
    }
}
