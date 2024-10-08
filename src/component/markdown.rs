use crate::component::safe_html::SafeHtml;
use comrak::{
    markdown_to_html_with_plugins, plugins::syntect::SyntectAdapterBuilder,
    ExtensionOptionsBuilder, Options, PluginsBuilder, RenderOptionsBuilder, RenderPluginsBuilder,
};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MarkdownProperties {
    #[prop_or_default]
    pub content: String,

    #[prop_or_default]
    pub max: bool,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProperties) -> Html {
    let html = use_memo(props.content.clone(), |markdown| {
        markdown_to_html_with_plugins(
            markdown,
            &Options {
                extension: {
                    ExtensionOptionsBuilder::default()
                        .strikethrough(true)
                        .build()
                        .unwrap_or_default()
                },
                render: RenderOptionsBuilder::default()
                    .unsafe_(true)
                    .build()
                    .unwrap_or_default(),
                ..Default::default()
            },
            &PluginsBuilder::default()
                .render(
                    RenderPluginsBuilder::default()
                        .codefence_syntax_highlighter(Some(
                            &SyntectAdapterBuilder::new()
                                .theme("base16-ocean.dark")
                                .build(),
                        ))
                        .build()
                        .unwrap_or_default(),
                )
                .build()
                .unwrap_or_default(),
        )
    });

    let mut class = classes!("ocx-default");
    if props.max {
        class.push("ocx-max");
    }

    html!(
        <section {class}>
            <SafeHtml html={(*html).clone()} />
        </section>
    )
}
