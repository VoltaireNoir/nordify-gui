use std::path::PathBuf;
use iced::{Length,alignment::Horizontal};
use iced::pure::{Element,container,row,text,button,column,text_input,radio,pick_list};
use tempfile::TempDir;
use whatsinaname::AboutFile;

use crate::browser::Browser;
use crate::preview::{Previews,ImageView};
use crate::theme;

use super::Event;
use super::theme::{BtmContainerStyle,JUST_GREY};

#[derive(Clone, Debug)]
pub enum MenuEvent {
    Preview,
    Save,
    Reset,
    SelectMode(Mode),
    FilenameChanged(String),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Default,
    Creative,
    Knn,
}

pub struct Menu {
    pub config: Config,
    temp: TempDir,
}

impl Default for Menu {
    fn default() -> Self {
       Menu { temp: tempfile::tempdir().unwrap(), config: Default::default() }
    }
}

impl Menu {
    pub fn view(&self) -> Element<Event> {
        container(
            column()
                .push(
                    row()
                        .push(
                            container(
                                text("MODES")
                                    .color(JUST_GREY)
                                    .vertical_alignment(iced::alignment::Vertical::Top)
                                    .horizontal_alignment(Horizontal::Left)
                                    .size(16)
                            )
                                .width(Length::Fill)
                                .padding(3)
                        )
                        .push(
                            button(text("PREVIEW")
                                   .vertical_alignment(iced::alignment::Vertical::Top)
                                   .horizontal_alignment(Horizontal::Right)
                                   .width(Length::Fill)
                                   .size(20)
                            )
                                .on_press(Event::Menu(MenuEvent::Preview))
                                .style(theme::MainButtonStyle::new(theme::BType::Preview))
                        )
                        .width(Length::Fill)
                        .spacing(5)
                )
                .push(
                    row()
                        .push(
                            container(
                                radio("Default", Mode::Default, Some(self.config.mode), |m| Event::Menu(MenuEvent::SelectMode(m)))
                                    .size(16)
                                    .spacing(4)
                                    .text_size(18)
                                    .style(theme::ModesStyle)
                            )
                                .width(Length::Fill)
                                .align_x(Horizontal::Center)
                                .padding(2)
                        )
                        .push(
                            container(
                                radio("Creative", Mode::Creative, Some(self.config.mode), |m| Event::Menu(MenuEvent::SelectMode(m)))
                                    .size(16)
                                    .spacing(4)
                                    .text_size(18)
                                    .style(theme::ModesStyle)
                            )
                                .width(Length::Fill)
                                .align_x(Horizontal::Center)
                                .padding(2)
                        )
                        .push(
                            container(
                                radio("kNN", Mode::Knn, Some(self.config.mode), |m| Event::Menu(MenuEvent::SelectMode(m)))
                                    .size(16)
                                    .spacing(4)
                                    .text_size(18)
                                    .style(theme::ModesStyle)
                            )
                                .align_x(Horizontal::Center)
                                .width(Length::Fill)
                                .padding(2)
                        )
                        .width(Length::Fill)
                        .padding(5)
                )
                .push(
                        text_input("filename", &self.config.filename, |s| Event::Menu(MenuEvent::FilenameChanged(s)))
                        .width(Length::FillPortion(25))
                        .size(16)
                        .padding(8)
                        .style(theme::FileInputStyle::new(self.config.filename.is_valid_image()))
                )
                .push(
                    row()
                        .push(
                            container(
                                button(text("SAVE").horizontal_alignment(Horizontal::Center).size(20))
                                    .on_press(Event::Menu(MenuEvent::Save))
                                    .style(theme::MainButtonStyle::new(theme::BType::Save))
                            )
                                .width(Length::Fill)
                                .align_x(Horizontal::Center)
                        )
                        .push(
                            container(
                                button(text("RESET").horizontal_alignment(Horizontal::Center).size(20))
                                    .on_press(Event::Menu(MenuEvent::Reset))
                                    .style(theme::MainButtonStyle::new(theme::BType::Reset))
                            )
                                .width(Length::Fill)
                                .align_x(Horizontal::Center)
                        )
                )
                .padding(10)
                .spacing(5)
        )
            .width(Length::FillPortion(25))
            .height(Length::FillPortion(50))
            .style(BtmContainerStyle)
            .into()
    }

    pub fn update(&mut self, previews: &mut Previews, browser: &mut Browser, event: MenuEvent) {
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
                if !browser.selected.is_empty() {
                    let sp = PathBuf::from(&browser.selected);
                    if self.config.filename.is_valid_image() {
                        let loc = format!("{}/{}",sp.parent().unwrap().display(),self.config.filename);
                        let predictor: nordify::Predictor = match self.config.mode {
                            Mode::Default => nordify::color_predictv2,
                            Mode::Creative => nordify::color_predict,
                            Mode::Knn => nordify::color_predictv2,
                        };
                        nordify::nordify(browser.selected.clone(), Some(loc), predictor, None);
                        browser.reload_contents();
                    }
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
        }
    }
}

#[derive(Default)]
pub struct Config {
    mode: Mode,
    pub filename: String,
}
