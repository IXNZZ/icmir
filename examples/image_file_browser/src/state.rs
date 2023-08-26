use std::fs::File;
use std::io::Read;
use std::path::Path;
use file::data::ImageData;
use iced::{Alignment, Element, Length, Renderer, Sandbox, theme, widget};
use iced::futures::{FutureExt, StreamExt};
use iced::widget::{button, Button, column, text, container, image, row, Row, scrollable, Text};
use rfd::FileDialog;
use tracing::{debug, info};

type Image = iced::widget::Image;

pub struct State{
    index: usize,
    dir: String,
    files: Vec<String>,
    image_idx: Vec<u32>,
    page: u32,
    page_size: u32,
    images: Vec<ImageData>
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
        // let name = &name[..name.len() - 4];
        let path = self.dir.to_string() + "/" + name + ".wzl";
        let start_idx = self.page * self.page_size;
        let end_idx = if start_idx + self.page_size + 1 > self.image_idx.len() as u32 {
            self.image_idx.len() as u32
        } else { start_idx + self.page_size + 1 };
        if start_idx >= end_idx { return; }

        let x = &self.image_idx[start_idx as usize..end_idx as usize];
        let mut images = Vec::with_capacity(x.len() - 1);
        for i in 0..x.len() - 1 {
            debug!("idx: {}, {}, {}", i, x[i], x[i+1]);
            let img = file::data::load_image(path.as_str(), x[i], x[i+1]);
            images.push(img);
        }
        self.images = images;
    }
}

impl Sandbox for State {
    type Message = AppMessage;

    fn new() -> Self {

        Self { index: 0, dir: "".to_string(), files: Vec::new(), image_idx: Vec::new(), page: 0, page_size: 50, images: Vec::new() }
    }

    fn title(&self) -> String {
        "Image File Browser".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::SelectIndex(idx) => {
                self.page = 0;
                self.index = idx;
                let path = self.dir.to_string() + "/" + self.files.get(idx - 1).unwrap() + ".idx";
                self.image_idx = file::data::load_index(path.as_str());

                self.load_images();
            },
            AppMessage::PagePrev(p) => {
                if self.page > 0 {
                    self.page -= p;
                    self.load_images();
                }
            },
            AppMessage::PageNext(p) => {
                if (self.page + 1) * self.page_size < self.image_idx.len() as u32 {
                    self.page += p;
                    self.load_images();
                }
            },
            AppMessage::OpenFile => {
                if let Some(dir) = FileDialog::new().set_directory("~/").pick_folder() {
                    let mut files: Vec<String> = dir.read_dir().unwrap()
                        .map(|f| { f.unwrap().file_name().to_str().unwrap().to_lowercase() })
                        .filter(|f| { f.ends_with(".idx") })
                        .map(|x| {x.split_at(x.len() -4).0.to_lowercase()})
                        .collect();
                    files.sort();
                    info!("init: {}, dir: {:?}", files.len(), dir);
                    self.dir = dir.to_str().unwrap().to_string();
                    self.files = files;
                }
            },
            _ => {

            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {

        let menu = row![
            button("选择目录").on_press(AppMessage::OpenFile).height(30),
            button("上一页").on_press(AppMessage::PagePrev(1)).height(30),
            button("下一页").on_press(AppMessage::PageNext(1)).height(30),
            text(format!("数量: {}, 当前: {}-{}", self.image_idx.len(), self.page * self.page_size, self.page * self.page_size + self.page_size))
        ].width(Length::Fill)
            .align_items(Alignment::Center)
            .padding([5, 0, 5, 10])
            .spacing(20);

        let x = column(self.files.iter().enumerate()
            .map(|(x, r)| {

                button(r.as_str())
                    .width(200)
                    .style(if x + 1 == self.index {theme::Button::Primary} else { theme::Button::Text })
                    .on_press(AppMessage::SelectIndex(x + 1)).into()})
            .collect::<Vec<_>>());
        // let chunks = self.images.iter().enumerate().map(|(i, h)| {
        //     Image::new(h.clone()).width(48).height(32).into()
        // }).chunks(10);
        let mut idx = self.page * self.page_size;
        let content = column(self.images.chunks(10).map(|x| {
            row::<Self::Message, Renderer>(x.iter().enumerate().map(|(i, h)| {
                let t1 = format!("{:05}", idx);
                let new_width = if h.bytes.len() != 0 { h.bytes.len() as u32 / 4 / h.height as u32 } else { h.width as u32 };
                let t2 = format!("{}X{}", h.width, h.height);
                idx += 1;
                let handle = if h.bytes.len() == 0 {
                    image::Handle::from_pixels(1, 1, [0, 0, 0, 0])
                } else {
                    image::Handle::from_pixels(h.width as u32, h.height as u32, h.bytes.clone())
                };
                column![
                    Image::new(handle).width(96).height(64), widget::text(t1), widget::text(t2)
                ].into()
                // Image::new(h.clone()).width(96).height(64).into()
            }).collect::<Vec<_>>()).into()
        }).collect::<Vec<_>>());

        let center = row![scrollable(x), scrollable(content)];

        column![menu, center].into()
    }
}