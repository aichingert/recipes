use yew::prelude::*;
use wasm_bindgen::{
    JsCast,
    UnwrapThrowExt
};
use web_sys::{
    Event,
    HtmlInputElement,
    InputEvent
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>
}

fn get_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { value, on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value(input_event));
    });

    html! {
        <input type="text" classes={classes!("")} {value} {oninput} />
    }
}
