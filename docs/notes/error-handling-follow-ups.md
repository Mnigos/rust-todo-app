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
