use yew::{
    prelude::*,
    suspense::{Suspension, SuspensionResult},
};
use yew_macro::Properties;

#[function_component(App)]
pub fn app() -> Html {
    let elems = 0..10;

    html! {
        <Suspense>
        { for elems.clone().map(|number|
            html! {
                <ToSuspendOrNot {number}/>
            }
        )}
        </Suspense>
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
    use_suspend()?;
    Ok(html! {
        <div>{props.number.to_string()}</div>
    })
}

#[function_component(ToSuspendOrNot)]
fn suspend_or_not(props: &NumberProps) -> HtmlResult {
    use_suspend()?;
    let number = props.number;
    Ok(html! {
        if number % 3 == 0 {
            <Suspense>
                <SuspendedNumber {number}/>
            </Suspense>
        } else {
            <Number {number}/>
        }
    })
}

#[hook]
pub fn use_suspend() -> SuspensionResult<()> {
    let s = use_state(|| Suspension::from_future(async {}));

    if s.resumed() {
        Ok(())
    } else {
        Err((*s).clone())
    }
}
