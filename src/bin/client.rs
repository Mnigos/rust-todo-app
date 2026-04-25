use leptos::prelude::*;
use rust_todo_app::components::tasks_view::TasksView;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
          <main class="flex justify-center p-6">
            <TasksView />
          </main>
        }
    });
}
