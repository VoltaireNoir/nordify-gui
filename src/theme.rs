use iced::{
    Color,Background,overlay,
    widget::{
        container,text_input,button,scrollable,text,pick_list}
};

pub const JUST_GREY: Color = Color { r: 0.65, g: 0.65, b: 0.65, a: 1. };
pub const D_GREY: Color = Color { r: 0.18, g: 0.203, b: 0.250, a: 1. };
pub const LD_GREY: Color = Color { r: 0.231, g: 0.258, b: 0.321, a: 1. };
pub const L_GREY: Color = Color { r: 0.262, g: 0.298, b: 0.368, a: 1. };
pub const GREY: Color = Color { r: 0.298, g: 0.337, b: 0.415, a: 1. };
pub const WHITE: Color = Color { r: 0.925, g: 0.937, b: 0.956, a: 1. };
pub const L_WHITE: Color = Color { r: 0.898, g: 0.913, b: 0.941, a: 1. };
pub const LL_WHITE: Color = Color { r: 0.847, g: 0.870, b: 0.913, a: 1. };
pub const BLUE: Color = Color { r: 0.533, g: 0.752, b: 0.815, a: 1. };
pub const RED: Color = Color { r: 0.749, g: 0.380, b: 0.415, a: 1. };
pub const GREEN: Color = Color { r: 0.639, g: 0.745, b: 0.549, a: 1. };
pub const YELLOW: Color = Color { r: 0.921, g: 0.796, b: 0.545, a: 1. };

#[derive(Default)]
pub struct NordTheme;

impl iced::application::StyleSheet for NordTheme {
    type Style = ();
    fn appearance(&self, _style: Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: BLUE,
            text_color: D_GREY,
        }
    }
}

#[derive(Default,Copy,Clone)]
pub enum ContainerType {
    #[default]
    Other,
    Bottom,
    Inner,
    Tooltip,
}

impl container::StyleSheet for NordTheme {
    type Style = ContainerType;

    fn appearance(&self, style: Self::Style) -> container::Appearance {
        match style {
            ContainerType::Bottom => {
                container::Appearance {
                    background: Some(Background::Color(D_GREY)),
                    border_width: 0.,
                    ..Default::default()
                }
            },
            ContainerType::Inner => {
                container::Appearance {
                    background: Some(Background::Color(LD_GREY)),
                    border_radius: 3.5,
                    border_width: 2.,
                    border_color: LD_GREY,
                    ..Default::default()
                }
            },
            ContainerType::Tooltip => {
                let clr = { let mut c = D_GREY; c.a = 0.9; c };
                container::Appearance {
                    background: Some(Background::Color(clr)),
                    border_color: clr,
                    text_color: Some(YELLOW),
                    ..Default::default()
                }
            }
            _ => container::Appearance::default()
        }
    }
}

#[derive(Default,Clone, Copy)]
pub enum TextInputType {
    #[default]
    BrowserBar,
    FileName { valid: bool },
}

impl text_input::StyleSheet for NordTheme {
    type Style = TextInputType;

    fn active(&self, style: Self::Style) -> text_input::Appearance {
        match style {
            TextInputType::FileName { .. } => {
                text_input::Appearance {
                    background: Background::Color(D_GREY),
                    border_color: LD_GREY,
                    border_radius: 0.,
                    border_width: 2.,
                }
            },
            TextInputType::BrowserBar => {
                text_input::Appearance {
                    background: Background::Color(LD_GREY),
                    border_radius: 3.5,
                    border_width: 2.,
                    border_color: LD_GREY,
                }
            }
        }
    }

    fn focused(&self, style: Self::Style) -> text_input::Appearance {
        match style {
            TextInputType::FileName { .. } => {
                text_input::Appearance {
                    background: Background::Color(D_GREY),
                    border_color: L_GREY,
                    border_radius: 0.,
                    border_width: 2.,
                }
            },
            TextInputType::BrowserBar => {
                text_input::Appearance {
                    background: Background::Color(L_GREY),
                    border_radius: 3.5,
                    border_width: 2.,
                    border_color: L_GREY,
                }
            }
        }
    }

    fn value_color(&self, style: Self::Style) -> Color {
        match style {
            TextInputType::FileName { valid } => {
                if valid { GREEN } else { RED }
            },
            TextInputType::BrowserBar => {
                JUST_GREY
            },
        }
    }

    fn placeholder_color(&self, style: Self::Style) -> Color {
        match style {
            TextInputType::FileName { .. } => Color { r: 0.847, g: 0.870, b: 0.913, a: 0.7 },
            TextInputType::BrowserBar => JUST_GREY,
        }

    }

