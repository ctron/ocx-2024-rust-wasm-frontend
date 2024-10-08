use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SectionProperties {
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
}

#[function_component(Section)]
pub fn section(props: &SectionProperties) -> Html {
    html!(
        <section class="ocx-section">
            <h1>{ &props.title }</h1>
            if let Some(subtitle) = &props.subtitle {
                <div class="subtitle">{ subtitle }</div>
            }
        </section>
    )
}
