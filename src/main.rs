/*
VCHECK.RS by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We declare the "components"
/// directory as a module.
mod components;

/// We load Yew's API.
use yew::prelude::*;

/// We load the footer section.
use components::footer_cog::FooterCog;

/// We load the heading section.
use components::heading_cog::HeadingCog;

/// We load the processing component.
use components::checker_cog::CheckerCog;

/// We declare our main
/// component "App".
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