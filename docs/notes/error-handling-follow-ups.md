# Error Handling Follow-Ups

## Task repository error display

Current state:

- `TaskRepositoryError` uses `#[derive(Debug)]`.
- Repository methods return `Result<_, TaskRepositoryError>`.
- Errors can be logged with debug formatting, for example `{:?}`.

Deferred improvement:

- Add user-friendly error messages for `TaskRepositoryError`.
- Prefer using the `thiserror` crate when the error type starts being passed through the application/service/server-function layers.

Possible future shape:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskRepositoryError {
    #[error("failed to read tasks file")]
    ReadFailed(#[source] io::Error),

    #[error("failed to write tasks file")]
    WriteFailed(#[source] io::Error),

    #[error("failed to serialize tasks")]
    SerializeFailed(#[source] serde_json::Error),

    #[error("failed to deserialize tasks")]
    DeserializeFailed(#[source] serde_json::Error),
}
```

Why later:

- Manual `Display` implementations are noisy while the app is still small.
- `Debug` is enough while the repository is still being introduced.
- `thiserror` will be more useful once errors need to cross module boundaries cleanly.

## Server function error responses

Current state:

- Server functions convert application errors with debug formatting.
- This is useful while learning because the browser console shows the concrete error shape.
- The client receives messages derived from internal error variants.

Deferred improvement:

- Avoid exposing raw internal errors through `ServerFnError`.
- Map domain/application/infrastructure failures into user-facing messages.
- Keep detailed error information in server logs.

Possible future shape:

```rust
service
    .add_task(title)
    .map_err(|err| {
        leptos::logging::error!("failed to add task: {:?}", err);
        ServerFnError::new("failed to add task")
    })
```

Why later:

- Debug strings are enough while the presentation layer is still being introduced.
- User-friendly error boundaries and UI messages will make this more valuable.
- The exact mapping should become clearer once the app has more operations.
