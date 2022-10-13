use iced::{Length,Settings,Color,keyboard, executor};
use iced::pure::{Element,column,row, Application};

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

impl Application for NordifyGUI {
    type Message = Event;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (NordifyGUI::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        "Nordify".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Event::Browser(event) => self.browser.update(&mut self.previews, &mut self.menu, event),
            Event::Menu(event) => self.menu.update(&mut self.previews, &mut self.browser, event),
        };

        iced::Command::none()
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
