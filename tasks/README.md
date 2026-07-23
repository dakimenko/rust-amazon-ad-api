# Tasks

## Convention

- **Active tasks** live directly in `tasks/`
- **Completed tasks** are moved to `tasks/done_tasks/` after execution
- Task files use the naming convention: `YYYY-MM-DD-description.md`
- Each task file starts with metadata: **Status**, **Priority**, **Depends on**

## Lifecycle

```
tasks/<task>.md          → pending / in_progress
tasks/done_tasks/<task>.md → completed
```
