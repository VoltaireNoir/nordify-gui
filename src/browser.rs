use iced::{
    Length,
    pure::widget::{svg, Column},
    pure::{button, column, container, row, scrollable, text, text_input, Element},
};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};
use whatsinaname::AboutFile;

use crate::menu::Menu;
use crate::preview::{ImageView, Previews};
use crate::theme::{self, BtmContainerStyle, InnerContainerStyle, JUST_GREY};
use crate::Event;

#[derive(Default)]
pub struct Browser {
    addrbar: AddressBar,
    contents: Contents,
    pub selected: String,
}

#[derive(Clone, Debug)]
pub enum BrowserEvent {
    AddrSubmit,
    AddrChanged(String),
    ContentClicked(usize),
    DirUp,
}

impl Browser {
    pub fn view(&self) -> Element<'_, Event> {
        container(
            column()
                .push(
                    row()
                        .push(
                            container(
                                text("BROWSE")
                                    .color(JUST_GREY)
                                    .size(16)
                                    .vertical_alignment(iced::alignment::Vertical::Center)
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    .height(Length::Fill),
                            )
                            .padding(3)
                            .align_y(iced::alignment::Vertical::Center),
                        )
                        .push(self.addrbar.view())
                        .spacing(12)
                        .width(Length::FillPortion(75)),
                )
                .push(self.contents.view())
                .width(Length::FillPortion(75))
                .padding(10)
                .spacing(5),
        )
        .width(Length::FillPortion(75))
        .height(Length::FillPortion(50))
        .style(BtmContainerStyle)
        .into()
    }

    pub fn update(&mut self, previews: &mut Previews, menu: &mut Menu, message: BrowserEvent) {
        match message {
            BrowserEvent::AddrChanged(v) => self.addrbar.value = v,

            BrowserEvent::AddrSubmit => {
                if PathBuf::from(&self.addrbar.value).is_dir() {
                    self.addrbar.addr.clear();
                    self.addrbar.addr.push(&self.addrbar.value);
                    self.contents.entries = Contents::get_contents(&self.addrbar.addr)
                }
            }
            BrowserEvent::ContentClicked(id) => {
                let entry = &self.contents.entries[id];
                match entry.ctype {
                    ContentType::Directory => {
                        let dir = entry.handle.path().to_string_lossy().to_string();
                        self.contents.entries = Contents::get_contents(&dir);
                        self.addrbar.addr.clear();
                        self.addrbar.addr.push(&dir);
                        self.addrbar.value = dir;
                    }
                    ContentType::Image => {
                        self.contents.clear_selection();
                        let entry = &mut self.contents.entries[id];
                        entry.selected = true;
                        let path = entry.handle.path().display().to_string();
                        previews.original.set_loc(&path);
                        self.selected.clear();
                        self.selected.push_str(&path);
                        let filename = entry.handle.file_name().to_string_lossy().to_string();
                        menu.config.filename = format!("{}_nordified.png",filename.get_name());
                    }
                    _ => (),
                }
            }
            BrowserEvent::DirUp => {
                if self.addrbar.addr.pop() {
                    self.addrbar.value = String::from(self.addrbar.addr.to_string_lossy());
                    self.contents.entries = Contents::get_contents(&self.addrbar.addr);
                }
            }
        }
    }

    pub fn reload_contents(&mut self) {
        self.contents.entries = Contents::get_contents(&self.addrbar.addr);
    }
}

struct AddressBar {
    value: String,
    addr: PathBuf,
}

impl AddressBar {
    fn view(&self) -> Element<Event> {
        text_input("Directory Location", &self.value, |s| {
            Event::Browser(BrowserEvent::AddrChanged(s))
        })
        .on_submit(Event::Browser(BrowserEvent::AddrSubmit))
        .size(16)
        .padding(5)
        .style(theme::AddressBarStyle)
        .into()
    }
}

impl Default for AddressBar {
    fn default() -> Self {
        let home = dirs::home_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        AddressBar {
            addr: PathBuf::from(&home),
            value: home,
        }
    }
}

