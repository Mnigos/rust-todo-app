use crate::{
    components::{add_task::AddTask, task_card::TaskCard},
    models::Task,
};
use leptos::prelude::*;

#[cfg(feature = "hydrate")]
fn load_tasks() -> Vec<Task> {
    let Some(storage) = window().local_storage().ok().flatten() else {
        return Vec::<Task>::new();
    };

    let tasks_json = storage.get_item("tasks").ok().flatten();

    match tasks_json {
        Some(tasks_json) => serde_json::from_str(&tasks_json).unwrap_or_default(),
        None => Vec::<Task>::new(),
    }
}

#[cfg(not(feature = "hydrate"))]
fn load_tasks() -> Vec<Task> {
    vec![]
}

#[cfg(feature = "hydrate")]
fn save_tasks(tasks: &[Task]) {
    let Some(storage) = window().local_storage().ok().flatten() else {
        return;
    };

    let Ok(tasks_json) = serde_json::to_string(tasks) else {
        return;
    };

    if let Err(err) = storage.set_item("tasks", &tasks_json) {
        leptos::logging::error!("failed to save tasks: {:?}", err);
    }
}

#[cfg(not(feature = "hydrate"))]
fn save_tasks(_tasks: &[Task]) {}

#[component]
pub fn TasksView() -> impl IntoView {
    let tasks = RwSignal::new(load_tasks());

    Effect::new(move |_| {
        let current_tasks = tasks.get();

        save_tasks(&current_tasks);
    });

    let on_add = move |name: String| {
        tasks.update(|tasks| {
            tasks.push(Task {
                id: u64::try_from(tasks.len()).expect("Failed to parse tasks len") + 1,
                name,
                is_completed: false,
            })
        })
    };

    let on_complete = move |task_id: u64| {
        tasks.update(|tasks| {
            if let Some(task) = tasks.iter_mut().find(|task| task.id == task_id) {
                task.is_completed = true;
            }
        });
    };

    view! {
      <section class="flex flex-col gap-6 w-full max-w-96">
        <AddTask on_add />

        <div class="flex flex-col gap-1">
          {move || {
              tasks.get().into_iter().map(|task|
                  view! {
                    <TaskCard task on_complete />
                  }
              ).collect_view()
          }}
        </div>
      </section>
    }
}
