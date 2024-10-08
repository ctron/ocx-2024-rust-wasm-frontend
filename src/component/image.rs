use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ImageProperties {
    pub src: AttrValue,
    #[prop_or_default]
    pub title: Option<AttrValue>,
}

#[function_component(Image)]
pub fn section(props: &ImageProperties) -> Html {
    html!(
        <section class="ocx-image">
            if let Some(title) = &props.title {
                <h1>{title}</h1>
            }
            <img src={&props.src} />
        </section>
    )
}
