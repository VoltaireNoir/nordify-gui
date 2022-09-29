use iced::Length;
use iced::pure::{Element,container,column,row,text,text_input,scrollable};
use super::Event;
use super::theme::{JUST_GREY,BtmContainerStyle};

#[derive(Default)]
pub struct Browser {
    addrbar: AddressBar,
}

#[derive(Clone, Debug)]
pub enum BrowserEvent {
    AddrSubmit,
    AddrChanged(String),
}

impl Browser {
    pub fn view(&self) -> Element<'_,Event> {
        container(
            column()
                .push(
                    row()
                        .push(text("BROWSE").color(JUST_GREY).size(16))
                        .push(self.addrbar.view())
                        .spacing(5)
                        .width(Length::FillPortion(75))
                )
                .push(
                    scrollable(
                        text("Files")
                            .width(Length::FillPortion(75))
                            )
                )
                .width(Length::FillPortion(75))
                .padding(5)
        )
            .width(Length::FillPortion(75))
            .height(Length::FillPortion(50))
            .style(BtmContainerStyle)
            .into()
    }

    pub fn update(&mut self, message: BrowserEvent) {
        match message {
            BrowserEvent::AddrChanged(v) => {self.addrbar.value = v},
            _ => (),
        }
    }
}

#[derive(Default)]
struct AddressBar {
    value: String,
}

impl AddressBar {
    fn view(&self) -> Element<Event> {
        text_input("Directory Location", &self.value, |s| Event::Browser(BrowserEvent::AddrChanged(s)))
            .on_submit(Event::Browser(BrowserEvent::AddrSubmit))
            .into()
    }
}
