mod components;
use leptos::prelude::*;
use components::tasks_view::TasksView;

fn main() {
    leptos::mount::mount_to_body(|| view! {
      <main class="p-6 flex justify-center">
        <TasksView />
      </main>
    });
}
