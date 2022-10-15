use iced::{
    Color,Background,
    widget::{
        container,text_input,button,radio,scrollable,pick_list}
};

pub const JUST_GREY: Color = Color { r: 0.65, g: 0.65, b: 0.65, a: 1. };
pub const D_GREY: Color = Color { r: 0.18, g: 0.203, b: 0.250, a: 1. };
pub const LD_GREY: Color = Color { r: 0.231, g: 0.258, b: 0.321, a: 1. };
pub const LL_GREY: Color = Color { r: 0.262, g: 0.298, b: 0.368, a: 1. };
pub const GREY: Color = Color { r: 0.298, g: 0.337, b: 0.415, a: 1. };
pub const WHITE: Color = Color { r: 0.925, g: 0.937, b: 0.956, a: 1. };
pub const L_WHITE: Color = Color { r: 0.898, g: 0.913, b: 0.941, a: 1. };
pub const LL_WHITE: Color = Color { r: 0.847, g: 0.870, b: 0.913, a: 1. };
pub const BLUE: Color = Color { r: 0.533, g: 0.752, b: 0.815, a: 1. };
pub const RED: Color = Color { r: 0.749, g: 0.380, b: 0.415, a: 1. };
pub const GREEN: Color = Color { r: 0.639, g: 0.745, b: 0.549, a: 1. };
pub const YELLOW: Color = Color { r: 0.921, g: 0.796, b: 0.545, a: 1. };

pub struct BtmContainerStyle;

impl container::StyleSheet for BtmContainerStyle {
    type Style = ();
    fn appearance(&self, _style: Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(D_GREY)),
            border_width: 0.,
            ..Default::default()
        }
    }
}

pub struct InnerContainerStyle;

impl container::StyleSheet for InnerContainerStyle {
    type Style = ();
    fn appearance(&self, _style: Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(LD_GREY)),
            border_radius: 3.5,
            border_width: 2.,
            border_color: LD_GREY,
            ..Default::default()
            }
    }
}

pub struct FileInputStyle {
    valid: bool,
}

impl FileInputStyle {
    pub fn new(valid: bool) -> Self {
        FileInputStyle { valid }
    }
}

impl text_input::StyleSheet for FileInputStyle {
    type Style = ();

    fn active(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(D_GREY),
            border_color: LD_GREY,
            border_radius: 0.,
            border_width: 2.,
        }
    }

    fn focused(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(D_GREY),
            border_color: LL_GREY,
            border_radius: 0.,
            border_width: 2.,
        }
    }

    fn value_color(&self, _style: Self::Style) -> Color {
        if self.valid { GREEN } else { RED }
    }

    fn placeholder_color(&self, _style: Self::Style) -> Color {
        Color { r: 0.847, g: 0.870, b: 0.913, a: 0.7 }
    }

    fn selection_color(&self, _style: Self::Style) -> Color {
        Color { r: 0.533, g: 0.752, b: 0.815, a: 0.7 }
    }
}

pub struct AddressBarStyle;

impl text_input::StyleSheet for AddressBarStyle {
    type Style = ();

    fn active(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(LD_GREY),
            border_radius: 3.5,
            border_width: 2.,
            border_color: LD_GREY,
        }
    }

    fn focused(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(LD_GREY),
            border_radius: 3.5,
            border_width: 2.,
            border_color: LD_GREY,
        }
    }

    fn value_color(&self, _style: Self::Style) -> Color {
        JUST_GREY
    }

    fn placeholder_color(&self, _style: Self::Style) -> Color {
        JUST_GREY
    }

    fn selection_color(&self, _style: Self::Style) -> Color {
        Color { r: 0.533, g: 0.752, b: 0.815, a: 0.7 }
    }
}

pub struct ContentButtonStyle {
    selected: bool,
}

impl ContentButtonStyle {
    pub fn new(selected: bool) -> Self {
        ContentButtonStyle { selected }
    }
}

impl button::StyleSheet for ContentButtonStyle {
    type Style = ();
    fn active(&self, style: Self::Style) -> button::Appearance {
        let text_color = if self.selected { BLUE } else { WHITE };

        button::Appearance {
            background: Some(Background::Color(GREY)),
            border_radius: 3.5,
            border_width: 2.,
            border_color: GREY,
            text_color,
            ..Default::default()
        }
    }

    fn hovered(&self, style: Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(LL_GREY)),
            border_radius: 3.5,
            border_width: 2.,
            border_color: LL_GREY,
            text_color: BLUE,
            ..Default::default()
        }
    }
}

pub struct MainButtonStyle {
    btype: BType,
}

impl MainButtonStyle {
    pub fn new(btype: BType) -> Self {
        MainButtonStyle { btype }
    }
}

pub enum BType {
    Preview,
    Save,
    Reset
}

impl button::StyleSheet for MainButtonStyle {
    type Style = ();
    fn active(&self, _style: Self::Style) -> button::Appearance {
        let text_color = match self.btype {
            BType::Save => BLUE,
            BType::Preview => RED,
            BType::Reset => YELLOW,
        };

        button::Appearance {
            background: None,
            text_color,
            ..Default::default()
        }
    }

    fn hovered(&self, style: Self::Style) -> button::Appearance {
        let text_color = match self.btype {
            BType::Preview | BType::Reset => BLUE,
            BType::Save => GREEN,
        };
        button::Appearance {
            background: None,
            text_color,
            ..Default::default()
        }
    }
}

pub struct ModesStyle;

impl radio::StyleSheet for ModesStyle {
    type Style = ();
    fn active(&self, style: Self::Style) -> radio::Appearance {
        radio::Appearance {
            dot_color: BLUE,
            text_color: Some(WHITE),
            background: Background::Color(D_GREY),
            border_color: D_GREY,
            border_width: 0.0,
        }
    }

    fn hovered(&self, style: Self::Style) -> radio::Appearance {
        radio::Appearance {
            dot_color: BLUE,
            text_color: Some(BLUE),
            background: Background::Color(D_GREY),
            border_color: D_GREY,
            border_width: 0.0,
        }
    }
}

pub struct ScrollableStyle;

impl scrollable::StyleSheet for ScrollableStyle {
    type Style = ();
    fn active(&self, _style: Self::Style) -> scrollable::Scrollbar {
        let fg = { let mut c = L_WHITE; c.a = 0.8; c };

        scrollable::Scrollbar {
            background: Some(Background::Color(LL_GREY)),
            border_radius: 5.0,
            border_width: 2.,
            border_color: LL_GREY,
            scroller: scrollable::Scroller {
                color: fg,
                border_radius: 5.0,
                border_width: 2.,
                border_color: fg,
            }
        }
    }

    fn hovered(&self, style: Self::Style) -> scrollable::Scrollbar {
        let mut scr = self.active(());
        scr.scroller.color = L_WHITE;
        scr.scroller.border_color = L_WHITE;
        scr
    }

    fn dragging(&self, style: Self::Style) -> scrollable::Scrollbar {
        let mut scr = self.hovered(());
        scr.scroller.color = BLUE;
        scr.scroller.border_color = BLUE;
        scr
    }
}
