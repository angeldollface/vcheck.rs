/*
VCHECK.RS by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We load Yew's APIs.
use yew::prelude::*;

// We define the footer
// component.
#[function_component]
pub fn FooterCog () -> Html {

    // The HTML to render to the DOM.
    // A link to the source.
    return html!{
        <div class="footer">
         <p class="footer">
          { "VCheck on "}
          <a href="https://github.com/iamtheblackunicorn/vcheck.rs">
           { "GitHub" }
          </a>
         </p>
        </div>
    };
}