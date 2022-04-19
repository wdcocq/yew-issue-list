use log::info;
use yew::prelude::*;
use yew_macro::Properties;

// #[cfg(not(target_arch = "wasm32"))]
#[function_component(App)]
pub fn app() -> Html {
    let size = use_state(|| 10);

    html! {
        <div>
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
        { for elems.clone().map(|number|
            html! {
                <ToSuspendOrNot {number}/>
            }
        )}
        </>
    }
}

#[derive(Properties, PartialEq)]
struct NumberProps {
    number: u32,
}

#[function_component(Number)]
fn number(props: &NumberProps) -> Html {
    html! {
        <div>{props.number.to_string()}</div>
    }
}
#[function_component(SuspendedNumber)]
fn suspended_number(props: &NumberProps) -> HtmlResult {
    sleep::use_sleep()?;
    Ok(html! {
        <div>{props.number.to_string()}</div>
    })
}
#[function_component(ToSuspendOrNot)]
fn suspended_number(props: &NumberProps) -> Html {
    let number = props.number;
    html! {
        <Suspense>
            if number % 3 == 0 {
                <SuspendedNumber {number}/>
            } else {
                <Number {number}/>
            }
        </Suspense>
    }
}

mod sleep {
    use std::rc::Rc;
    use yew::prelude::*;
    use yew::suspense::{Suspension, SuspensionResult};

    #[derive(PartialEq)]
    pub struct SleepState {
        s: Suspension,
    }

    impl SleepState {
        fn new() -> Self {
            let s = Suspension::from_future(async {
                #[cfg(target_arch = "wasm32")]
                gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
            });

            Self { s }
        }
    }

    impl Reducible for SleepState {
        type Action = ();

        fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
            Self::new().into()
        }
    }

    #[hook]
    pub fn use_sleep() -> SuspensionResult<Rc<dyn Fn()>> {
        let sleep_state = use_reducer(SleepState::new);

        if sleep_state.s.resumed() {
            Ok(Rc::new(move || sleep_state.dispatch(())))
        } else {
            Err(sleep_state.s.clone())
        }
    }
}