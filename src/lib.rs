use yew::{
    prelude::*,
    suspense::{Suspension, SuspensionResult},
};
use yew_macro::Properties;

#[function_component(App)]
pub fn app() -> Html {
    let fallback = html!{ <h1>{"Loading..."}</h1> };
    html! {
        <Suspense {fallback}>
            <Suspended/>
        </Suspense>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct NumberProps {
    number: u32,
}

#[function_component(SuspendedNumber)]
fn suspended_number(props: &NumberProps) -> HtmlResult {
    use_suspend()?;

    Ok(html! {
        <Number ..{props.clone()}/>
    })
}
#[function_component(Number)]
fn number(props: &NumberProps) -> Html {
    html! {
        <div>
            {props.number.to_string()}
        </div>
    }
}

#[function_component(Suspended)]
fn suspended() -> HtmlResult {
    use_suspend()?;

    Ok(html! {
        { for (0..10).clone().map(|number|
            html! {
                <SuspendedNumber {number}/> // Flashes the screen
                // <Number {number}/> // Does work
            }
        )}
    })
}

#[hook]
pub fn use_suspend() -> SuspensionResult<()> {
    let s = use_state(|| Suspension::from_future(async {
        #[cfg(target_arch = "wasm32")]
        gloo::timers::future::sleep(std::time::Duration::from_millis(300)).await;
    }));

    if s.resumed() {
        Ok(())
    } else {
        Err((*s).clone())
    }
}
