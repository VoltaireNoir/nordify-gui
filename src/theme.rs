use iced::Color;

pub const D_GREY: Color = Color { r: 0.18, g: 0.203, b: 0.250, a: 1. };
pub const JUST_GREY: Color = Color { r: 0.65, g: 0.65, b: 0.65, a: 1. };

use iced::container;
use iced::Background;

pub struct BtmContainerStyle;

impl container::StyleSheet for BtmContainerStyle {
    fn style(&self) -> container::Style {
        container::Style{
            background: Some(Background::Color(D_GREY)),
            ..Default::default()
            }
    }
}
