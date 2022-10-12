use iced::{ContentFit,Length,Color};
use iced::pure::{image,text,Element,container,row,horizontal_space, vertical_space, column};
use iced_aw::floating_button::Offset;
use iced_aw::pure::FloatingElement;

use crate::theme::{BtmContainerStyle,PreviewLabelStyle};
use crate::Event;

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

    fn view(&self) -> Element<'_,Event> {
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
    pub fn view(&self) -> Element<'_,Event> {
        FloatingElement::new(self.image_view(),|| self.labels())
            .offset(Offset {x: -6., y:-6.})
            .into()
    }

    fn image_view(&self) -> Element<'_,Event> {
        row()
            .push(
                self.original.view()
            )
            .push(
                self.nordified.view(),
            )
            .spacing(6)
            .height(Length::FillPortion(50))
            .into()
    }

    fn labels(&self) -> Element<'_,Event> {
        container(
            row()
                .push(
                    container(
                        text("ORIGINAL")
                            .color(Color { r: 0.847, g: 0.870, b: 0.913, a: 0.65 }))
                        .align_x(iced::alignment::Horizontal::Left)
                        .center_y()
                        .width(Length::FillPortion(1))
                        .height(Length::Units(40))
                        .style(PreviewLabelStyle)
                        .padding(10)
                )
                .push(
                    container(
                        text("NORDIFIED")
                            .color(Color { r: 0.847, g: 0.870, b: 0.913, a: 0.65 })
                    )
                        .align_x(iced::alignment::Horizontal::Right)
                        .center_y()
                        .width(Length::FillPortion(1))
                        .height(Length::Units(40))
                        .style(PreviewLabelStyle)
                        .padding(10)
                )
                .spacing(6)
        )
            .padding(6)
            .into()
    }
}
