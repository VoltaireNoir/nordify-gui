use iced::{Length,Settings,Color};
use iced::pure::{Sandbox,Element,column,row};

mod menu;
mod browser;
mod preview;
mod theme;

use browser::{Browser,BrowserEvent};
use menu::{Menu,MenuEvent};
use preview::Previews;

fn main() {
    NordifyGUI::run(Settings::default())
        .expect("Failed to run Nordify GUI")
}

#[derive(Default)]
struct NordifyGUI {
    previews: Previews,
    browser: Browser,
    menu: Menu,
}

#[derive(Clone, Debug)]
pub enum Event {
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
            Event::Browser(event) => self.browser.update(&mut self.previews, &mut self.menu, event),
            Event::Menu(event) => self.menu.update(&mut self.previews, &mut self.browser, event),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let spacing = 6;
        column()
            .push(
                self.previews.view()
            )
            .push(
                row()
                    .push(
                        self.browser.view()
                    )
                    .push(
                        self.menu.view()
                    )
                    .spacing(spacing)
                    .height(Length::FillPortion(50))
            )
            .spacing(spacing)
            .padding(spacing)
            .into()
    }

    fn background_color(&self) -> Color {
        theme::BLUE
    }
}
