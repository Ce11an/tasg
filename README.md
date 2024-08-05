# tasg

_Manage your tasks with `tasg`!_

`tasg` is a simple command-line task manager written in Rust. It allows you to add, list, complete, and delete tasks, helping you stay organized.

## Features

- **Add New Tasks**: Easily add tasks with descriptions.
- **List Tasks**: View all tasks or filter to see only incomplete tasks.
- **Complete Tasks**: Mark tasks as complete.
- **Delete Tasks**: Remove tasks when they are no longer needed.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone the repository:

   ```sh
   git clone https://github.com/Ce11an/tasg.git
   cd tasg
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

4. The compiled binary will be available in the `target/release` directory. To install it globally, use:

   ```sh
   cargo install --path .
   ```

## Uninstallation

To uninstall `tasg`, you can use Cargo to remove the installed binary:

```sh
cargo uninstall tasg
```

### Clean Up Configuration Files

After uninstalling, you may want to remove the configuration directory where `tasg` stores task data:

- **Linux**:

  ```sh
  rm -rf ~/.config/tasg
  ```

- **Windows**:

  ```cmd
  rd /s /q C:\Users\Annie\AppData\Roaming\tasg
  ```

- **macOS**:

  ```sh
  rm -rf /Users/Annie/Library/Application\ Support/tasg
  ```

Adjust the paths as necessary if your username or configuration directory differs.

## Usage

`tasg` provides a straightforward CLI interface. Below are the available commands:

### Add a Task

Add a new task with a description:

```sh
tasg add "Your task description"
```

### List Tasks

To list incomplete tasks:

```sh
tasg list
```

To list all tasks, including completed ones:

```sh
tasg list --all
```

### Complete a Task

Mark a task as complete by specifying its ID:

```sh
tasg complete <task_id>
```

### Delete a Task

Remove a task by specifying its ID:

```sh
tasg delete <task_id>
```

### Nuke All Tasks

To delete all tasks (irreversible action), use:

```sh
tasg nuke
```

You will be prompted to confirm this action.

## Invalid Commands

If you enter an invalid command or missing arguments, `tasg` will display an error message to guide you.

## Running Tests

`tasg` includes a comprehensive suite of tests. To run the tests, use:

```sh
cargo test
```

## Example

Here’s a step-by-step example of how to use `tasg`:

```sh
# Add a new task
tasg add "Write README"

# List all incomplete tasks
tasg list

# Complete the task with ID 1
tasg complete 1

# List all tasks, including completed ones
tasg list --all

# Delete the task with ID 1
tasg delete 1

# Verify the task has been deleted
tasg list
```

## Contributing

Contributions are welcome! Please fork the repository, make your changes, and submit a pull request. Ensure all tests pass before submitting your pull request.
