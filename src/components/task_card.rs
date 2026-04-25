use crate::{components::ui::checkbox::Checkbox, models::Task};
use leptos::prelude::*;

#[component]
pub fn TaskCard(
    #[prop(into)] task: Task,
    #[prop(into)] on_complete: Callback<u64>,
) -> impl IntoView {
    view! {
      <div class="flex justify-between items-center py-2 px-3 rounded-lg bg-muted/20">
        <p>{task.name}</p>

        <Checkbox checked={task.is_completed} on_checked_change=move |checked| {
            if checked { on_complete.run(task.id) }
        } />
      </div>
    }
}
