use std::default;
use std::env::set_var;
use std::ops::BitAnd;
use iced::futures::io::empty;
use iced::widget::shader::wgpu::naga::proc::Alignment;
use iced::{alignment, command, executor, window, Rectangle, Sandbox, Size};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_space, row, scrollable, slider, text, text_editor, text_input, toggler, vertical_space, Column, Image, Space
};
use iced::{Command, Element, Settings, Theme, Length};
use native_dialog::{FileDialog, MessageDialog, MessageType};

use crate::window_options::{BitMode, DitherMode, InterpolationMode, LCDSize, LCDWindowData, WindowType};

#[derive(Debug, Clone)]
pub enum WindowMessage {
    FileSelected(String),
    DitherSelected(DitherMode),
    BitSelected(BitMode),
    InterpolationSelected(InterpolationMode),
    LCDSelected(LCDSize),
    SetLCDSize(usize, usize),
    TrySetXLCDSize(String),
    TrySetYLCDSize(String),
}

impl<'a> WindowType {
    pub fn view(&self) -> Element<WindowMessage> {
        match self {
            WindowType::LCDConverter(data) => Self::lcd_converter_view(data),
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
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.selected_file = value.into();
                }
            },
            WindowMessage::DitherSelected(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.dither = value.into();
                }  
            },
            WindowMessage::BitSelected(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.bit_mode = value.into();
                }  
            },
            WindowMessage::InterpolationSelected(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.interpolation = value.into();
                }
            },
            WindowMessage::LCDSelected(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.size_x = value.1;
                    data.size_y = value.2;
                    data.selected_lcd = value.into();
                }
            },
            WindowMessage::SetLCDSize(x, y) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.size_x = x;
                    data.size_y = y;
                }
            },
            WindowMessage::TrySetXLCDSize(value) => {

            },
            WindowMessage::TrySetYLCDSize(value) => {

            },
        }
    }

    fn creator(title: &str) -> Column<'_, WindowMessage> {
        column![text(title).size(50)].spacing(20)
    }
    
    fn lcd_converter_view(data: &'a LCDWindowData) -> Element<'a, WindowMessage> {
        let image = Image::new(data.selected_file.clone().unwrap_or("none".into()).to_string()).width(Length::Fill).height(Length::Fill);
        let img_container = container(image).padding(10);

        let dither_combo = combo_box(
            &data.dither_options,
            "Select Option", 
            data.dither.as_ref(), 
            WindowMessage::DitherSelected);
        
        let interpolation_combo = combo_box(
            &data.interpolation_options,
            "Select Option", 
            data.interpolation.as_ref(), 
            WindowMessage::InterpolationSelected);

        let image_options = column![text("DitherMode"), dither_combo, text("InterpolationMode"), interpolation_combo].spacing(1);

        let bitmode_combo = combo_box(
            &data.bit_options,
            "Select Option", 
            data.bit_mode.as_ref(), 
            WindowMessage::BitSelected);
        let lcdtype_combo = combo_box(
            &data.lcd_options,
            "Select Option", 
            data.selected_lcd.as_ref(), 
            WindowMessage::LCDSelected);

        let x_inp = text_input("x", &data.size_x.to_string());
        let y_inp = text_input("y", &data.size_x.to_string());
    
        let lcd_options = row!(
            lcdtype_combo, 
            row![x_inp, text("x").horizontal_alignment(alignment::Horizontal::Center), y_inp].spacing(2)
        ).spacing(5);
        
        let other_options = column![text("BitMode"), bitmode_combo, text("LCDSize"), lcd_options].spacing(1);
        let convert_btn = button("Convert");
        let bottom_options = row![image_options, other_options, convert_btn].spacing(5);

        return container(column![img_container, bottom_options.align_items(iced::Alignment::End)])
        .width(Length::Fill).height(Length::Fill)
        .into();
    }
}
