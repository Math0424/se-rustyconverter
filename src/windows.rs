use std::default;
use std::env::set_var;
use iced::{alignment, command, executor, window, Rectangle, Sandbox, Size};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_space, row, scrollable, slider, text, text_editor, text_input, toggler, vertical_space, Column, Image
};
use iced::{Command, Element, Settings, Theme, Length};
use native_dialog::{FileDialog, MessageDialog, MessageType};

use crate::window_options::WindowType;

#[derive(Debug, Clone)]
pub enum WindowMessage {
    FileSelected(String),
}

impl<'a> WindowType {
    pub fn view(&self) -> Element<WindowMessage> {
        match self {
            WindowType::LCDConverter { .. } => Self::lcd_converter_view(self),
            WindowType::BlueprintConverter => Self::creator("Blueprint").into(),
            WindowType::LCDGifConverter => Self::creator("GIF").into(),
            WindowType::SpraysModConverter => Self::creator("SpraysMod").into(),
            WindowType::DDSConverter => Self::creator("DDS").into(),
        }.into()
    }

    pub fn title(&self) -> &str {
        match self {
            WindowType::LCDConverter { .. } => "LCDConverter",
            WindowType::BlueprintConverter => "BlueprintConverter",
            WindowType::LCDGifConverter => "LCDGifConverter",
            WindowType::SpraysModConverter => "SpraysModConverter",
            WindowType::DDSConverter => "DDSConverter",
        }.into()
    }

    pub fn update(&mut self, msg: WindowMessage) {
        match msg {
            WindowMessage::FileSelected(value) => {
                if let WindowType::LCDConverter { ref mut selected_file, .. } = self {
                    *selected_file = value.into();
                }
            },
        }
    }

    fn creator(title: &str) -> Column<'_, WindowMessage> {
        column![text(title).size(50)].spacing(20)
    }
    
    fn lcd_converter_view(&self) -> Element<'a, WindowMessage> {
        if let WindowType::LCDConverter { selected_file, .. } = self {
            let image = Image::new(selected_file.clone().unwrap_or("none".into()).to_string()).width(Length::Fill).height(Length::Fill);
            
            let img_container = container(image).width(Length::FillPortion(2)).height(Length::FillPortion(2));

            return container(img_container).width(Length::Fill).height(Length::Fill)
            .into();
        }
        panic!()
    }
}
