<p align="center">
  <img src="https://img.shields.io/crates/l/tasg.svg" alt="license">
  <a href="https://crates.io/crates/tasg"><img src="https://img.shields.io/crates/v/tasg.svg?colorB=319e8c" alt="Version info"></a><br>
  <img src="https://github.com/Ce11an/tasg/actions/workflows/ci.yaml/badge.svg?branch=main" alt="CI status"><br>
    <i>Manage your tasks with tasg!</i>
</p>

## Features

- **Add New Tasks**: Easily add tasks with descriptions.
- **List Tasks**: View all tasks or filter to see only incomplete tasks.
- **Complete Tasks**: Mark tasks as complete.
- **Delete Tasks**: Remove tasks when they are no longer needed.
- **Edit Tasks**: Edit the descriptions of tasks.

## Installation

Install via Cargo:

```sh
cargo install tasg
```

## Uninstallation

First delete the `tasg` storage data (irreversible action):

```sh
tasg nuke
```

To uninstall `tasg`, you can use Cargo to remove the installed binary:

```sh
cargo uninstall tasg
```

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

### Edit a Task

Edit a task by specifying its ID and a description:

```sh
tasg edit <task_id> --description "Your edited description"
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
