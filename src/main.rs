#![windows_subsystem = "windows"]

use std::sync::Arc;
use std::{default, io};
use std::env::set_var;
use iced::application::StyleSheet;
use iced::{alignment, command, executor, window, Application, Rectangle, Sandbox, Size};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_space, row, scrollable, slider, text, text_editor, text_input, toggler, vertical_space
};
use iced::{Command, Element, Settings, Theme, Length};
use window_options::WindowType;
use windows::WindowMessage;

mod window_options;
mod windows;

pub fn main() -> iced::Result {
    //set_var("DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1", "1");
    SEImageConverter::run(Settings {
        window: window::Settings {
            size: Size::new(500., 600.),
            resizable: false,
            ..window::Settings::default()
        },
        ..Default::default()
    })
}

struct SEImageConverter {
    window_options: combo_box::State<WindowType>,
    file_text: String,
    window_selected: Option<WindowType>,
}

#[derive(Debug, Clone)]
pub enum Message {
    None(String),
    SelectFile,
    OpenFile(Result<Arc<String>, Error>),
    SelectWindow(WindowType),
    WindowMessage(WindowMessage),
}

impl Application for SEImageConverter {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (SEImageConverter, Command<Message>) {
        (
            Self {
                window_options: combo_box::State::new(WindowType::all().to_vec()),
                window_selected: Some(WindowType::default()),
                file_text: "Select File".into(),
            }, 
            Command::none()
        )
    }

    fn title(&self) -> String {
        format!("SE-ImageConverter 2.0 - {}", self.window_selected.as_ref().unwrap().title())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::None(_value) => Command::none(),
            Message::SelectWindow(value) => {
                self.window_selected = Some(value);
                Command::none()
            },
            Message::SelectFile => Command::perform(pick_file(), Message::OpenFile),

            Message::OpenFile(Ok(value)) => {
                let val = value.to_string();
                self.file_text = val.clone();
                self.window_selected.as_mut().unwrap().update(WindowMessage::FileSelected(val));
                Command::none()
            },
            Message::OpenFile(Err(value)) => {
                if let Error::IO(value) = value {
                    self.file_text = value;
                }
                Command::none()
            },

            Message::WindowMessage(value) => {
                self.window_selected.as_mut().unwrap().update(value);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let combo = combo_box(
            &self.window_options, 
            "Select Option", 
            self.window_selected.as_ref(), 
            Message::SelectWindow);

        let file_selector = text_input("Select File", &self.file_text)
        .width(Length::Fill)
        .on_input(Message::None);
        
        let button = button("Open File").on_press(Message::SelectFile);
        let file_bar = row![file_selector, button].spacing(10);
        
        let col = column![column![text("Select converter"), combo], file_bar].spacing(25);

        if let Some(x) = &self.window_selected {
            let component_view = x.view().map(Message::WindowMessage);
            container(column!(col, component_view)).width(Length::Fill).padding(10).into()
        } else {
            container(col).width(Length::Fill).padding(10).into()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn pick_file() -> Result<Arc<String>, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Pick an image")
        .add_filter("Image Files", &["png", "jpg", "jpeg", "webp", "bmp"])
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?; 

    let path_str = handle.path().to_str().ok_or_else(|| Error::IO("Failed to convert path to string".into()))?.to_owned();
    Ok(Arc::new(path_str))
}

#[derive(Debug, Clone)]
enum Error {
    IO(String),
    DialogClosed
}