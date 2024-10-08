use crate::component::slides::Slides;
use crate::talk::slides;
use yew::prelude::*;

#[function_component(Application)]
pub fn app() -> Html {
    let slides = use_memo((), |()| slides());

    html!(
        <Slides slides={(*slides).clone()}/>
    )
}
