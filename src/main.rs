use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use std::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/pages/:page_name")]
    WikiPage { page_name: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, PartialEq, Properties)]
struct PageName {
    name: String,
}

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

fn main() {
    yew::Renderer::<App>::new().render();
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <Home />
        },
        Route::WikiPage { page_name } => {
            html! {
                <WikiPage name={page_name} />
            }
        }
        Route::NotFound => html! {<h1>{"404"}</h1>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

#[function_component(Home)]
fn home() -> Html {
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
                    let page_index = page_index.clone();
                    let fetched_page_markdown =
                        Request::get(format!("/pages/{}", &page_index.filename).as_str())
                            .send()
                            .await
                            .expect("Error fetching the Markdown page file!")
                            .text()
                            .await
                            .expect("Error parsing the Markdown page file!");
                    let fetched_page = Page {
                        name: page_index.name,
                        filename: page_index.filename,
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

#[function_component(WikiPage)]
fn wiki_page(page_name: &PageName) -> Html {
    let page_name = page_name.clone();
    let page = use_state(|| Page {
        name: String::from("loading"),
        filename: String::from("loading.md"),
        markdown: String::from("# Loading..."),
    });
    {
        let page = page.clone();

        use_effect_with((), move |_| {
            let page = page.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let page = page.clone();
                let page_name = page_name.clone();
                // TODO: Cache fetched page indices
                // TODO: Move page index file path into a static variable
                let fetched_page_indices: Vec<PageIndex> = Request::get("/pages/index.json5")
                    .send()
                    .await
                    .expect("Failed fetching the index!")
                    .json()
                    .await
                    .expect("Failed parsing the index!");

                for fetched_page_index in fetched_page_indices {
                    log!(fetched_page_index.name.clone());
                    log!(page_name.name.clone());

                    if fetched_page_index.name == page_name.name {
                        let fetched_page_markdown = Request::get(
                            format!("/pages/{}", &fetched_page_index.filename).as_str(),
                        )
                        .send()
                        .await
                        .expect("Error fetching the Markdown page file!")
                        .text()
                        .await
                        .expect("Error parsing the Markdown page file!");

                        page.set(Page {
                            name: fetched_page_index.name,
                            filename: fetched_page_index.filename,
                            markdown: fetched_page_markdown,
                        });
                    }
                }
            });
            || ()
        });
    }

    let page_html = markdown::to_html(page.markdown.as_str());
    let page_html_nodes = Html::from_html_unchecked(AttrValue::from(page_html));

    html! {
        <div class="markdown-page-container">
        {
            page_html_nodes
        }
        </div>
    }
}
