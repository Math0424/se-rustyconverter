use std::default;
use std::env::set_var;
use iced::{alignment, command, executor, window, Rectangle, Sandbox, Size};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_space, row, scrollable, slider, text, text_editor, text_input, toggler, vertical_space
};
use iced::{Command, Element, Settings, Theme, Length};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use window_options::WindowType;

mod window_options;

pub fn main() -> iced::Result {
    set_var("DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1", "1");
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
    SelectWindow(WindowType),
}

impl Sandbox for SEImageConverter {
    type Message = Message;

    fn new() -> SEImageConverter {
        SEImageConverter {
            window_options: combo_box::State::new(WindowType::ALL.to_vec()),
            window_selected: Some(WindowType::LCDConverter),
            file_text: "Select File".into(),
        }
    }

    fn title(&self) -> String {
        String::from("SE-ImageConverter 2.0")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::None(_value) => (),
            Message::SelectWindow(value) => {
                self.window_selected = Some(value);
            },
            Message::SelectFile => {
                let path = FileDialog::new()
                    .set_location("~/Downloads")
                    .add_filter("Image files", &["png", "jpg", "jpeg", "webp", "bmp"])
                    .show_open_single_file()
                    .unwrap();
            
                let path = match path {
                    Some(path) => path,
                    None => return,
                };
                
                if let Some(x) = path.as_path().to_str() {
                    self.file_text = x.into();
                }

                return
            },
        }

    }

    fn view(&self) -> Element<Self::Message> {
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



        container(col).width(Length::Fill).padding(10).into()
    }

    fn theme(&self) -> iced::Theme {
		iced::Theme::Dark
	}
}