use crate::components::ui::{button::Button, input::Input};
use leptos::prelude::*;

#[component]
pub fn AddTask(#[prop(into)] on_add: Callback<String>) -> impl IntoView {
    let task_name = RwSignal::new(String::new());

    let add_current_task = move || {
        let name = task_name.get();

        if name.trim().is_empty() {
            return;
        }

        on_add.run(name);
        task_name.set(String::new());
    };

    view! {
      <form class="flex justify-between items-end gap-4 w-full" on:submit= move|ev| {
          ev.prevent_default();
          add_current_task();
      }>
        <div class="flex flex-col gap-1.5 w-full">
          <p>"Task name"</p>

          <Input bind_value=task_name placeholder="Task name" />
        </div>

        <Button>
          "Add"
        </Button>
      </form>
    }
}
