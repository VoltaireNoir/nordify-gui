use iced::{
    alignment::Horizontal,
    widget::{button, column, container, pick_list, row, slider, text, text_input},
    Command, Length,
};
use mapped::{mappers, Mapper, ProcOptions};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use whatsinaname::AboutFile;

use crate::{
    browser::Browser,
    preview::{ImageView, Previews},
    theme, Event,
};

#[derive(Clone, Debug)]
pub enum MenuEvent {
    Preview,
    Save,
    Reset,
    SelectMode(Mode),
    FilenameChanged(String),
    FocusFileName,
    SetKVal(UType),
}

#[derive(Clone, Debug)]
pub enum UType {
    Text(String),
    Num(u8),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Default,
    Creative,
    Knn,
}

impl Mode {
    const ALL: [Mode; 3] = [Self::Default, Self::Creative, Self::Knn];
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
            button(
                text("PREVIEW")
                    .vertical_alignment(iced::alignment::Vertical::Top)
                    .horizontal_alignment(Horizontal::Right)
                    .width(Length::Fill)
                    .size(19)
            )
            .on_press(Event::Menu(MenuEvent::Preview))
            .style(theme::ButtonType::MainButton {
                btype: theme::MainType::Preview
            }),
        ]
        .width(Length::Fill)
        .spacing(5);

        let modes = pick_list(Mode::ALL.to_vec(), Some(self.config.mode), |m| {
            Event::Menu(MenuEvent::SelectMode(m))
        })
        .width(Length::Fill);

        let filename = text_input("filename", &self.config.filename, |s| {
            Event::Menu(MenuEvent::FilenameChanged(s))
        })
        .width(Length::FillPortion(25))
        .style(theme::TextInputType::FileName {
            valid: self.config.filename.is_valid_file_with_ext(&crate::EXT),
        })
        .id(self.filename_id.clone())
        .size(16)
        .padding(8);

        let save_reset = row![
            container(
                button(
                    text("SAVE")
                        .horizontal_alignment(Horizontal::Center)
                        .size(19)
                )
                .on_press(Event::Menu(MenuEvent::Save))
                .style(theme::ButtonType::MainButton {
                    btype: theme::MainType::Save
                })
            )
            .width(Length::Fill)
            .align_x(Horizontal::Center),
            container(
                button(
                    text("RESET")
                        .horizontal_alignment(Horizontal::Center)
                        .size(19)
                )
                .on_press(Event::Menu(MenuEvent::Reset))
                .style(theme::ButtonType::MainButton {
                    btype: theme::MainType::Reset
                })
            )
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Horizontal::Center),
        ];

        container(
            column![top, modes, self.options(), filename, save_reset]
                .padding(10)
                .spacing(8),
        )
        .style(theme::ContainerType::Bottom)
        .width(Length::FillPortion(25))
        .height(Length::FillPortion(50))
        .into()
    }

    pub fn update(
        &mut self,
        previews: &mut Previews,
        browser: &mut Browser,
        event: MenuEvent,
    ) -> Command<Event> {
        match event {
            MenuEvent::Preview => {
                if !browser.selected.is_empty() {
                    let r: String = (0..3).map(|_| fastrand::alphanumeric()).collect();
                    let loc = format!("{}/nordified{r}.png", self.temp.path().display());
                    let (file, out): (&Path, &Path) = (browser.selected.as_ref(), loc.as_ref());
                    previews.nordified.set_loc(&loc);
                    match self.config.mode {
                        Mode::Default => nordify(ProcOptions::default(), file, out),
                        Mode::Creative => nordify(ProcOptions::new(mappers::Creative), file, out),
                        Mode::Knn => nordify(
                            ProcOptions::new(
                                mappers::Knn::with(self.config.kval as usize).memoized(),
                            ),
                            file,
                            out,
                        ),
                    }
                }
            }
            MenuEvent::Save => {
                if !browser.selected.is_empty()
                    && self.config.filename.is_valid_file_with_ext(&crate::EXT)
                {
                    let mut loc = browser.addrbar.addr.to_path_buf();
                    loc.push(&self.config.filename);
                    let (file, out): (&Path, &Path) = (browser.selected.as_ref(), loc.as_path());
                    match self.config.mode {
                        Mode::Default => nordify(ProcOptions::default(), file, out),
                        Mode::Creative => nordify(ProcOptions::new(mappers::Creative), file, out),
                        Mode::Knn => nordify(
                            ProcOptions::new(
                                mappers::Knn::with(self.config.kval as usize).memoized(),
                            ),
                            file,
                            out,
                        ),
                    };
                    browser.reload_contents();
                }
            }
            MenuEvent::Reset => {
                self.config = Default::default();
                if !browser.selected.is_empty() {
                    let sel = PathBuf::from(&browser.selected);
                    let filename = sel.file_name().unwrap().to_string_lossy();
                    let (name, _) = filename.split_at(filename.rfind('.').unwrap());
                    self.config.filename = format!("{name}_nordified.png");
                }
            }
            MenuEvent::SelectMode(m) => self.config.mode = m,

            MenuEvent::SetKVal(update) => match update {
                UType::Num(n) => self.config.kval = n,
                UType::Text(t) => {
                    if let Ok(n) = t.parse::<u8>() {
                        self.config.kval = n;
                    }
                }
            },

            MenuEvent::FilenameChanged(s) => self.config.filename = s,

            MenuEvent::FocusFileName => return text_input::focus(self.filename_id.clone()),
        }

        Command::none()
    }

    fn options(&self) -> crate::IcedElement {
        if self.config.mode == Mode::Knn {
            container(
                container(
                    row![
                        container(text("K").style(theme::TextType::Option).size(20))
                            .padding(2)
                            .style(theme::ContainerType::Options),
                        text_input("val", &self.config.kval.to_string(), |s| Event::Menu(
                            MenuEvent::SetKVal(UType::Text(s))
                        ))
                        .width(Length::Units(30))
                        .style(theme::TextInputType::BrowserBar),
                        slider(1..=255, self.config.kval, |v| Event::Menu(
                            MenuEvent::SetKVal(UType::Num(v))
                        ))
                    ]
                    .spacing(10)
                    .padding(10),
                )
                .center_y(),
            )
            .style(theme::ContainerType::Inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            container(
                text("No additional options \navailable for the \nselected mode")
                    .size(14)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center),
            )
            .style(theme::ContainerType::Inner)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
    }
}

pub struct Config {
    mode: Mode,
    pub filename: String,
    kval: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: Default::default(),
            filename: Default::default(),
            kval: 32,
        }
    }
}

fn nordify<M: Mapper>(opts: ProcOptions<M>, file: &Path, out: &Path) {
    opts.load(file)
        .expect("failed to load file")
        .process()
        .save(out)
        .expect("failed to save")
}
