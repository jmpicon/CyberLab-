# Mission Definition Schema (v1)

Missions are defined in YAML. This format is designed to be easily extensible without changing the game core.

## Schema Specification

```yaml
id: "linux-fundamentals-01"
title: "The First Login"
description: "Welcome to the grid. Find the secret file hidden in the home directory."
category: "Linux Fundamentals"
difficulty: 1 # 1-10
reward:
  credits: 100
  rep: 10

environment:
  image: "cyberlab/base-linux:latest"
  network: "isolated-net"
  files:
    - path: "/home/user/top_secret.txt"
      content: "The password for the next level is 'terminal_master_2026'."
      owner: "root"
      permissions: "0600"

objectives:
  - id: "find-file"
    type: "file_exists"
    path: "/home/user/found_it.flag"
    description: "Create a file named 'found_it.flag' in the user directory."
  - id: "read-secret"
    type: "command_executed"
    command_pattern: "cat .*/top_secret.txt"
    description: "Read the content of the secret file."

validation:
  mode: "continuous" # continuous or on_submit
  poll_interval: 5 # seconds
```

## Supported Objective Types
1.  `file_exists`: Checks if a file exists at a given path.
2.  `file_content`: Checks if a file contains a specific string.
3.  `command_executed`: Checks if a command matching a regex was logged in the bash history.
4.  `port_open`: Checks if a specific port is listening inside the container.
5.  `process_running`: Checks for a specific process name.

## Directory Structure
Missions should be placed in:
`missions/<pack_name>/<mission_id>.yaml`
`missions/<pack_name>/assets/` (for any extra files/scripts needed)
