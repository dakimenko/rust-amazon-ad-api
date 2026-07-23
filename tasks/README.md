# Tasks Management Guide

This directory manages all development tasks and tracking files for human developers and AI agents working on `rust-amazon-ad-api`.

## Structure

```
tasks/
├── README.md                  # Task management conventions (this file)
├── TEMPLATE.md                # Template for creating new task files
├── YYYY-MM-DD-task-name.md    # Active / Pending task files
└── done_tasks/
    └── YYYY-MM-DD-task-name.md# Completed task files
```

## Conventions & Rules

1. **File Naming**: Task files must follow `YYYY-MM-DD-description.md` (e.g. `2026-07-23-upstream-sync.md`).
2. **Metadata Header**: All task files must start with the metadata block from [`TEMPLATE.md`](file:///mnt/i/AB_proj/amazon-ad-api/rust-amazon-ad-api/tasks/TEMPLATE.md):
   ```markdown
   # Task Title

   **Date:** YYYY-MM-DD
   **Status:** Pending | In Progress | Completed
   **Priority:** High | Medium | Low
   **Depends on:** None | <filename>
   ```
3. **Lifecycle**:
   - Create task in `tasks/<task>.md` (Status: Pending or In Progress).
   - Perform implementation and verify via `cargo test --all-features` and `cargo clippy --all-features -- -D warnings`.
   - Once verified, set **Status** to `Completed` and move to `tasks/done_tasks/<task>.md`.

For full agent instructions and code standards, refer to [`AGENTS.md`](file:///mnt/i/AB_proj/amazon-ad-api/rust-amazon-ad-api/AGENTS.md).
