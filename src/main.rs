/*
VCHECK.RS by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We declare
// a module
// for "components".
mod components;

// We load Yew's API.
use yew::prelude::*;

// We load the footer section.
use components::FooterCog::FooterCog;

// We load the heading section.
use components::HeadingCog::HeadingCog;

// We load the processing component.
use components::CheckerCog::CheckerCog;

// We declare our main
// component "App".
#[function_component]
fn App() -> Html {
    return html! {
        <>
         <div class="content">
          <HeadingCog/>
          <CheckerCog/>
         </div>
         <FooterCog/>
        </>
    };
}

/// We load "App" and render it.
fn main() {
    yew::Renderer::<App>::new().render();
}