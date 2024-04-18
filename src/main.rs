use std::env::set_var;
use iced::{command, executor, window, Sandbox, Size};
use iced::widget::{
    button, checkbox, column, container, horizontal_space, row, scrollable, slider, text, text_editor, text_input, toggler, vertical_space
};
use iced::{Application, Command, Element, Settings, Theme, Length};
use native_dialog::{FileDialog, MessageDialog, MessageType};

pub fn main() -> iced::Result {
    set_var("DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1", "1");
    SEImageConverter::run(Settings {
        window: window::Settings {
            size: Size::new(500., 600.),
            resizable: false,
            exit_on_close_request: true,
            ..window::Settings::default()
        },
        ..Default::default()
    })
}

struct SEImageConverter {
    file_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    SelectFile,
}

impl Application for SEImageConverter {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                file_text: "Select File".into(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("SE-ImageConverter 2.0")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectFile => {
                let path = FileDialog::new()
                    .set_location("~/Downloads")
                    .add_filter("Image files", &["png", "jpg", "jpeg", "webp", "bmp"])
                    .show_open_single_file()
                    .unwrap();
            
                let path = match path {
                    Some(path) => path,
                    None => return Command::none(),
                };
                
                if let Some(x) = path.as_path().to_str() {
                    let start_index = x.len().saturating_sub(80);
                    let trimmed_str = &x[start_index..];
                    self.file_text = trimmed_str.into();
                }

                return Command::none()
            },
        }

    }

    fn view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        let file_selector = text(&self.file_text);
        let button = button("File").on_press(Message::SelectFile);
        let file_bar = row![file_selector, horizontal_space(), button];
        
        container(file_bar).padding(10).into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}