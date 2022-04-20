use once_cell::sync::Lazy;
use std::path::PathBuf;
use tokio_util::task::LocalPoolHandle;
use warp::Filter;
use yew_issue::App;

static LOCAL_POOL: Lazy<LocalPoolHandle> = Lazy::new(|| LocalPoolHandle::new(num_cpus::get()));

async fn render(index_html_s: &str) -> String {
    let content = LOCAL_POOL
        .spawn_pinned(move || async move {
            let renderer = yew::ServerRenderer::<App>::new();

            renderer.render().await
        })
        .await
        .expect("the task has failed.");

    index_html_s.replace("{insert}", &content) 
}

// cargo run --bin server
#[tokio::main]
async fn main() {
    let base_dir = PathBuf::from("./dist/");
    let index_file = base_dir.join("index.html");

    let index_html_s = tokio::fs::read_to_string(index_file)
        .await
        .expect("failed to read index.html");

    let html = warp::path::end().then(move || {
        let index_html_s = index_html_s.clone();

        async move { warp::reply::html(render(&index_html_s).await) }
    });

    let routes = html.or(warp::fs::dir(base_dir));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
