use crate::{Event, Model};

use qr_image_core::Version;
use yew::prelude::*;

impl Model {
    pub fn qr_version_view(&self) -> String {
        let n = match self.qr_version {
            Version::Normal(i) => i,
            Version::Micro(i) => i,
        };
        return format!("{}", n);
    }

    pub fn form_view(&self) -> Html {
        html! {
        <form class="form-horizontal">
            <div class="form-group">
                <label class="col-sm-3 control-label">{"Value:"}</label>
                <div class="col-sm-9">
                    <textarea v-model="value" class="form-control" rows="3"></textarea>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-3 control-label">{"QR Version:"}</label>
                <div class="col-sm-8">
                    <div class="form-control-static">
                        <input type="range" min="1" max="40" step="1"
                            value=self.qr_version_view()
                            onchange=self.link.callback(|input: ChangeData| Event::QRVersion(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1 control-label">{self.qr_version_view()}</label>
            </div>
            <div class="form-group">
                <label class="col-sm-3 control-label">{"Output Size:"}</label>
                <div class="col-sm-8">
                    <div class="form-control-static">
                        <input type="range" min="100" max="800" step="20"
                            value=self.output_size
                            onchange=self.link.callback(|input: ChangeData| Event::OutputSize(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1 control-label">{self.output_size}</label>
            </div>
            <div class="form-group">
                <label class="col-sm-3 control-label">{"Level:"}</label>
                <div class="col-sm-9">
                    <select v-model="level" class="form-control">
                        <option value="L">{"L"}</option>
                        <option value="Q">{"Q"}</option>
                        <option value="M">{"M"}</option>
                        <option value="H">{"H"}</option>
                    </select>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-3 control-label">{"Enhanced Mode:"}</label>
                <div class="col-sm-9">
                    <select v-model="renderAs" class="form-control">
                        <option value="svg">{"svg"}</option>
                        <option value="canvas">{"canvas"}</option>
                    </select>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-3 control-label">{"Background:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="color"
                            onchange=self.link.callback(|input: ChangeData| Event::LightColor(input))
                        />
                    </div>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-3 control-label">{"Foreground:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="color"
                            onchange=self.link.callback(|input: ChangeData| Event::DarkColor(input))
                        />
                    </div>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-3 control-label">{"QR_CODE:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        {format!("{:?}", self)}
                    </div>
                </div>
            </div>
        </form>
        }
    }
}
