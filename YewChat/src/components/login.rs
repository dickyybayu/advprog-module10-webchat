use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;
use gloo::utils::window;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let gradients = vec![
        "linear-gradient(to right, #ffecd2, #fcb69f)",
        "linear-gradient(to right, #a1c4fd, #c2e9fb)",
        "linear-gradient(to right, #fdfcfb, #e2d1c3)",
        "linear-gradient(to right, #667eea, #764ba2)",
        "linear-gradient(to right, #89f7fe, #66a6ff)",
    ];
    let rand_index = (js_sys::Math::random() * gradients.len() as f64).floor() as usize;
    let background_style = format!("min-height:100vh; background: {};", gradients[rand_index]);

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div style={background_style} class="flex w-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <form class="m-4 flex">
                    <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" placeholder="Username" />
                    <Link<Route> to={Route::Chat}>
                        <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r">
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
