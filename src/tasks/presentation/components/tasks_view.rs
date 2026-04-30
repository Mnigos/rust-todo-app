use super::{AddTask, TaskCard};
use crate::tasks::presentation::server_functions::{add_task, complete_task, list_tasks};
use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn TasksView() -> impl IntoView {
    let tasks = Resource::new(
        || (),
        |_| async move {
            match list_tasks().await {
                Ok(tasks) => tasks,
                Err(err) => {
                    leptos::logging::error!("failed to load tasks: {:?}", err);
                    Vec::new()
                }
            }
        },
    );

    let on_add = move |title: String| {
        spawn_local(async move {
            match add_task(title).await {
                Ok(_) => tasks.refetch(),
                Err(err) => leptos::logging::error!("failed to add task: {:?}", err),
            }
        });
    };

    let on_complete = move |task_id: u64| {
        spawn_local(async move {
            match complete_task(task_id).await {
                Ok(()) => tasks.refetch(),
                Err(err) => leptos::logging::error!("failed to complete task: {:?}", err),
            }
        });
    };

    view! {
      <section class="flex flex-col gap-6 w-full max-w-96">
        <AddTask on_add />

        <div class="flex flex-col gap-1">
            <Suspense fallback=move || view! { <p>"Loading..."</p>}>
              {move || {
                  tasks.get().unwrap_or_default().into_iter().map(|task|
                      view! {
                        <TaskCard task on_complete />
                      }
                  ).collect_view()
              }}
            </Suspense>
        </div>
      </section>
    }
}
