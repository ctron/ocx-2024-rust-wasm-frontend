use crate::component::{
    embed::Embed, image::Image, markdown::Markdown, normal::Normal, section::Section, title::Title,
};
use yew::prelude::*;

pub fn slides() -> Vec<Html> {
    vec![
        html!(<Title title="Rusty Frontends!" subtitle="Creating frontends for WebAssembly using Rust."/>),
        html!(
            <Markdown content={include_str!("who_am_i.md")} />
        ),
        html!(
            <Markdown content={include_str!("shout_out.md")} />
        ),
        html!(
            <Normal title="What to expect?">
                <ul>
                    <li>{"What is WebAssembly?"}</li>
                    <li>{"In the context of Rust"}</li>
                    <li>{"Frameworks and tools"}</li>
                    <li>{"Caveats"}</li>
                    <li>{"So why?"}</li>
                </ul>
            </Normal>
        ),
        html!(
            <Normal title="What is the goal?">
                <ul>
                    <li>{"Get you excited"}</li>
                    <li>{"Encourage you to go learn about it"}</li>
                </ul>
            </Normal>
        ),
        html!(
            <Normal title="What not to expect?">
                <ul>
                    <li>{"Becoming a Rust frontend developer"}</li>
                    <li>{"Making fun of JavaScript"}</li>
                </ul>
            </Normal>
        ),
        html!(
            <Section title="WebAssembly"/>
        ),
        html!(
            <Markdown content={r#"
# What is WebAssembly?

* "WebAssembly (abbreviated WASM) is a binary instruction format for a stack-based virtual machine."
* "WebAssembly 1.0 has shipped in 4 major browser engines."
"#}/>
        ),
        html!(
            <Image src="static/support.png"/>
        ),
        html!(
            <Image src="static/languages.png" />
        ),
        html!(
            <Image src="static/drawing1.webp" title="In the browser" />
        ),
        html!(
            <Section title="In the context of Rust"/>
        ),
        html!(
            <Image src="static/drawing2.webp" title="Cargo build"/>
        ),
        html!(
            <Image src="static/drawing3.webp" />
        ),
        html!(
            <Markdown content=r#"
# Instantiating

```javascript
const module = new WebAssembly.Module(module);
const instance = new WebAssembly.Instance(module,
    my_js_fn: function () {
      // can be called from wasm
    }
});

// calls into wasm
instance.my_rust_fn();
```
"# />
        ),
        html!(
            <Markdown content=r#"
# There and back again

```rust
#[wasm_bindgen]
pub fn my_rust_fn() {
    web_sys::console::log_1(&"Hello from WASM!".into());
}
```
"# />
        ),
        html!(
            <Markdown content=r##"
# Calling into custom JavaScript (inline)

```rust
#[wasm_bindgen(inline_js = r#"
export function snippet_test() {
  console.log('Hello from JS FFI!');
}
"#)]
extern "C" { fn snippet_test(); }
```
"## />
        ),
        html!(
            <Image src="static/drawing4.webp" />
        ),
        html!(
            <Image src="static/drawing5.webp" />
        ),
        html!(
            <Image src="static/drawing6.webp" />
        ),
        html!(
            <Markdown content=r#"
# Trunk

> Build, bundle & ship your Rust WASM application to the web

<https://trunkrs.dev>

* Orchestrate a web frontend build
* Re-use existing tools and dependencies
* Start simple, allow for complex configurations

Available on: crates.io, GitHub releases, Fedora, Brew, NixOS, ...
"#/>
        ),
        html!(
            <Markdown content=r#"
# Trunk (cont.)

* Downloads required tools (except when using `--offline`)
* Asset processing: SASS, Tailwind
* Post-processing: minification, PNG optimization, SRI, copying
* Customized initialization (e.g. loading screen)

And more!

"#/>
        ),
        html!(
            <Markdown content=r#"
# Trunk: 0.21.0 and 0.21.1

🎉 New and noteworthy:

* Cargo profile support
* OS specific hook overrides
* Revamped configuration system: YAML, JSON, Cargo.toml

"#/>
        ),
        html!(
            <Section title="Now what?"/>
        ),
        html!(
            <Image src="static/construct.png" />
        ),
        html!(
            <Markdown content=r##"
# Yes, but no!
```rust
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // ...
}
```
"## />
        ),
        html!(
            <Markdown content=r##"
# Gloo! But still, no!
```rust
use gloo_utils::body;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let body = body();

    // ...
}
```
"## />
        ),
        html!(
            <Image src="static/frameworks.png" />
        ),
        html!(
            <Normal title="Frameworks">
                <div class="ocx-columns">
                    <div>
                        <h2>{"Yew"}</h2>
                        <ul>
                            <li><strong>{"Established"}</strong></li>
                            <li>{"⭐ ~30.6k"}</li>
                            <li>{"Uses VDOM"}</li>
                            <li>{"Inspired by React"}</li>
                        </ul>
                    </div>
                    <div>
                        <h2>{"Dioxus"}</h2>
                        <ul>
                            <li><strong>{"Growing Fast"}</strong></li>
                            <li>{"⭐ ~20.5k"}</li>
                            <li>{"Uses VDOM*"}</li>
                            <li>{"Faster VDOM"}</li>
                        </ul>
                    </div>
                    <div>
                        <h2>{"Leptos"}</h2>
                        <ul>
                            <li><strong>{"Growing Fast"}</strong></li>
                            <li>{"⭐ ~16k"}</li>
                            <li>{"No VDOM"}</li>
                            <li>{"Inspired by SolidJS"}</li>
                        </ul>
                    </div>
                </div>

            </Normal>
        ),
        html!(
            <Image src="static/screenshot1.png" />
        ),
        html!(
            <Markdown max=true content=r#"
```rust
#[function_component(Example)]
fn example() -> Html {
  let selected = use_state_eq(|| 1);
  let onselect = use_callback(selected.clone(), |index, selected| selected.set(index));

  html!(
    <Tabs<usize> selected={*selected} {onselect}>
      <Tab<usize> index=1 title="Foo">
        {"Foo"}
      </Tab<usize>>
      <Tab<usize> index=2 title="Bar">
        {"Bar"}
      </Tab<usize>>
      <Tab<usize> index=3 title="Baz">
        {"Baz"}
      </Tab<usize>>
    </Tabs<usize>>
  )
}
```
"# />
        ),
        html!(
            <Markdown max=true content=r#"
# Getting started (with Yew)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Add WebAssembly target
rustup target add wasm32-unknown-unknown
# Install Trunk & cargo-generate
cargo install --locked trunk
cargo install --locked cargo-generate
# Generate a template
cargo generate --git https://github.com/yewstack/yew-trunk-minimal-template
# Serve
trunk serve --open
```

Also see: <https://yew.rs/docs/getting-started/introduction>

"#/>
        ),
        html!(
            <Markdown content=r#"
# Evo 2024

* > The Largest E-Sports Event In The World!
* > I wanted a fairly sophisticated PWA including accounts, state management, QR code scanning, picture taking, file upload, emails, printing, and ...
* > I like my development and production environments to have essentially no differences.
Using Trunk for Development is convenient, but […]

– <https://blog.danielc.us/rust-wasm-evo-2024>

"#/>
        ),
        html!(<Markdown content=r#"

# Leptos example

```rust
#[component]
pub fn Landing<F>(landing_view_toggle: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <Box class="landing">
            <button on:click:undelegated=landing_view_toggle>"Chat!"</button>
        </Box>
    }
}
```

"#/>),
        html!(
            <Section title="Caveats"/>
        ),
        html!(
            <Normal title="On the con side">
                <div class="ocx-columns">
                    <div>
                        <h2>{"Technical"}</h2>
                        <ul>
                            <li>
                                {"The web is JS first¹"}
                            </li>
                            <li>{"You still need some JS expertise"}</li>
                        </ul>
                    </div>
                    <div>
                        <h2>{"Social"}</h2>
                        <ul>
                            <li>{"Some people just seem to enjoy JavaScript"}</li>
                            <li>{"There are more JS frontend developers than Rust frontend developers²"}</li>
                            <li>{"In the context of frontends, there is more information for JS than for Rust"}</li>
                        </ul>
                    </div>
                </div>
                <p class="ocx-footnote">
                    {"¹ Although a good number of JS dependencies and tools use WASM under the hood"} <br />
                    {"² Doesn't matter if you don't have them"}
                </p>
            </Normal>
        ),
        html!(
            <Section title="So why?"/>
        ),
        html!(
            <Markdown content=r#"
# On the pro side

* Same language for backend and frontend!
* Look how fast it is!
* Rust is just more fun!
"# />
        ),
        html!(
            <Markdown content=r#"
# Performance (ElementX)

> **Then to achieve absolute peak performance, the core of Element X is written in Rust**: powered by matrix-rust-sdk.  Rust provides zero cost abstractions, meaning that higher-level programming abstractions do not incur a performance penalty: the app’s engine runs as fast as it possibly can.

> Rust is also the key to Element X’s incredible stability. **By using a single SDK for all the heavy lifting, shared between iOS and Android,** we can ensure that bug fixes on one platform immediately benefit the other - and meanwhile Rust’s intrinsic memory safety, type safety and concurrency safety help ensure the engine is as robust as possible.

– <https://element.io/blog/element-x-ignition/>

"# />
        ),
        html!(
            <Image src="static/benchmarks1.png"/>
        ),
        html!(
            <Image src="static/speed1.png"/>
        ),
        html!(
            <Image src="static/speed2.png"/>
        ),
        html!(
            <Image src="static/speed3.png"/>
        ),
        html!(
            <Section title="Wait, there is more!"/>
        ),
        html!(
            <Markdown content=r#"
# Tauri

> Tauri is a polyglot and generic toolkit that is very composable and allows engineers to make a wide variety of applications. It is used for building applications for desktop computers using a combination of Rust tools and HTML rendered in a Webview.

Kind of like Electron.

"# />
        ),
        html!(
            <Markdown content=r#"
# Bevy

> A refreshingly simple data-driven game engine built in Rust

"# />
        ),
        html!(
            <Embed src="https://bevyengine.org/examples/animation/animated-fox/"/>
        ),
        html!(
            <Markdown content=r#"
# Ok, now what?

* Go and explore!
* Start small!
* Start fresh!
* Embrace JS when it makes sense!

"#/>
        ),
        html!(
            <Markdown content=r#"
# Links

### Book
* <https://rustwasm.github.io/docs/book/>

### Rust and WebAssembly Docs
* <https://rustwasm.github.io/wasm-bindgen/>
* <https://rustwasm.github.io/wasm-pack>

### This presentation
* <https://ctron.github.io/ocx-2024-rust-wasm-frontend/>

"#/>
        ),
        html!(
            <Markdown content=r#"
# Links (cont.)

<div class="ocx-columns">

<div>

### Trunk (Build, Bundle, Ship)
* <https://trunkrs.dev>
### wasm-pack
* <https://github.com/rustwasm/wasm-pack>

</div><div>

### Yew (Uses VDOM)
* <https://yew.rs/docs/getting-started/introduction>
* <https://github.com/jetli/awesome-yew>
### Leptos (No VDOM)
* <https://book.leptos.dev/> // Introduction
* <https://github.com/leptos-rs/awesome-leptos>

</div>

</div>

"#/>
        ),
        html!(
            <Markdown content=r#"
# Links (cont.)
### Tauri (Desktop / Mobile Apps)
* <https://github.com/tauri-apps/create-tauri-app>
* <https://github.com/tauri-apps/awesome-tauri>
### Bevy (Game Engine)
* <https://github.com/bevyengine/bevy>
"#/>
        ),
        html!(
            <Section title="🗨️ Questions?"/>
        ),
        html!(
            <Section title="❤️ Thank you!"/>
        ),
    ]
}
