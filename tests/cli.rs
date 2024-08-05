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

    assert.success().stdout(predicate::str::contains("Task added successfully"));
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
fn test_complete_task() {
    let (mut cmd, temp_dir) = setup();

    // Add a task
    cmd.arg("add").arg("Test task").assert().success();

    // Create a new command instance to complete the task
    let mut cmd = prepare_cmd(&temp_dir);

    // Complete the task
    cmd.arg("complete")
        .arg("1")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task marked as complete"));

    // Create a new command instance to list all tasks
    let mut cmd = prepare_cmd(&temp_dir);

    // List all tasks to verify
    let assert = cmd.arg("list").arg("--all").assert();

    assert.success().stdout(predicate::str::contains("Yes"));
}

#[test]
fn test_delete_task() {
    let (mut cmd, temp_dir) = setup();

    // Add a task
    cmd.arg("add").arg("Test task").assert().success();

    // Create a new command instance to delete the task
    let mut cmd = prepare_cmd(&temp_dir);

    // Delete the task
    cmd.arg("delete")
        .arg("1")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task deleted successfully"));

    // Create a new command instance to list tasks
    let mut cmd = prepare_cmd(&temp_dir);

    // List tasks to verify
    let assert = cmd.arg("list").assert();

    assert.success().stdout(predicate::str::contains("No tasks found"));
}

#[test]
fn test_invalid_command() {
    let (mut cmd, _temp_dir) = setup();

    let assert = cmd.arg("invalid").assert();

    assert.failure().stderr(predicate::str::contains("error: unrecognized subcommand 'invalid'"));
}
