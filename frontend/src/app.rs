use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    console::log_1(&format!("rerender").into());

    let username_value_handle = use_state(String::default);

    let on_username_change = {
        let username_value_handle = username_value_handle.clone();

        Callback::from(move |e: Event| {
            let new_username = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
            console::log_1(&format!("onusernamechange: {new_username}").into());
            username_value_handle.set(new_username);
        })
    };

    let password_value_handle = use_state(String::default);

    let on_password_change = {
        let password_value_handle = password_value_handle.clone();

        Callback::from(move |e: Event| {
            let new_password = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
            console::log_1(&format!("onpasswordchange: {new_password}").into());
            password_value_handle.set(new_password);
        })
    };

    let on_submit = {
        let username_value_handle = username_value_handle.clone();
        let password_value_handle = password_value_handle.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username_value_handle).clone();
            let password = (*password_value_handle).clone();

            console::log_1(&format!("submit username: {username}, password: {password}").into());
        })
    };

    html! {
        <form onsubmit={on_submit} class="d-flex">
            <input
                id="login-username"
                onchange={on_username_change}
                value={(*username_value_handle).clone()}
                required=true
                class="form-control me-2"
                type="username"
            />
            <input
                id="login-password"
                onchange={on_password_change}
                value={(*password_value_handle).clone()}
                required=true
                class="form-control me-2"
                type="password"
            />
            <button class="btn btn-outline-success" type="submit" id="login-button">
                { "Login" }
            </button>
        </form>
    }
}
