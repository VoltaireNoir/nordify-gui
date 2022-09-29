use iced::{ContentFit,Length};
use iced::pure::{image,widget::Image};

pub struct NordifiedImage {
    loc: String,
}

impl ImageView for NordifiedImage {
    fn location(&self) -> &str {
        &self.loc
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
}

impl Default for OriginalImage {
    fn default() -> Self {
        OriginalImage { loc: "nomanssky.jpg".into() }
    }
}

pub trait ImageView {
    fn location(&self) -> &str;

    fn view(&self) -> Image {
        image(self.location())
            .content_fit(ContentFit::Fill)
            .width(Length::FillPortion(50))
            .height(Length::FillPortion(50))
    }
}
