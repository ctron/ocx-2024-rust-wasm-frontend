use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct NormalProperties {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Normal)]
pub fn normal(props: &NormalProperties) -> Html {
    html!(
        <section class="ocx-default">
            <h1>{ &props.title }</h1>
            { &props.children }
        </section>
    )
}
