use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::add_task::AddTask;

#[derive(Clone, Serialize, Deserialize)]
struct Task {
    name: String,
    is_completed: bool
}

fn load_tasks() -> Vec<Task> {
    let Some(storage) = window().local_storage().ok().flatten()
        else { return Vec::<Task>::new() };

    let tasks_json = storage.get_item("tasks").ok().flatten();

    match tasks_json {
        Some(tasks_json) => serde_json::from_str(&tasks_json).unwrap_or_default(),
        None => Vec::<Task>::new()
    }
}

fn save_tasks(tasks: &[Task]) {
    let Some(storage) = window().local_storage().ok().flatten()
        else { return; };

    let Ok(tasks_json) = serde_json::to_string(tasks)
        else { return; };

    if let Err(err) = storage.set_item("tasks", &tasks_json) {
        leptos::logging::error!("failed to save tasks: {:?}", err);
    }
}

#[component]
pub fn TasksView() -> impl IntoView {
    let tasks = RwSignal::new(load_tasks());

    Effect::new(move |_| {
        let current_tasks = tasks.get();

        save_tasks(&current_tasks);
    });

    let on_add = Callback::new(move |name: String| {
        tasks.update(|tasks|
            tasks.push(Task {
                name,
                is_completed: false,
            })
        )
    });

   view! {
     <section class="w-full max-w-96 flex flex-col gap-6">
       <AddTask on_add />

       <div>
         {move || {
             tasks.get().into_iter().map(|task|
                 view! {
                   <div class="py-2 px-3 bg-muted/20 rounded-lg">
                     <p>{task.name}</p>

                     <p>{if task.is_completed { "Completed" } else { "" }}</p>
                   </div>
                 }
             ).collect_view()
         }}
       </div>
     </section>
   }
}
