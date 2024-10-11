use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EmbedProperties {
    pub src: AttrValue,
}

#[function_component(Embed)]
pub fn embed(props: &EmbedProperties) -> Html {
    html!(
        <iframe class="ocx-embed" src={&props.src}/>
    )
}
