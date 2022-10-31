use iced::widget::{image, image::Handle, row, tooltip};
use iced::{ContentFit, Length};

use crate::theme;

static NORD_SRC: &[u8] = include_bytes!("../media/preview2.png");
static ORIG_SRC: &[u8] = include_bytes!("../media/preview1.png");

#[derive(Default)]
pub struct NordifiedImage {
    loc: String,
}

impl ImageView for NordifiedImage {
    fn src(&self) -> &[u8] {
        NORD_SRC
    }

    fn location(&self) -> &str {
        &self.loc
    }

    fn set_loc(&mut self, new: &str) {
        self.loc.clear();
        self.loc.push_str(new);
    }
}

#[derive(Default)]
pub struct OriginalImage {
    loc: String,
}

impl ImageView for OriginalImage {
    fn src(&self) -> &[u8] {
        ORIG_SRC
    }

    fn location(&self) -> &str {
        &self.loc
    }

    fn set_loc(&mut self, new: &str) {
        self.loc.clear();
        self.loc.push_str(new);
    }
}

pub trait ImageView {
    fn location(&self) -> &str;
    fn set_loc(&mut self, new: &str);
    fn src(&self) -> &[u8];

    fn view(&self) -> crate::IcedElement {
        let handle = if self.location().is_empty() {
            Handle::from_memory(self.src().to_vec())
        } else {
            Handle::from(self.location())
        };
        image(handle)
            .content_fit(ContentFit::Fill)
            .width(Length::FillPortion(50))
            .height(Length::FillPortion(50))
            .into()
    }
}

#[derive(Default)]
pub struct Previews {
    pub original: OriginalImage,
    pub nordified: NordifiedImage,
}

impl Previews {
    pub fn view(&self) -> crate::IcedElement {
        row!(
            tooltip(
                self.original.view(),
                "Original",
                tooltip::Position::FollowCursor
            )
            .size(16)
            .style(theme::ContainerType::Tooltip),
            tooltip(
                self.nordified.view(),
                "Nordified",
                tooltip::Position::FollowCursor
            )
            .size(16)
            .style(theme::ContainerType::Tooltip),
        )
        .spacing(6)
        .height(Length::FillPortion(50))
        .into()
    }
}
