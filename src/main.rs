use iced::{Length,Settings,Element,Application,executor,Event as KeyEvent, Renderer, Command};
use iced::widget::{column,row};
use iced::keyboard::{self, KeyCode, Modifiers};

mod menu;
mod browser;
mod preview;
mod theme;

use browser::{Browser,BrowserEvent};
use menu::{Menu,MenuEvent};
use preview::Previews;
use theme::*;

pub type IcedElement<'a> = Element<'a, Event, iced::Renderer<NordTheme>>;

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
    type Theme = NordTheme;

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
            Event::Quit => { self.exit = true; Command::none() },
        }
    }

    fn view(&self) -> IcedElement {
        let spacing = 6;
        column![
                self.previews.view(),
            row!(
                self.browser.view(),
                self.menu.view(),
            )
                .spacing(spacing)
                .height(Length::FillPortion(50))
            ]
            .spacing(spacing)
            .padding(spacing)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events_with(|e,_| {
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

    fn theme(&self) -> Self::Theme {
        NordTheme
    }
}

fn keyboard_event_handler(key: KeyCode, modifier: Modifiers) -> Option<Event> {
    match modifier {
        Modifiers::CTRL => {
            use menu::MenuEvent::{Preview,Reset,Save};
            use BrowserEvent::DelSelected;

            match key {
                KeyCode::P => Some(Event::Menu(Preview)),
                KeyCode::R => Some(Event::Menu(Reset)),
                KeyCode::S => Some(Event::Menu(Save)),
                KeyCode::Delete => Some(Event::Browser(DelSelected)),
                KeyCode::Q => Some(Event::Quit),
                KeyCode::L => Some(Event::Browser(BrowserEvent::FocusAddrBar)),
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
