use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn age(&self) -> usize {
        2 * 2
    }
}

fn comp() -> Html {
    let personal = Person { name: "hahahha" };
    html! {
        <div style="color:red">{"tests comp"}
            <div class={classes!("name")}>{personal.name}{ "age is: "}{personal.age()}</div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    num: u32,
}

#[function_component]
pub fn NumberComp(props: &Props) -> Html {
    log(&props.num.to_string());
    html! {
        <div>{"number is: "}{props.num}</div>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    HomeComp,
    #[at("/one")]
    PageOne,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomeComp => html! {<HomeComp />},
        Route::PageOne => html!(<PageOne />),
    }
}

#[function_component]
fn HomeComp() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        web_sys::console::log_1(&greeting.into());
    });

    let handlejump = Callback::from(move |_| navigator.push(&Route::PageOne));

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            {comp()}
            <NumberComp num={99}/>
            <button {onclick}>{ "Click" }</button>
            <button onclick={handlejump}>{ "go page one" }</button>
        </main>
    }
}

#[function_component]
fn PageOne() -> Html {
    html! {
        <div>{"page-one"}</div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
