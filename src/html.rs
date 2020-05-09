use yew::prelude::*;

pub fn input_field(
    label: impl ToString,
    placeholder: impl ToString,
    value: impl ToString,
    oninput: Callback<yew::events::InputData>,
) -> Html {
    html! {
        <div class="field">
            <label class="label">{ label.to_string() }</label>
            <div class="control has-icons-left">
                <input
                    class="input" type="text"
                    placeholder=placeholder
                    value=value
                    oninput=oninput
                    />
                <span class="icon is-small is-left">
                    <i class="fas fa-server"></i>
                </span>
            </div>
        </div>
    }
}
