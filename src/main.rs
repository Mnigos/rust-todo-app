#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::{config::get_configuration, context::provide_context};
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use rust_todo_app::{app::*, config::AppConfig};

    dotenvy::dotenv().ok();

    let app_config = AppConfig::from_env().unwrap();
    let db_pool = sqlx::PgPool::connect(app_config.database_url())
        .await
        .unwrap();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let db_pool = db_pool.clone();

                move || {
                    provide_context(db_pool.clone());
                }
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    println!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
