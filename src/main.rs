use iced::{Length,Color,Settings};
use iced::pure::{Sandbox,Element,text,column,row};

mod menu;
mod browser;
mod preview;
mod theme;

use browser::{Browser,BrowserEvent};
use menu::{Menu,MenuEvent};
use preview::{NordifiedImage,OriginalImage,ImageView};

fn main() {
    NordifyGUI::run(Settings::default())
        .unwrap()
}

#[derive(Default)]
struct NordifyGUI {
    original: OriginalImage,
    nordified: NordifiedImage,
    browser: Browser,
    menu: Menu,
}

#[derive(Clone, Debug)]
enum Event {
    ShowPreview,
    Browser(BrowserEvent),
    Menu(MenuEvent),
}

impl Sandbox for NordifyGUI {
    type Message = Event;

    fn new() -> Self {
        NordifyGUI::default()
    }

    fn title(&self) -> String {
        "Nordify".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Event::Browser(event) => self.browser.update(event),
            _ => (),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let spacing = 6;
        column()
            .push(
                row()
                    .push(
                        self.original.view()
                    )
                    .push(
                        self.nordified.view()
                    )
                    .spacing(spacing)
                    .height(Length::FillPortion(50))
            )
            .push(
                row()
                    .push(
                        self.browser.view()
                    )
                    .push(
                        text("Controls")
                            .width(Length::FillPortion(25))
                            .height(Length::FillPortion(50))
                    )
                    .spacing(spacing)
                    .height(Length::FillPortion(50))
            )
            .spacing(spacing)
            .padding(spacing)
            .into()
    }

    fn background_color(&self) -> iced::Color {
        Color { r: 0.533, g: 0.752, b: 0.815, a: 1. }
    }

}

