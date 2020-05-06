#![recursion_limit = "1024"]

use qr_image_core::{EcLevel, Rgb, Version};
use std::str::FromStr;
use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

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
}

#[derive(Debug)]
pub struct Model {
    link: ComponentLink<Self>,
    input: String,
    output_size: u32,
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
            input: String::new(),
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
            Event::OutputSize(data) => match data {
                ChangeData::Value(v) => match u32::from_str(&v) {
                    Ok(o) => self.output_size = o,
                    Err(_) => (),
                },
                _ => (),
            },
            Event::QRVersion(data) => match data {
                ChangeData::Value(v) => match i16::from_str(&v) {
                    Ok(o) => self.qr_version = Version::Normal(o),
                    Err(_) => (),
                },
                _ => (),
            },
            Event::ECLevel(_) => unimplemented!(),
            Event::DarkColor(_) => unimplemented!(),
            Event::LightColor(_) => unimplemented!(),
            Event::EnhanceMode(_) => unimplemented!(),
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
            </div>
            {self.form_view()}
        </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
