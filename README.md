# Task Tracker CLI Documentation

The Task Tracker CLI provides a simple interface to manage tasks with commands for adding, updating, deleting, and listing tasks. It also includes functionality to update task statuses (e.g., marking them as "in-progress" or "done"). Tasks are stored in a JSON file for persistence. It is built as part of the [Task Tracker Project](https://roadmap.sh/projects/task-tracker) from the roadmap.sh.

---

## Installation
1. Clone the repository containing the CLI.
2. Build the executable using `cargo build --release`:
   ```bash
   cargo build --release
   mv target/release/task-tracker-cli-rust task-cli
   ```
3. Run the executable using the commands described below.

---

## Commands

### 1. **`add`**
Adds a new task.

**Usage**:
```bash
./task-cli add <task_name>
```

**Example**:
```bash
./task-cli add "Buy groceries"
```

---

### 2. **`update`**
Updates the name of an existing task.

**Usage**:
```bash
./task-cli update <task_id> <new_task_name>
```

**Example**:
```bash
./task-cli update 1 "Buy groceries and cook dinner"
```

---

### 3. **`delete`**
Deletes a task by its ID.

**Usage**:
```bash
./task-cli delete <task_id>
```

**Example**:
```bash
./task-cli delete 2
```

---

### 4. **`mark-in-progress`**
Marks a task as "in-progress" by its ID.

**Usage**:
```bash
./task-cli mark-in-progress <task_id>
```

**Example**:
```bash
./task-cli mark-in-progress 3
```

---

### 5. **`mark-done`**
Marks a task as "done" by its ID.

**Usage**:
```bash
./task-cli mark-done <task_id>
```

**Example**:
```bash
./task-cli mark-done 3
```

---

### 6. **`list`**
Lists tasks based on their status or all tasks if no status is provided.

**Usage**:
```bash
./task-cli list [status]
```

**Status Options**:
- `todo`: List tasks with "To Do" status.
- `in-progress`: List tasks with "In Progress" status.
- `done`: List tasks with "Done" status.

**Examples**:
1. List all tasks:
   ```bash
   ./task-cli list
   ```
2. List tasks with "In Progress" status:
   ```bash
   ./task-cli list in-progress
   ```

---

## Error Messages
- If a required parameter is missing or invalid, the CLI provides a relevant error message and exits with a non-zero status.
- Example: `Error: Task ID must be a valid number.` when a non-integer ID is passed.

---

## Data Storage
- Tasks are stored in a JSON file located at `./data.json`.
- Ensure this file is accessible and writable by the CLI for proper functionality.