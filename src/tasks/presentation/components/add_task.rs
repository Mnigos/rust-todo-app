use crate::components::ui::{button::Button, input::Input};
use leptos::prelude::*;

#[component]
pub fn AddTask(#[prop(into)] on_add: Callback<String>) -> impl IntoView {
    let task_title = RwSignal::new(String::new());

    let add_current_task = move || {
        let title = task_title.get();

        if title.trim().is_empty() {
            return;
        }

        on_add.run(title);
        task_title.set(String::new());
    };

    view! {
      <form class="flex gap-4 justify-between items-end w-full" on:submit=move |ev| {
          ev.prevent_default();
          add_current_task();
      }>
        <div class="flex flex-col gap-1.5 w-full">
          <p>"Task name"</p>

          <Input bind_value=task_title placeholder="Task name" />
        </div>

        <Button>
          "Add"
        </Button>
      </form>
    }
}
