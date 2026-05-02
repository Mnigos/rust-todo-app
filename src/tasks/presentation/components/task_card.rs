use crate::{components::ui::checkbox::Checkbox, tasks::domain::Task};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn TaskCard(
    #[prop(into)] task: Task,
    #[prop(into)] on_completion_change: Callback<(Uuid, bool)>,
) -> impl IntoView {
    let title = task.title().value().to_string();
    let is_completed = task.is_completed();
    let task_id = task.id().value();

    view! {
      <div class="flex justify-between items-center py-2 px-3 rounded-lg bg-muted/20">
        <p>{title}</p>

        <Checkbox checked={is_completed} on_checked_change=move |is_checked| {
            on_completion_change.run((task_id, is_checked));
        } />
      </div>
    }
}
