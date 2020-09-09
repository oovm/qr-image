use crate::{Event, Model};
use image::{imageops::FilterType, GenericImageView, ImageFormat, ImageOutputFormat};
use qr_image_core::{EcLevel, QrImage, Version};
use yew::prelude::*;

impl Model {
    pub fn format_qr_version(&self) -> String {
        let n = match self.qr_version {
            Version::Normal(i) => i,
            Version::Micro(i) => i,
        };
        return format!("{}", n);
    }

    pub fn format_ec_level(&self) -> String {
        let n = match self.ec_level {
            EcLevel::L => "L",
            EcLevel::M => "M",
            EcLevel::Q => "Q",
            EcLevel::H => "H",
        };
        return String::from(n);
    }

    pub fn qr_render(&self) -> anyhow::Result<(String, u32)> {
        let renderer = QrImage {
            qr_version: self.qr_version,
            ec_level: self.ec_level,
            dark_color: self.dark_color,
            light_color: self.light_color,
            enhanced: self.enhanced,
            auto_size: true,
        };

        let input = image::load_from_memory_with_format(&self.image, ImageFormat::Png)?;
        let mut base_img = renderer.render(self.input.as_bytes(), &input)?;
        if base_img.width() < self.output_size as u32 {
            base_img = base_img.resize_exact(self.output_size as u32, self.output_size as u32, FilterType::Nearest)
        }
        let mut buf = vec![];
        base_img.write_to(&mut buf, ImageOutputFormat::Png)?;
        return Ok((base64::encode(&buf), base_img.width().max(self.output_size as u32)));
    }

    pub fn qr_code_view(&self) -> Html {
        let qr = match self.qr_render() {
            Ok((o, size)) => {
                html! {
                    <img width=size height=size
                        src=format!("data:image/png;base64,{}",o)
                    />
                }
            }
            Err(e) => {
                html! {
                    <label style="color:#FF0000">{format!("{:?}", e)}</label>
                }
            }
        };
        return html! {
        <div class="form-group">
            <label class="col-sm-2">{"QR_CODE:"}</label>
            <div class="col-sm-10">{qr}</div>
        </div>
        };
    }

    pub fn form_view(&self) -> Html {
        html! {
        <form class="form-horizontal">
            {self.qr_code_view()}
            <div class="form-group">
                <label class="col-sm-2">{"Text:"}</label>
                <div class="col-sm-10">
                    <textarea class="form-control" rows="3"
                        value=self.input
                        oninput=self.link.callback(|input: InputData| Event::Input(input.value))
                    />
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Image:"}</label>
                <div class="col-sm-10">
                    <input type="file" multiple=true
                        onchange=self.link.callback(|input: ChangeData| Event::Files(input))
                    />
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"QR Version:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="range" min="1" max="10" step="1"
                            value=self.format_qr_version()
                            onchange=self.link.callback(|input: ChangeData| Event::QRVersion(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1">{self.format_qr_version()}</label>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Output Size:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="range" min="80" max="640" step="20"
                            value=self.output_size
                            onchange=self.link.callback(|input: ChangeData| Event::OutputSize(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1">{self.output_size}</label>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"EC Level:"}</label>
                <div class="col-sm-10">
                    <select class="form-control"
                        value=self.format_ec_level()
                        onchange=self.link.callback(|input: ChangeData| Event::ECLevel(input))
                    >
                        <option value="L">{"L"}</option>
                        <option value="Q">{"Q"}</option>
                        <option value="M">{"M"}</option>
                        <option value="H">{"H"}</option>
                    </select>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Enhanced:"}</label>
                <div class="col-sm-10">
                    <input type="checkbox"
                        checked=self.enhanced
                        onchange=self.link.callback(|input: ChangeData| Event::EnhanceMode(input))
                    />
                 </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Background:"}</label>
                <div class="col-sm-10">
                    <div class="form-control-static">
                        <input type="color"
                            onchange=self.link.callback(|input: ChangeData| Event::LightColor(input))
                        />
                    </div>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Foreground:"}</label>
                <div class="col-sm-10">
                    <div class="form-control-static">
                        <input type="color"
                            onchange=self.link.callback(|input: ChangeData| Event::DarkColor(input))
                        />
                    </div>
                </div>
            </div>
        </form>
        }
    }
}
