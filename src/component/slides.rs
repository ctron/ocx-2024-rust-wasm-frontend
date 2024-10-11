use yew::prelude::*;
use yew_more_hooks::prelude::use_page_state;

#[derive(Properties, Clone, PartialEq)]
pub struct SlidesProperties {
    pub slides: Vec<Html>,
}

#[function_component(Slides)]
pub fn slide(props: &SlidesProperties) -> Html {
    let current = use_page_state(|| 0usize);
    let slide = use_memo(
        (*current, props.slides.clone()),
        |(current, children)| match children.get(*current) {
            Some(slide) => slide.clone(),
            None => html!("No content").into(),
        },
    );

    let num_slides = props.slides.len();
    yew_hooks::use_event_with_window("keydown", move |e: KeyboardEvent| {
        log::info!("{} is pressed!", e.key());

        match e.key().as_str() {
            "ArrowLeft" | "PageUp" | "b" => {
                if *current > 0 {
                    current.set(*current - 1);
                }
            }
            "ArrowRight" | "PageDown" | " " | "n" => {
                if *current + 1 < num_slides {
                    current.set(*current + 1)
                }
            }
            "Home" | "Escape" | "h" => {
                current.set(0);
            }
            "End" | "e" => {
                if num_slides > 1 {
                    current.set(num_slides - 1);
                }
            }
            "." | "f" => {
                let _ = gloo_utils::document_element().request_fullscreen();
            }
            _ => {}
        }
    });

    html!(
        <main>
            { (*slide).clone() }
        </main>
    )
}
