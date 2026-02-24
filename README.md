# Quati command-line

Versioning on this project mean:

| Major                   | Minor            | Patch                  |
| ----------------------- | ---------------- | ---------------------- |
| Break change or release | Finish an "EPIC" | Merge a task/story/fix |

This is a commandline line tool that help you to work with AI libraries to support
development.

It is build in rust.

> Note: The agent setup (e.g copilot, codex) need be done before run the cli. It means,
> that you need to open, do login, setup etc. Or write a custom agent_command that setup
> it.

You do not need this cli to do anything here. You can just write some shellscript
functions. You can check in quati.sh to see the shellscript functions that do basically
the same. Or just ask to your AI agent to write a commit message for you, or create a
branch, etc.

> It is a simple cli used to be a use case to study rust and how to create a cli in rust.
> It can be not perfect, but it feat my daily usage.

## How to use it?

```sh
# check the help
quati -h

# Basic usage
quati <action> <options>
```

There are few basic commands:

| Action   | Description                                                                                                                                        |
| -------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `start`  | Create a new branch with the name `wip/{branch_name}` and switch to it. The branch name is optional, if not provided will be created with the name |
| `save`   | Do only commit locally with AI assistance                                                                                                          |
| `update` | Do commit with AI assistance and push to origin                                                                                                    |

## How it works?

It is simple, when you run `quati save` it will stage all the changes, get the diff, run
the prompt with the diff asking for the message, return the message and do commit with the
message.

```text
+------+     +-------+     +----+
| user |     | quati |     | AI |
+---+--+     +---+---+     +--+-+
    |            |            |
    | quati save |            |
    +----------->|            |
    |            |            |
    |            | stage all changes
    |            +--+         |
    |            |  |         |
    |            |<-+         |
    |            |            |
    |            | get the diff
    |            +--+         |
    |            |  |         |
    |            |<-+         |
    |            |            |
    |            | run the prompt with the diff asking for the message
    |            +----------->|
    |            |            |
    |            | return the message
    |            |<-----------+
    |            |            |
    |            | do commit with the message
    |            +--+         |
    |            |  |         |
    |            |<-+         |
    |            |            |
```
