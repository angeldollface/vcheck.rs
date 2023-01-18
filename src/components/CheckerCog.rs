/*
VCHECK.RS by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We load the Rust library
// that implements my algorithm.
use flek::*;

// We load Yew's APIs.
use yew::prelude::*;

// We need this to work with
// events.
use wasm_bindgen::JsCast;

// We need this to capture event
// results.
use web_sys::EventTarget;

// We need this to interact
// with the HTML "input"
// element.
use web_sys::HtmlInputElement;

// We define the main password processing
// component.
#[function_component]
pub fn CheckerCog() -> Html {

    // Setting up our stateful result information holder as a string.
    let number_result: UseStateHandle<String> = use_state(|| "".to_owned());

    // Setting up our stateful result information holder as a string.
    let bool_result: UseStateHandle<String> = use_state(|| "".to_owned());

    // We capture the user's input in this stateful variable.
    let user_pwd: UseStateHandle<String> = use_state(|| "".to_owned());

    // We initialize a stateful string variable to suggest a password.
    let pwd_suggestion: UseStateHandle<String> = use_state(|| "".to_owned());

    // Defining the closure to intercept and process user input.
    let onchange = {

        // We clone our initial variable to re-use it.
        let user_pwd_cloned: UseStateHandle<String> = user_pwd.clone();

        // We instantiate a callback closure in which we capture the
        // password that the user types and update the stateful variable.
        Callback::from(
            move |event: Event| {

                // We get the event's target: "input".
                let target: EventTarget = event.target().unwrap();

                // We convert it into an HTML element.
                let input: HtmlInputElement = target.unchecked_into::<HtmlInputElement>();

                // We update "user_pwd" with the user's input.
                user_pwd_cloned.set(input.value());
            }
        )
    };

    // Defining our closure to intercept button clicks
    // and render feedback on the user's password.
    let onclick = {

        // We clone our initial variable to re-use it.
        let number_result_clone = number_result.clone();

        // We clone our initial variable to re-use it.
        let bool_result_clone = bool_result.clone();

        // We clone our initial variable to re-use it.
        let pwd_suggestion_clone = pwd_suggestion.clone();

        // We make a closure to interact with our variables
        // when a button is clicked.
        move |_| {

            number_result_clone.set(password_strength(&user_pwd).to_string());
            pwd_suggestion_clone.set(generate_password(&8));

            // Is it a strong password? 
            // We update "result" accordingly.
            if is_secure(&user_pwd.to_string()) {
                bool_result_clone.set(String::from("Yes."));
            }

            // If the password isn't secure.
            else {
                bool_result_clone.set(String::from("No."));
            }
        }
    };

    // The HTML to render to the DOM.
    // The "input", "button", and feedback paragraphs.
    return html!{
        <>
         <input type="text" {onchange} placeholder={ "YOUR PASSWORD GOES HERE." }/>
         <button {onclick}>{ "CHECK" }</button>
         <ul>
          <p>{ format!("Security score: {}", &number_result.clone().to_string()) }</p>
          <p>{ format!("Password secure: {}", &bool_result.clone().to_string()) }</p>
          <p>{ format!("Suggested password: {}", &pwd_suggestion.clone().to_string()) }</p>
         </ul>
        </>
    }
}
