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

/// Progress must be 0 <= x <= 1, or it will be clamped otherwise.
pub fn progress_bar(progress: f32, class: impl AsRef<str>) -> Html {
    let percentage = ((progress * 100.0).round() as u32).min(100).max(0);
    html! {
        <progress class=("progress", class.as_ref()) value=percentage max="100">{ percentage }{ "%" }</progress>
    }
}
