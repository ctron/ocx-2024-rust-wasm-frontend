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
            <Image src="static/drawing1.svg" title="In the browser" />
        ),
        html!(
            <Section title="In the context of Rust"/>
        ),
        html!(
            <Image src="static/drawing2.svg" title="Cargo build"/>
        ),
        html!(
            <Image src="static/drawing3.svg" />
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
            <Image src="static/drawing4.svg" />
        ),
        html!(
            <Image src="static/drawing5.svg" />
        ),
        html!(
            <Image src="static/drawing6.svg" />
        ),
        html!(
            <Markdown content=r#"
# Trunk

> Build, bundle & ship your Rust WASM application to the web

<https://trunkrs.dev>

* Orchestrate a web frontend build
* Re-use existing tools and dependencies
* Start simple, allow for complex configurations

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
                            <li>{"Works like ReactJS"}</li>
                        </ul>
                    </div>
                    <div>
                        <h2>{"Dioxus"}</h2>
                        <ul>
                            <li><strong>{"Growing Fast"}</strong></li>
                            <li>{"⭐ ~20.5k"}</li>
                            <li>{"Works like ReactJS"}</li>
                        </ul>
                    </div>
                </div>

                <div class="ocx-columns">
                    <div>
                        <h2>{"Leptos"}</h2>
                        <ul>
                            <li><strong>{"Growing Fast"}</strong></li>
                            <li>{"⭐ ~16k"}</li>
                            <li>{"Works like SolidJS"}</li>
                        </ul>
                    </div>
                    <div>
                        <h2>{"Sycamore"}</h2>
                        <ul>
                            <li><strong>{"Fresh"}</strong></li>
                            <li>{"⭐ ~2.8k"}</li>
                            <li>{"Works like SolidJS"}</li>
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

– <https://danielc.us/rust-wasm-evo-2024>

"#/>
        ),
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
# Performance

* Comparison (TODO)
* Element X, using Rust
* And, …
"# />
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

* <https://rustwasm.github.io/wasm-bindgen/>
* <https://trunkrs.dev>
* <https://yew.rs/docs/getting-started/introduction>
* <https://github.com/jetli/awesome-yew>
* <https://github.com/leptos-rs/awesome-leptos>
* <https://github.com/tauri-apps/awesome-tauri>
"#/>
        ),
        html!(
            <Section title="Questions?"/>
        ),
        html!(
            <Section title="❤️ Thank you!"/>
        ),
    ]
}
