mod components;
mod models;

use components::tasks_view::TasksView;
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
          <main class="p-6 flex justify-center">
            <TasksView />
          </main>
        }
    });
}
