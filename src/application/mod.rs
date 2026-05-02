//! Application layer: usecases + view-models. No filesystem / SQLite /
//! agent-specific parsing here — go through `ports/` (ARCHITECTURE.md §5).

pub mod usecases;
pub mod view_models;
