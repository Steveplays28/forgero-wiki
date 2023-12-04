use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use std::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct PageIndex {
    name: String,
    filename: String,
}

#[derive(Clone, PartialEq)]
struct Page {
    name: String,
    filename: String,
    markdown: String,
}

#[function_component]
fn App() -> Html {
    let pages = use_state(Vec::new);
    {
        let pages = pages.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let pages = pages.clone();

                let fetched_page_indices: Vec<PageIndex> = Request::get("/pages/index.json5")
                    .send()
                    .await
                    .expect("Failed fetching the index!")
                    .json()
                    .await
                    .expect("Failed parsing the index!");
                let mut fetched_pages: Vec<Page> = Vec::new();

                for page_index in fetched_page_indices.iter() {
                    let fetched_page_markdown = Request::get(format!("/pages/{}", &page_index.filename).as_str())
                        .send()
                        .await
                        .expect("Error fetching the Markdown page file!")
                        .text()
                        .await
                        .expect("Error parsing the Markdown page file!");
                    let fetched_page = Page {
                        name: String::from("Main"),
                        filename: String::from("main.md"),
                        markdown: fetched_page_markdown,
                    };

                    log!("{}", &fetched_page.markdown);

                    fetched_pages.push(fetched_page);
                }

                pages.set(fetched_pages);
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
