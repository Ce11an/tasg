# Tasg

Work in progress!

Tasg is a simple command-line task manager written in Rust. It allows you to add, list, complete, and delete tasks, helping you stay organised.

## Features

- Add new tasks with descriptions
- List all tasks or only incomplete tasks
- Mark tasks as complete
- Delete tasks

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/tasg.git
   cd tasg
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

4. The binary will be available in the `target/release` directory. You can also install it globally:

   ```sh
   cargo install --path .
   ```

## Usage

Tasg uses a simple CLI interface. Below are the available commands:

### Add a Task

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

```sh
tasg complete <task_id>
```

### Delete a Task

```sh
tasg delete <task_id>
```

### Invalid Command

If you enter an invalid command, Tasg will notify you with an error message.

## Running Tests

Tasg includes a suite of tests to ensure functionality. To run the tests, use:

```sh
cargo test
```

## Example

Here's a complete example demonstrating adding, listing, completing, and deleting a task:

```sh
# Add a task
tasg add "Write README"

# List tasks
tasg list

# Complete the task with ID 1
tasg complete 1

# List all tasks including completed ones
tasg list --all

# Delete the task with ID 1
tasg delete 1

# List tasks to verify deletion
tasg list
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes. Ensure all tests pass before submitting your PR.

