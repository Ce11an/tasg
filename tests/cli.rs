use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn prepare_cmd(temp_dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("tasg").unwrap();
    cmd.env("TASG_FILE", temp_dir.path().join("tasks.json").to_str().unwrap());
    cmd
}

fn setup() -> (Command, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let cmd = prepare_cmd(&temp_dir);
    (cmd, temp_dir)
}

#[test]
fn test_add_task() {
    let (mut cmd, _temp_dir) = setup();
    let assert = cmd.arg("add").arg("Test task").assert();
    assert.success();
}

#[test]
fn test_add_task_with_empty_description() {
    let (mut cmd, _temp_dir) = setup();
    let assert = cmd.arg("add").arg("").assert();
    assert
        .failure()
        .stderr(predicate::str::contains("Error: Invalid input - Description cannot be empty"));
}

#[test]
fn test_list_tasks() {
    let (mut cmd, temp_dir) = setup();
    // Add a task
    cmd.arg("add").arg("Test task").assert().success();
    // Create a new command instance to list tasks
    let mut cmd = prepare_cmd(&temp_dir);
    // List tasks
    let assert = cmd.arg("list").assert();
    assert.success().stdout(predicate::str::contains("Test task"));
}

#[test]
fn test_list_tasks_with_flags() {
    let (mut cmd, temp_dir) = setup();
    // Add a task
    cmd.arg("add").arg("Test task").assert().success();
    // Create a new command instance to list tasks without --all
    let mut cmd = prepare_cmd(&temp_dir);
    let assert_no_flag = cmd.arg("list").assert();
    assert_no_flag.success().stdout(predicate::str::contains("Test task"));
    // Create a new command instance to list tasks with --all
    let mut cmd = prepare_cmd(&temp_dir);
    let assert_with_flag = cmd.arg("list").arg("--all").assert();
    assert_with_flag.success().stdout(predicate::str::contains("Test task"));
}

#[test]
fn test_complete_task() {
    let (mut cmd, temp_dir) = setup();
    // Add a task
    cmd.arg("add").arg("Test task").assert().success();
    // Create a new command instance to complete the task
    let mut cmd = prepare_cmd(&temp_dir);
    // Complete the task
    cmd.arg("complete").arg("1").assert().success();
    // Create a new command instance to list all tasks
    let mut cmd = prepare_cmd(&temp_dir);
    // List all tasks to verify
    let assert = cmd.arg("list").arg("--all").assert();
    assert.success().stdout(predicate::str::contains("Yes"));
}

#[test]
fn test_complete_non_existent_task() {
    let (mut cmd, _temp_dir) = setup();
    let assert = cmd.arg("complete").arg("9999").assert();
    assert.failure().stderr(predicate::str::contains("Task with ID 9999 not found"));
}

#[test]
fn test_delete_task() {
    let (mut cmd, temp_dir) = setup();
    // Add a task
    cmd.arg("add").arg("Test task").assert().success();
    // Create a new command instance to delete the task
    let mut cmd = prepare_cmd(&temp_dir);
    // Delete the task
    cmd.arg("delete").arg("1").assert().success();
    // Create a new command instance to list tasks
    let mut cmd = prepare_cmd(&temp_dir);
    // List tasks to verify
    let assert = cmd.arg("list").assert();
    assert.success().stdout(predicate::str::contains("No tasks found"));
}

#[test]
fn test_delete_non_existent_task() {
    let (mut cmd, _temp_dir) = setup();
    let assert = cmd.arg("delete").arg("9999").assert();
    assert.failure().stderr(predicate::str::contains("Task with ID 9999 not found"));
}

#[test]
fn test_invalid_command() {
    let (mut cmd, _temp_dir) = setup();
    let assert = cmd.arg("invalid").assert();
    assert.failure().stderr(predicate::str::contains("error: unrecognized subcommand 'invalid'"));
}

