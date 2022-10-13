use iced::{Length,Settings,Color,executor};
use iced::pure::{Element,column,row, Application};
use iced_native::keyboard::{self, KeyCode, Modifiers};
use iced_native::Event as KeyEvent;

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
    exit: bool,
}

#[derive(Clone, Debug)]
pub enum Event {
    Browser(BrowserEvent),
    Menu(MenuEvent),
    Quit,
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
            Event::Quit => self.exit = true,
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

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced_native::subscription::events_with(|e,_| {
            match e {
                KeyEvent::Keyboard(keyboard::Event::KeyPressed { key_code, modifiers })
                    => keyboard_event_handler(key_code, modifiers),
                _ => None
            }
        })
    }

    fn should_exit(&self) -> bool {
        self.exit
    }
}

fn keyboard_event_handler(key: KeyCode, modifier: Modifiers) -> Option<Event> {
    match modifier {
        Modifiers::CTRL => {
            use menu::MenuEvent::{Preview,Reset,Save};

            match key {
                KeyCode::P => Some(Event::Menu(Preview)),
                KeyCode::R => Some(Event::Menu(Reset)),
                KeyCode::S => Some(Event::Menu(Save)),
                KeyCode::Q => Some(Event::Quit),
                _ => None,
            }
        },

        Modifiers::ALT => {
            use menu::{MenuEvent::SelectMode,Mode::*};

            match key {
                KeyCode::Key1 => Some(Event::Menu(SelectMode(Default))),
                KeyCode::Key2 => Some(Event::Menu(SelectMode(Creative))),
                KeyCode::Key3 => Some(Event::Menu(SelectMode(Knn))),
                _ => None,
            }
        },

        _ => None,
    }
}
