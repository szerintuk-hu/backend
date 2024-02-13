use web_sys::HtmlInputElement;
use web_sys::console;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let input_ref = NodeRef::default();
    let input_ref_clone = input_ref.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        if let Some(input) = input_ref_clone.cast::<HtmlInputElement>() {
            let input_value = input.value();
            console::log_1(&input_value.into());
        }
    });

    html! {
        <>
            <h1>{ "Szerintük.hu" }</h1>
            <form onsubmit={onsubmit}>
                <input type="text" ref={input_ref} />
                <button type="submit">{ "Új kérdés létrehozása" }</button>
            </form>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}