use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::use_interval;

#[derive(Properties, PartialEq)]
pub struct EmbedProperties {
    pub src: AttrValue,
}

#[function_component(Embed)]
pub fn embed(props: &EmbedProperties) -> Html {
    let outer = use_node_ref();

    {
        let outer = outer.clone();
        use_interval(
            move || {
                if let Some(ele) = outer.cast::<HtmlElement>() {
                    if let Err(err) = ele.focus() {
                        web_sys::console::log_2(&"Enable to focus:".into(), &err);
                    }
                } else {
                    web_sys::console::log_1(&"Element not found".into());
                }
            },
            1000,
        );
    }

    html!(
        <div ref={outer}>
            <iframe class="ocx-embed" src={&props.src}/>
        </div>
    )
}
