use yew::prelude::*;
use yewtil::{Pure, PureComponent};

#[derive(Clone, Properties, PartialEq)]
pub struct PureTextInputField {
    pub label: String,
    pub placeholder: String,
    pub value: String,
    pub oninput: Callback<yew::events::InputData>,
    #[prop_or(false)]
    pub disabled: bool,
}
pub type TextInputField = Pure<PureTextInputField>;

impl PureComponent for PureTextInputField {
    fn render(&self) -> Html {
        html! {
            <div class="field">
                <label class="label">{ &self.label }</label>
                <div class="control has-icons-left">
                    <input
                        class="input" type="text"
                        placeholder=&self.placeholder
                        value=&self.value
                        oninput=&self.oninput
                        disabled=self.disabled
                        />
                    <span class="icon is-small is-left">
                        <i class="fas fa-server"></i>
                    </span>
                </div>
            </div>
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct PureTextAreaField {
    pub label: String,
    pub placeholder: String,
    pub value: String,
    pub oninput: Callback<yew::events::InputData>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(None)]
    pub class: Option<String>,
}
pub type TextAreaField = Pure<PureTextAreaField>;

impl PureComponent for PureTextAreaField {
    fn render(&self) -> Html {
        html! {
            <div class="field">
                <label class="label">{ &self.label }</label>
                <div class="control has-icons-left">
                    <textarea
                        class=("textarea", &self.class)
                        placeholder=&self.placeholder
                        value=&self.value
                        oninput=&self.oninput
                        disabled=self.disabled
                    ></textarea>
                </div>
            </div>
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct PureProgressBar {
    pub progress: f32,
    pub class: String,
}
pub type ProgressBar = Pure<PureProgressBar>;

impl PureComponent for PureProgressBar {
    fn render(&self) -> Html {
        let percentage = ((self.progress * 100.0).round() as u32).min(100).max(0);
        html! {
            <progress class=("progress", &self.class) value=percentage max="100">{ percentage }{ "%" }</progress>
        }
    }
}
