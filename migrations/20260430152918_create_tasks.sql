CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    is_completed BOOLEAN NOT NULL DEFAULT FALSE
);