#[test]
fn test_nuke_tasks_confirm() {
    let (mut cmd, temp_dir) = setup();
    // Add a task
    cmd.arg("add").arg("Test task").assert().success();
    // Create a new command instance to nuke tasks
    let mut cmd = prepare_cmd(&temp_dir);
    // Test nuke with confirmation
    let assert = cmd
        .arg("nuke")
        .write_stdin("y\n") // Simulate user input for confirmation
        .assert();
    assert.success().stdout(predicate::str::contains("All tasks have been deleted."));
    // Verify that the tasks file is deleted
    assert!(!temp_dir.path().join("tasks.json").exists());
}

#[test]
fn test_nuke_tasks_cancel() {
    let (mut cmd, temp_dir) = setup();
    // Add a task
    cmd.arg("add").arg("Test task").assert().success();
    // Create a new command instance to nuke tasks
    let mut cmd = prepare_cmd(&temp_dir);
    // Test nuke with cancellation
    let assert = cmd
        .arg("nuke")
        .write_stdin("n\n") // Simulate user input for cancellation
        .assert();
    assert.success().stdout(predicate::str::contains("Operation cancelled."));
    // Verify that the tasks file still exists
    assert!(temp_dir.path().join("tasks.json").exists());
    let mut cmd = prepare_cmd(&temp_dir);
    // Check if the task still exists
    let assert = cmd.arg("list").assert();
    assert.success().stdout(predicate::str::contains("Test task"));
}

#[test]
fn test_nuke_tasks_with_no_tasks() {
    let (_, temp_dir) = setup();
    // Create a new command instance to nuke tasks
    let mut cmd = prepare_cmd(&temp_dir);
    // Test nuke with no tasks
    let assert = cmd
        .arg("nuke")
        .write_stdin("y\n") // Simulate user input for confirmation
        .assert();
    assert.success().stdout(predicate::str::contains("All tasks have been deleted."));
    // Verify that the tasks file is deleted
    assert!(!temp_dir.path().join("tasks.json").exists());
}

#[test]
fn test_nuke_tasks_when_file_does_not_exist() {
    let temp_dir = TempDir::new().unwrap();
    let tasks_file_path = temp_dir.path().join("tasks.json");
    // Ensure the tasks file does not exist
    assert!(!tasks_file_path.exists());
    let mut cmd = prepare_cmd(&temp_dir);
    // Test nuke when file does not exist
    let assert = cmd
        .arg("nuke")
        .write_stdin("y\n") // Simulate user input for confirmation
        .assert();
    assert.success().stdout(predicate::str::contains("All tasks have been deleted."));
    // Verify that the tasks file is still not present
    assert!(!tasks_file_path.exists());
}

#[test]
fn test_nuke_tasks_with_various_confirmation_inputs() {
    let (mut cmd, temp_dir) = setup();
    // Add a task
    cmd.arg("add").arg("Test task").assert().success();

    let inputs = vec!["Y\n", "y\n", "n\n", "\n"];
    let expected_outputs = vec![
        "All tasks have been deleted.",
        "All tasks have been deleted.",
        "Operation cancelled.",
        "Operation cancelled.",
    ];

    for (input, expected_output) in inputs.into_iter().zip(expected_outputs.into_iter()) {
        let mut cmd = prepare_cmd(&temp_dir);
        let assert = cmd.arg("nuke").write_stdin(input).assert();

        assert.success().stdout(predicate::str::contains(expected_output));

        if expected_output == "All tasks have been deleted." {
            // Ensure the tasks file is deleted
            assert!(!temp_dir.path().join("tasks.json").exists());
        } else {
            // Ensure the tasks file still exists
            assert!(temp_dir.path().join("tasks.json").exists());
        }
    }
}

#[test]
fn test_special_characters_in_task_description() {
    let (mut cmd, _temp_dir) = setup();
    let special_description = "Test task with special characters !@#$%^&*()";
    let assert = cmd.arg("add").arg(special_description).assert();
    assert.success();
    // Verify the task was added
    let mut cmd = prepare_cmd(&_temp_dir);
    let assert = cmd.arg("list").assert();
    assert.success().stdout(predicate::str::contains(special_description));
}
