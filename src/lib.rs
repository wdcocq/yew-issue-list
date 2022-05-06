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
            <ToSuspend depth={3} range={(*range).clone()}/>
        </Suspense>
    }
}

#[derive(Properties, PartialEq)]
struct ToSuspendProps {
    depth: u64,
    range: Range<u32>,
}

#[function_component(ToSuspend)]
fn to_suspend(ToSuspendProps { depth, range }: &ToSuspendProps) -> HtmlResult {
    use_suspend(500 / (depth + 1))?;
    Ok(html! {
        if *depth > 0 {
            <Suspense>
                {for range.clone().map(|i|
                    html!{ <div>{format!("{i}, depth: {depth}")}</div> }
                )}
                <ToSuspend depth={depth - 1} range={range.clone()}/>
            </Suspense>
        }
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
