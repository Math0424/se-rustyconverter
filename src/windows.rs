use std::env::set_var;
use std::path::{Path, PathBuf};
use iced::{alignment, Command};
use iced::widget::{
    button, column, combo_box, container, image, row, text, text_input, Column
};
use iced::{Element, Length};

use crate::window_options::{BitMode, DitherMode, InterpolationMode, LCDSize, LCDWindowData, WindowType};
use crate::{image_processor, Message};

#[derive(Debug, Clone)]
pub enum WindowMessage {
    FileSelected(PathBuf),
    DitherSelected(DitherMode),
    BitSelected(BitMode),
    InterpolationSelected(InterpolationMode),
    LCDSelected(LCDSize),
    TrySetXLCDSize(String),
    TrySetYLCDSize(String),
}

impl<'a> WindowType {
    pub fn view(&self) -> Element<WindowMessage> {
        match self {
            WindowType::LCDConverter(data) => Self::lcd_converter_view(data),
            WindowType::BlueprintConverter => Self::temp("Blueprint").into(),
            WindowType::LCDGifConverter => Self::temp("GIF").into(),
            WindowType::SpraysModConverter => Self::temp("SpraysMod").into(),
            WindowType::DDSConverter => Self::temp("DDS").into(),
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

    pub fn update(&mut self, msg: WindowMessage) -> Command<Message>  {
        match msg {
            WindowMessage::FileSelected(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.selected_file = Some(value.as_os_str().to_str().unwrap().to_string());
                    let path = value.as_os_str().to_str().unwrap().to_string();
                    let processed_image_result = crate::image_processor::process_image(path, data.interpolation.as_ref().unwrap().clone(), data.size_x, data.size_y);
                
                    match processed_image_result {
                        Ok(bytes) => {
                            data.image_handle = Some(image::Handle::from_memory(bytes));
                        },
                        Err(e) => {
                            eprintln!("Error processing image: {:?}", e);
                        }
                    }
                }
                Command::none()
            },
            WindowMessage::DitherSelected(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.dither = value.into();
                }  
                Command::none()
            },
            WindowMessage::BitSelected(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.bit_mode = value.into();
                }  
                Command::none()
            },
            WindowMessage::InterpolationSelected(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.interpolation = value.into();
                }
                Command::none()
            },
            WindowMessage::LCDSelected(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    data.size_x = value.1;
                    data.size_y = value.2;
                    data.selected_lcd = value.into();
                }
                Command::none()
            },
            WindowMessage::TrySetXLCDSize(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    if let Ok(number) = value.parse::<usize>() {
                        data.size_x = number;
                    } 
                }
                Command::none()
            },
            WindowMessage::TrySetYLCDSize(value) => {
                if let WindowType::LCDConverter(ref mut data) = self {
                    if let Ok(number) = value.parse::<usize>() {
                        data.size_y = number;
                    } 
                }
                Command::none()
            },
        }
    }

    fn temp(title: &str) -> Column<'_, WindowMessage> {
        column![text(title).size(50)].spacing(20)
    }
    
    fn lcd_converter_view(data: &'a LCDWindowData) -> Element<'a, WindowMessage> {
        let preview_img = iced::widget::Image::new(data.image_handle.clone().unwrap_or("none".into()))
        .content_fit(iced::ContentFit::Contain);

        let img_container = container(preview_img)
        .width(Length::Fill).height(Length::Fill).padding(10);

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

        let mut x_inp = text_input("x", &data.size_x.to_string());
        let mut y_inp = text_input("y", &data.size_y.to_string());
    
        if let Some(lcd) = &data.selected_lcd {
            if lcd.0 == "Custom" {
                x_inp = x_inp.on_input(WindowMessage::TrySetXLCDSize);
                y_inp = y_inp.on_input(WindowMessage::TrySetYLCDSize);
            }
        }

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
