use gloo_console::log;
use gloo_net::http::Request;
use std::*;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Page {
    name: String,
    markdown: String,
}

#[function_component]
fn App() -> Html {
    let pages = use_state(Vec::new);
    {
        let pages = pages.clone();

        use_effect_with((), move |_| {
            let pages = pages.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let pages = pages.clone();

                let fetched_page_markdown = Request::get("/pages/main.md")
                    .send()
                    .await
                    .expect("Error fetching the Markdown page file!")
                    .text()
                    .await
                    .expect("Error parsing the Markdown page file!");
                let fetched_page = Page {
                    name: String::from("Main"),
                    markdown: fetched_page_markdown,
                };

                log!("{}", &fetched_page.markdown);

                pages.set(vec![fetched_page]);
            });
            || ()
        });
    }

    html! {
        <div class="markdown-page-container">
        {
            if pages.len() > 0 {
                let main_page_html = markdown::to_html(pages[0].markdown.as_str());
                let main_page_html_nodes = Html::from_html_unchecked(AttrValue::from(main_page_html));

                return main_page_html_nodes
            }
        }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
