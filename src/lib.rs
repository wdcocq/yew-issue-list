use yew::prelude::*;
use yew_macro::Properties;

#[cfg(not(target_arch = "wasm32"))]
#[function_component(App)]
pub fn app() -> Html {
    let size = use_state(|| 4);

    html! {
        <div>
            <List size={*size}/>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
#[function_component(App)]
pub fn app() -> Html {
    use yew::callback;

    let size = use_state(|| 4);

    let callback = {
        let size = size.clone();
        Callback::from(move |_| {
            size.set(10);
        })
    };

    html! {
        <div onclick={callback}>
            <List size={*size}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ListProps {
    size: u32,
}

#[function_component(List)]
fn list(props: &ListProps) -> Html {
    let elems = 0..props.size;

    html! {
        <>
        { for elems.clone().map(|_|
            html! {
                <Test2/> // fails
                // <Test1> // works
            }
        )}
        </>
    }
}

#[function_component(Test1)]
fn test1() -> Html {
    html! {
        <span>{"test"}</span>
    }
}
#[function_component(Test2)]
fn test2() -> Html {
    html! {
        <Test1/>
    }
}