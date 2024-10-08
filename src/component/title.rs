use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TitleProperties {
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
}

#[function_component(Title)]
pub fn title(props: &TitleProperties) -> Html {
    html!(
        <section class="ocx-title">
            <div>
                <h1>{ &props.title }</h1>
                if let Some(subtitle) = &props.subtitle {
                    <div class="subtitle">{ subtitle }</div>
                }
            </div>
        </section>
    )
}
