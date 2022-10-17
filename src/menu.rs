use std::path::PathBuf;
use iced::{
    Length,alignment::Horizontal,
    widget::{container,row,text,button,column,text_input,pick_list}, Command,
};
use tempfile::TempDir;
use whatsinaname::AboutFile;

use crate:: { browser::Browser, preview::{Previews,ImageView}, theme, Event };

#[derive(Clone, Debug)]
pub enum MenuEvent {
    Preview,
    Save,
    Reset,
    SelectMode(Mode),
    FilenameChanged(String),
    FocusFileName,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Default,
    Creative,
    Knn,
}

impl Mode {
    const ALL: [Mode;3] = [Self::Default,Self::Creative,Self::Knn];
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Creative => "Creative",
            Self::Default => "Default",
            Self::Knn => "kNN",
        };
        write!(f, "{name}")
    }
}

pub struct Menu {
    pub config: Config,
    temp: TempDir,
    filename_id: text_input::Id,
}

impl Default for Menu {
    fn default() -> Self {
       Menu {
           temp: tempfile::tempdir().unwrap(),
           config: Default::default(),
           filename_id: text_input::Id::unique(),
       }
    }
}

impl Menu {
    pub fn view(&self) -> crate::IcedElement {
        let top = row![
                    container(
                        text("MODES")
                            .vertical_alignment(iced::alignment::Vertical::Top)
                            .horizontal_alignment(Horizontal::Left)
                            .style(theme::TextType::Label)
                            .size(16)
                    )
                        .width(Length::Fill)
                        .padding(3),
                    button(text("PREVIEW")
                           .vertical_alignment(iced::alignment::Vertical::Top)
                           .horizontal_alignment(Horizontal::Right)
                           .width(Length::Fill)
                           .size(19)
                    )
                .on_press(Event::Menu(MenuEvent::Preview))
                .style(theme::ButtonType::MainButton { btype: theme::MainType::Preview }),
                ]
                    .width(Length::Fill)
                    .spacing(5);

        let modes = pick_list(
                    Mode::ALL.to_vec(),
                    Some(self.config.mode),
                    |m| Event::Menu(MenuEvent::SelectMode(m))
                )
                    .width(Length::Fill);

        let filename = text_input("filename", &self.config.filename, |s| Event::Menu(MenuEvent::FilenameChanged(s)))
                    .width(Length::FillPortion(25))
                    .style(theme::TextInputType::FileName { valid: self.config.filename.is_valid_image() })
                    .id(self.filename_id.clone())
                    .size(16)
                    .padding(8);

        let save_reset = row![
                    container(
                        button(text("SAVE").horizontal_alignment(Horizontal::Center).size(19))
                            .on_press(Event::Menu(MenuEvent::Save))
                            .style(theme::ButtonType::MainButton { btype: theme::MainType::Save })
                    )
                        .width(Length::Fill)
                        .align_x(Horizontal::Center),
                    container(
                        button(text("RESET").horizontal_alignment(Horizontal::Center).size(19))
                            .on_press(Event::Menu(MenuEvent::Reset))
                            .style(theme::ButtonType::MainButton { btype: theme::MainType::Reset })
                    )
                        .width(Length::Fill)
                        .align_x(Horizontal::Center),
                ];

        container(
            column![top, modes, filename, save_reset]
                .padding(10)
                .spacing(6)
        )
            .style(theme::ContainerType::Bottom)
            .width(Length::FillPortion(25))
            .height(Length::FillPortion(50))
            .into()
    }

    pub fn update(&mut self, previews: &mut Previews, browser: &mut Browser, event: MenuEvent) -> Command<Event> {
        match event {
            MenuEvent::Preview => {
                if !browser.selected.is_empty() {
                    let r: String = (0..3).map(|_| fastrand::alphanumeric()).collect();
                    let loc = format!("{}/nordified{r}.png", self.temp.path().display());
                    previews.nordified.set_loc(&loc);
                    let predictor: nordify::Predictor = match self.config.mode {
                        Mode::Default => nordify::color_predictv2,
                        Mode::Creative => nordify::color_predict,
                        Mode::Knn => nordify::color_predictv2,
                    };
                    nordify::nordify(browser.selected.clone(), Some(loc), predictor, None);
                }
            },
            MenuEvent::Save => {
                if !browser.selected.is_empty() && self.config.filename.is_valid_image() {
                    let mut loc = browser.addrbar.addr.clone();
                    loc.push(&self.config.filename);
                    let predictor: nordify::Predictor = match self.config.mode {
                        Mode::Default => nordify::color_predictv2,
                        Mode::Creative => nordify::color_predict,
                        Mode::Knn => nordify::color_predictv2,
                    };
                    nordify::nordify(
                        browser.selected.clone(),
                        Some(dbg!(loc.to_string_lossy().to_string())),
                        predictor,
                        None
                    );
                    browser.reload_contents();
                }
            },
            MenuEvent::Reset => {
                self.config.mode = Default::default();
                if !browser.selected.is_empty() {
                    let sel = PathBuf::from(&browser.selected);
                    let filename = sel.file_name().unwrap().to_string_lossy();
                    let (name, _) = filename.split_at(
                        filename.rfind('.').unwrap()
                    );
                    self.config.filename = format!("{name}_nordified.png");
                }
            },
            MenuEvent::SelectMode(m) => self.config.mode = m,

            MenuEvent::FilenameChanged(s) => self.config.filename = s,

            MenuEvent::FocusFileName => return text_input::focus(self.filename_id.clone()),
        }

        Command::none()
    }
}

#[derive(Default)]
pub struct Config {
    mode: Mode,
    pub filename: String,
}
