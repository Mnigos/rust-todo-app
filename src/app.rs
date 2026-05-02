use crate::tasks::presentation::views::TasksView;
use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos_meta::MetaTags;

    view! {
      <!DOCTYPE html>
      <html>
        <head>
          <AutoReload options=options.clone() />
          <HydrationScripts options />
          <MetaTags />
        </head>
        <body class="bg-background dark">
          <App />
        </body>
      </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
      <Stylesheet id="leptos" href="/pkg/rust-todo-app.css"/>
      <Title text="Welcome to Leptos"/>

      <Router>
        <main class="flex justify-center p-6">
          <Routes fallback=|| "Page not found".into_view()>
              <Route path=StaticSegment("") view=HomePage />
          </Routes>
        </main>
      </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
      <TasksView />
    }
}
