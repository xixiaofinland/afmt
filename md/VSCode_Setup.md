# Config afmt in VSCode

VSCode supports running tasks via `tasks.json`, which allows running shell commands on files.

To configure VSCode to run `afmt` against the currently opened file, follow these steps:

## Define a Custom Task

1. Open the Command Palette (Ctrl+Shift+P or Cmd+Shift+P on Mac).
1. Search for and select "Tasks: Configure Task".
1. Choose "Create tasks.json file from template".
1. Select "Others".

## Add the Task Configuration

In the tasks.json file, use this content below.
Make sure the "command" section points to your `afmt` binary.

```
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Run afmt on current file",
            "type": "shell",
            "command": "~/afmt -w ${file}", // assume afmt binary is on the ~ path
            "group": {
                "kind": "build",
                "isDefault": true
            },
            "presentation": {
                "echo": true,           // Echo the command
                "reveal": "never",      // Do not show the terminal
                "focus": false,         // Do not focus on the terminal
                "panel": "dedicated",   // Use a dedicated terminal (optional)
                "clear": false          // Do not clear the terminal before execution
            },
            "background": true,         // Marks the task as running in the background
            "problemMatcher": [],
            "detail": "Runs afmt against the currently opened file"
        }
    ]
}
```

## Run the Task

1. Open the Command Palette (Ctrl+Shift+P or Cmd+Shift+P on Mac).
2. Search for "Tasks: Run Task".
3. Select "Run afmt on current file" (i.e. the name of the custom task you defined above).
4. You should see that `afmt` formats the Apex file.
5. If nothing happens, open a terminal and run the same to diagonize, such as run: `> ~/afmt -w path/to/valid_apex_file.cls`

## Assign a Keybinding (Optional)

If you want to quickly trigger the task with a shortcut:

1. Open the Command Palette and search for "Preferences: Open Keyboard Shortcuts".
2. Search for `workbench.action.tasks.runTask`.
3. Add a custom keybinding in `keybindings.json`:

```
{
    "key": "ctrl+alt+r", // Choose your preferred shortcut
    "command": "workbench.action.tasks.runTask",
    "args": "Run afmt on current file"
}
```