    fn selection_color(&self, style: Self::Style) -> Color {
        match style {
            TextInputType::FileName { .. } => Color { r: 0.533, g: 0.752, b: 0.815, a: 0.7 },
            TextInputType::BrowserBar => Color { r: 0.533, g: 0.752, b: 0.815, a: 0.7 },
        }

    }
}

#[derive(Copy, Clone)]
pub enum ButtonType {
    MainButton { btype: MainType },
    Content { selected: bool },
}


#[derive(Clone, Copy)]
pub enum MainType {
    Preview,
    Save,
    Reset
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::MainButton { btype: MainType::Preview }
    }
}

impl button::StyleSheet for NordTheme {
    type Style = ButtonType;

    fn active(&self, style: Self::Style) -> button::Appearance {
        match style {
            ButtonType::Content { selected } => {
                let text_color = if selected { BLUE } else { WHITE };
                button::Appearance {
                    background: Some(Background::Color(GREY)),
                    border_radius: 3.5,
                    border_width: 2.,
                    border_color: GREY,
                    text_color,
                    ..Default::default()
                }
            },

            ButtonType::MainButton { btype } => {
                let text_color = match btype {
                    MainType::Save => BLUE,
                    MainType::Preview => RED,
                    MainType::Reset => YELLOW,
                };

                button::Appearance {
                    background: None,
                    text_color,
                    ..Default::default()
                }
            }
        }

    }

    fn hovered(&self, style: Self::Style) -> button::Appearance {
        match style {
            ButtonType::Content { .. } => {
                button::Appearance {
                    background: Some(Background::Color(L_GREY)),
                    border_radius: 3.5,
                    border_width: 2.,
                    border_color: L_GREY,
                    text_color: BLUE,
                    ..Default::default()
                }
            },
            ButtonType::MainButton { btype } => {
                let text_color = match btype {
                    MainType::Preview | MainType::Reset => BLUE,
                    MainType::Save => GREEN,
                };
                button::Appearance {
                    background: None,
                    text_color,
                    ..Default::default()
                }
            }
        }
    }
}

impl scrollable::StyleSheet for NordTheme {
    type Style = ();
    fn active(&self, _style: Self::Style) -> scrollable::Scrollbar {
        let fg = { let mut c = L_WHITE; c.a = 0.8; c };

        scrollable::Scrollbar {
            background: Some(Background::Color(L_GREY)),
            border_radius: 5.0,
            border_width: 2.,
            border_color: L_GREY,
            scroller: scrollable::Scroller {
                color: fg,
                border_radius: 5.0,
                border_width: 2.,
                border_color: fg,
            }
        }
    }

    fn hovered(&self, _style: Self::Style) -> scrollable::Scrollbar {
        let mut scr = self.active(());
        scr.scroller.color = L_WHITE;
        scr.scroller.border_color = L_WHITE;
        scr
    }

    fn dragging(&self, _style: Self::Style) -> scrollable::Scrollbar {
        let mut scr = self.hovered(());
        scr.scroller.color = BLUE;
        scr.scroller.border_color = BLUE;
        scr
    }
}

impl pick_list::StyleSheet for NordTheme {
    type Style = ();

    fn active(&self, _style: <Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: LL_WHITE,
            placeholder_color: L_GREY,
            background: Background::Color(L_GREY),
            border_radius: 3.5,
            border_width: 2.,
            border_color: L_GREY,
            icon_size: 0.5,
        }
    }

    fn hovered(&self, _style: <Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            background: Background::Color(LD_GREY),
            border_color: LD_GREY,
            text_color: BLUE,
            ..self.active(())
        }
    }
}

impl overlay::menu::StyleSheet for NordTheme {
    type Style = ();
    fn appearance(&self, _style: Self::Style) -> overlay::menu::Appearance {
        overlay::menu::Appearance {
            text_color: LL_WHITE,
            background: Background::Color(LD_GREY),
            border_width: 2.,
            border_radius: 3.5,
            border_color: LD_GREY,
            selected_text_color: BLUE,
            selected_background: Background::Color(L_GREY),
        }
    }
}

#[derive(Default,Clone,Copy)]
pub enum TextType {
    Label,
    #[default]
    Other,
}

impl text::StyleSheet for NordTheme {
    type Style = TextType;
    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            TextType::Other => text::Appearance { color: None },
            TextType::Label => text::Appearance { color: Some(JUST_GREY) },
        }

    }
}
