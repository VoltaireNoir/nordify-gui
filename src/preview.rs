use iced::{ContentFit,Length};
use iced::widget::{image,row,tooltip};

use crate::theme;

pub struct NordifiedImage {
    loc: String,
}

impl ImageView for NordifiedImage {
    fn location(&self) -> &str {
        &self.loc
    }

    fn set_loc(&mut self, new: &str) {
        self.loc.clear();
        self.loc.push_str(new);
    }
}

impl Default for NordifiedImage {
    fn default() -> Self {
        NordifiedImage { loc: "nomanssky_nordified.png".into() }
    }
}

pub struct OriginalImage {
    loc: String,
}

impl ImageView for OriginalImage {
    fn location(&self) -> &str {
        &self.loc
    }

    fn set_loc(&mut self, new: &str) {
        self.loc.clear();
        self.loc.push_str(new);
    }
}

impl Default for OriginalImage {
    fn default() -> Self {
        OriginalImage { loc: "nomanssky.jpg".into() }
    }
}

pub trait ImageView {
    fn location(&self) -> &str;
    fn set_loc(&mut self, new: &str);

    fn view(&self) -> crate::IcedElement {
        image(self.location())
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
            tooltip(self.original.view(),"Original",tooltip::Position::FollowCursor)
                .size(14)
                .style(theme::ContainerType::Tooltip),

            tooltip(self.nordified.view(),"Nordified",tooltip::Position::FollowCursor)
                .size(14)
                .style(theme::ContainerType::Tooltip),
        )
            .spacing(6)
            .height(Length::FillPortion(50))
            .into()
    }
}
