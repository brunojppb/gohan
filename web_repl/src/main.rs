use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlTextAreaElement};
use yew::prelude::*;

use md_parser::renderer::render_html;

const INITIAL_MD: &str = r"## Hello from Gohan!

Gohan is a [Rust-based](https://www.rust-lang.org/) markdown parser and HTML compiler.
Give it a **try!**.
";

#[function_component(App)]
fn app() -> Html {
    let rendered_html_handle = use_state(|| render_html(INITIAL_MD));
    let html_value: String = (*rendered_html_handle).clone();

    let input_value_handle = use_state(|| INITIAL_MD.to_string());
    let input_value: String = (*input_value_handle).clone();

    let rendered_html = Html::from_html_unchecked(AttrValue::from(html_value));

    let on_change = {
        let html_value = rendered_html_handle.clone();
        let input_value = input_value_handle.clone();

        Callback::from(move |e: KeyboardEvent| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            if let Some(input) = input {
                let h = render_html(&input.value());
                html_value.set(h);
                input_value.set(input.value());
            }
        })
    };

    html! {
        <>
            <div>
                <h1 class="mb-4 text-3xl font-extrabold leading-none tracking-tight text-gray-900 dark:text-white">
                    <span class="text-transparent bg-clip-text bg-gradient-to-r to-emerald-600 from-sky-400">{"Gohan - Markdown Parser"}</span>
                </h1>
            </div>
            <div class="grid grid-cols-2 gap-8 mt-4">
                <h2 class="mb-4 text-xl font-extrabold leading-none tracking-tight text-gray-900 dark:text-white">{"Markdown input"}</h2>
                <h2 class="mb-4 text-xl font-extrabold leading-none tracking-tight text-gray-900 dark:text-white">{"HTML output"}</h2>
            </div>

            <div class="grid grid-cols-2 gap-8 mt-2">
                <textarea
                    onkeyup={on_change}
                    value={input_value}
                    class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                <article class="p-2 prose lg:prose-xl rounded-lg border border-gray-300 dark:border-gray-600 dark:prose-invert">
                    {rendered_html}
                </article>
            </div>
            <p class="mt-4 text-gray-500 dark:text-gray-200 text-xs">
                {"Built with ❤️ by "}<a href="https://x.com/bpaulino0" class="underline">{"Bruno Paulino"}</a> {" ⋅ "}
                <a href="https://github.com/brunojppb/gohan" class="underline">{"This project is open-source 🐙"}</a>
            </p>
        </>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
