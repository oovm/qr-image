#![recursion_limit = "1024"]

use colors_transform::Color;
use qr_image_core::{EcLevel, Rgb, Version};
use std::str::FromStr;
use yew::{
    html,
    prelude::*,
    services::{reader::FileData, ConsoleService, Task},
    Component, ComponentLink, Html, ShouldRender,
};
use yew::services::reader::ReaderService;

mod form;

pub fn header_view() -> Html {
    let title = "KaTeX for Yew";
    html! {
    <header>
        <h1 color="#009688">{title}</h1>
        <a href="https://github.com/GalAster/yew-katex">{"Fork me!"}</a>
    </header>
    }
}

pub enum Event {
    Input(String),
    OutputSize(ChangeData),
    QRVersion(ChangeData),
    ECLevel(ChangeData),
    DarkColor(ChangeData),
    LightColor(ChangeData),
    EnhanceMode(ChangeData),
    Files(ChangeData),
    Loaded(FileData),
}

#[derive(Debug)]
pub struct Model {
    link: ComponentLink<Self>,
    input: String,
    image: Vec<u8>,
    output_size: usize,
    qr_version: Version,
    ec_level: EcLevel,
    dark_color: Rgb<u8>,
    light_color: Rgb<u8>,
    enhanced: bool,
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::from("https://galaster.github.io/qr-image"),
            image: include_bytes!("github.png").to_vec(),
            output_size: 400,
            qr_version: Version::Normal(3),
            ec_level: EcLevel::L,
            dark_color: Rgb([0, 0, 0]),
            light_color: Rgb([255, 255, 255]),
            enhanced: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(s) => {
                self.input = s;
            }
            Event::OutputSize(data) => {
                if let ChangeData::Value(v) = data {
                    if let Ok(o) = usize::from_str(&v) {
                        self.output_size = o
                    }
                }
            }
            Event::QRVersion(data) => {
                if let ChangeData::Value(v) = data {
                    if let Ok(o) = i16::from_str(&v) {
                        self.qr_version = Version::Normal(o)
                    }
                }
            }
            Event::ECLevel(data) => {
                if let ChangeData::Value(v) = data {
                    self.input = v
                }
            }
            Event::DarkColor(data) => {
                if let ChangeData::Value(v) = data {
                    if let Ok(color) = colors_transform::Rgb::from_hex_str(&v) {
                        self.dark_color = Rgb([color.get_red() as u8, color.get_green() as u8, color.get_blue() as u8])
                    }
                }
            }
            Event::LightColor(data) => {
                if let ChangeData::Value(v) = data {
                    if let Ok(color) = colors_transform::Rgb::from_hex_str(&v) {
                        self.light_color = Rgb([color.get_red() as u8, color.get_green() as u8, color.get_blue() as u8])
                    }
                }
            }
            Event::EnhanceMode(_) => self.enhanced = !self.enhanced,
            Event::Files(data) =>
            {
                if let ChangeData::Files(f) = data {
                    ReaderService::new().read_file(f.get(0).unwrap(), self.link.callback(Event::Loaded))
                        .unwrap()
                        .is_active();
                }
            }
            Event::Loaded(data) => {
                ConsoleService::log(&format!("{:?}", data));
                self.image = data.content
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <main class="container-fluid">
            <div class="page-header">
                <h1>{"QR Image Embed"}</h1>
                <iframe
                    src="https://ghbtns.com/github-btn.html?user=GalAster&repo=qr-image&type=star&count=true&size=large"
                    frameborder="0" scrolling="0" width="170" height="30" title="GitHub"
                />
            </div>
            {self.form_view()}
        </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
