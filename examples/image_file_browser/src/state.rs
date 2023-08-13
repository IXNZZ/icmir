use std::fs::File;
use std::io::Read;
use std::path::Path;
use iced::{Alignment, Element, Length, Renderer, Sandbox, theme};
use iced::widget::{button, Button, column, Column, row, Row, scrollable, Text};
use tracing::info;


pub struct State{
    index: usize,
    dir: String,
    files: Vec<String>,
    images: Vec<u32>,
    page: u32,
    page_size: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
    SelectIndex(usize),
    PagePrev(u32),
    PageNext(u32),
    OpenFile,
}

impl State {
    fn load_images(&mut self) {
        if self.index == 0 { return; }
        let idx = self.index - 1;
        let name = self.files.get(idx).unwrap().as_str();
        let name = &name[..name.len() - 4];
        let path = self.dir.to_string() + "/" + name + ".wzl";
        let start_idx = self.page * self.page_size;
        let end_idx = if start_idx + self.page_size + 1 > self.images.len() as u32 {
            self.images.len() as u32
        } else { start_idx + self.page_size + 1 };
        if start_idx >= end_idx { return; }

        let x = &self.images[start_idx as usize..end_idx as usize];
        for i in 0..x.len() - 1 {
            file::data::load_image(path.as_str(), x[i], x[i+1]);
        }
    }
}

impl Sandbox for State {
    type Message = AppMessage;

    fn new() -> Self {
        let mut files: Vec<String> = Path::new("/Users/vt/Documents/LegendOfMir/data").read_dir().unwrap()
            .map(|f| { String::from(f.unwrap().file_name().to_str().unwrap()) })
            .filter(|f| { f.ends_with(".idx") }).collect();
        files.sort();
        files.remove(0);
        info!("init: {}", files.len());
        Self { index: 0, dir: "/Users/vt/Documents/LegendOfMir/data".to_string(), files, images: Vec::new(), page: 0, page_size: 50 }
    }

    fn title(&self) -> String {
        "Image File Browser".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::SelectIndex(idx) => {
                self.index = idx;
                let path = self.dir.to_string() + "/" + self.files.get(idx - 1).unwrap();
                self.images = file::data::load_index(path.as_str());

                self.load_images();
            }
            _ => {

            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {

        let menu = row![
            button("选择目录").on_press(AppMessage::OpenFile).height(30),
            button("上一页").on_press(AppMessage::PagePrev(1)).height(30),
            button("下一页").on_press(AppMessage::PageNext(1)).height(30),
        ].width(Length::Fill)
            .align_items(Alignment::Center)
            .padding([5, 0, 5, 10])
            .spacing(30);

        let x = column(self.files.iter().enumerate()
            .map(|(x, r)| {

                button(r.as_str())
                    .width(200)
                    .style(if x + 1 == self.index {theme::Button::Primary} else { theme::Button::Text })
                    .on_press(AppMessage::SelectIndex(x + 1)).into()})
            .collect::<Vec<_>>());




        column![menu, scrollable(x)].into()
    }


}