# Task ID Follow-Ups

Current state:

- `TaskId` wraps a `u64`.
- Task IDs are generated from the current JSON-backed task collection.
- APIs can borrow `&TaskId` instead of assuming IDs are cheap `Copy` values.

Deferred improvement:

- When introducing database persistence, consider changing `TaskId` to a UUID-backed value object.
- At that point, ID generation should likely move away from `max(existing_id) + 1`.

Possible future shape:

```rust
pub struct TaskId(uuid::Uuid);
```

or:

```rust
pub struct TaskId(String);
```

Why later:

- A numeric ID is enough while learning the repository/service flow with JSON persistence.
- UUID generation is easier to justify once tasks are created through a server/database boundary.
- Avoiding `Copy` now keeps the code closer to the future ownership model.
