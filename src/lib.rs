use std::ops::Range;
use yew::{prelude::*, suspense::SuspensionResult};
use yew_macro::Properties;

#[function_component(App)]
pub fn app() -> Html {
    let range = use_state(|| 0u32..5);
    {
        let range = range.clone();
        use_effect_with_deps(
            move |_| {
                range.set(0..6);
                || ()
            },
            (),
        );
    }

    html! {
        <Suspense>
            <ToSuspend range={(*range).clone()}/>
        </Suspense>
    }
}

#[derive(Properties, PartialEq)]
struct ToSuspendProps {
    range: Range<u32>,
}

#[function_component(ToSuspend)]
fn to_suspend(ToSuspendProps { range }: &ToSuspendProps) -> HtmlResult {
    use_suspend(100)?;
    Ok(html! {
        { for range.clone().map(|i|
            html!{ <div key={i}>{i}</div> }
        )}
    })
}

#[hook]
pub fn use_suspend(_wait: u64) -> SuspensionResult<()> {
    yew::suspense::use_future(|| async move {
        #[cfg(target_arch = "wasm32")]
        gloo::timers::future::sleep(std::time::Duration::from_millis(_wait)).await;
    })?;

    Ok(())
}