struct Contents {
    entries: Vec<Content>,
}

impl Default for Contents {
    fn default() -> Self {
        Contents {
            entries: Self::get_contents(dirs::home_dir().unwrap_or_default().as_path()),
        }
    }
}

impl Contents {
    fn view(&self) -> Element<Event> {
        let col: Column<'_, Event> = column()
            .push(
                button(text(" ..").size(18))
                    .on_press(Event::Browser(BrowserEvent::DirUp))
                    .width(Length::FillPortion(75))
                    .style(theme::ContentButtonStyle::new(false))
                    .padding(4),
            )
            .spacing(10)
            .width(Length::FillPortion(75));

        container(
            scrollable(
                container(self.entries.iter().fold(col, |c, f| c.push(f.view()))).padding(20),
            )
            .style(theme::ScrollableStyle),
        )
        .padding(4)
        .style(InnerContainerStyle)
        .width(Length::FillPortion(75))
        .height(Length::Fill)
        .into()
    }

    fn clear_selection(&mut self) {
        self.entries
            .iter_mut()
            .for_each(|e| e.selected = false);
    }

    fn get_contents<P: AsRef<Path>>(dir: P) -> Vec<Content> {
        let fnd: (Vec<DirEntry>, Vec<DirEntry>) = (Vec::new(), Vec::new());
        let (mut files, mut dirs) = fs::read_dir(dir)
            .unwrap()
            .filter_map(|r| if let Ok(p) = r { Some(p) } else { None })
            .fold(fnd,|mut fnd, f| { if f.path().is_dir() { fnd.1.push(f) } else { fnd.0.push(f) }; fnd } );

        dirs.sort_by_key(|e| e.file_name());
        files.sort_by_key(|e| e.file_name());

        dirs
            .into_iter()
            .chain(files.into_iter())
            .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
            .enumerate()
            .map(|(i,e)| Content::new(e,i))
            .collect()
    }
}

const FOLDER_ICON_SRC: &[u8] = include_bytes!("../icons/newfolder.svg");
const IMAGE_ICON_SRC: &[u8] = include_bytes!("../icons/image.svg");
const FILE_ICON_SRC: &[u8] = include_bytes!("../icons/file.svg");

struct Content {
    handle: DirEntry,
    ctype: ContentType,
    id: usize,
    selected: bool,
}

impl Content {
    fn new(file: DirEntry, id: usize) -> Self {
        if file.path().is_dir() {
            Content {
                handle: file,
                ctype: ContentType::Directory,
                id,
                selected: false,
            }
        } else if file.file_name().to_string_lossy().is_image() {
            Content {
                handle: file,
                ctype: ContentType::Image,
                id,
                selected: false,
            }
        } else {
            Content {
                handle: file,
                ctype: ContentType::Generic,
                id,
                selected: false,
            }
        }
    }

    fn view(&self) -> Element<Event> {
        use ContentType::*;
        let btcontent = text(self.handle.file_name().to_string_lossy()).size(16).width(Length::FillPortion(1));

        let icon = {
            let src = match self.ctype {
                Directory => &FOLDER_ICON_SRC,
                Image => &IMAGE_ICON_SRC,
                Generic => &FILE_ICON_SRC,
            };
            container(
                svg::Svg::new(svg::Handle::from_memory(*src))
                    .content_fit(iced::ContentFit::Contain)
                    .width(Length::Units(22))
                    .height(Length::Units(20))
            )
                .padding(2)
                .center_x()
                .center_y()
        };
        let button = match self.ctype {
            Directory | ContentType::Image => button(btcontent)
                .on_press(Event::Browser(BrowserEvent::ContentClicked(self.id)))
                .style(theme::ContentButtonStyle::new(self.selected))
                .width(Length::Fill),
            Generic => button(btcontent)
                .style(theme::ContentButtonStyle::new(false))
                .width(Length::Fill),
        };

        container(row().push(icon).push(button).spacing(6))
            .center_y()
            .into()
    }
}

enum ContentType {
    Image,
    Directory,
    Generic,
}
