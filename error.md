>task.json

```json
    {
        "version": "2.0.0",
        "tasks": [
            {
                "label": "cargo build",
                "type": "shell",
                "command": "cargo",
                "args": [
                    "build"
                ],
                "group": {
                    "kind": "build",
                    "isDefault": true
                },
                "problemMatcher": [
                    "$rustc"
                ]
            }
        ]
    }
```
    
   > launch.json
```json
{
        "version": "0.2.0",
        "configurations": [
            {
                "type": "lldb",
                "request": "launch",
                "name": "Debug with Args",
                "program": "${workspaceRoot}/target/debug/${workspaceRootFolderName}",
                "args": "${input:args}",
                "cwd": "${workspaceRoot}",
                "preLaunchTask": "cargo build",
                "sourceLanguages": [
                    "rust"
                ]
            },
            // Your existing configuration
        ],
        "inputs": [
            {
                "id": "args",
                "type": "promptString",
                "description": "Enter command line arguments (space-separated)"
            }
        ]
    }
```

> What input is expected

```bash
Usage: termoil <COMMAND>

Commands:
  config  
  search  Search for a command
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

> When I currently enter I get this

```bash
error: unrecognized subcommand 'search "convert mov to mp4"'

  tip: a similar subcommand exists: 'search'
  tip: to pass 'search "convert mov to mp4"' as a value, use 'termoil -- search "convert mov to mp4"'

Usage: termoil <COMMAND>

For more information, try '--help'.
```