# Project Overview

## Versioning Guide

Versioning for this project follows these criteria:

| Major                              | Minor                     | Patch                         |
| ---------------------------------- | ------------------------- | ----------------------------- |
| Breaking changes or major releases | Completion of an **EPIC** | Merging a task, story, or fix |

---

## About the Tool

This is a command-line interface (CLI) tool built in **Rust** designed to help you work
with AI libraries to support your development workflow.

> **Note:** AI agent setup (e.g., GitHub Copilot, OpenAI Codex) must be completed before
> running the CLI. This means you need to open the agent, log in, and complete the initial
> configuration. Alternatively, you can write a custom `agent_command` to handle the setup
> automatically.

## Project Philosophy

You don’t strictly need this CLI to perform these tasks. You could achieve similar results
with shell script functions; for example, you can check `quati.sh` to see scripts that
perform basically the same actions. You could also simply ask your AI agent to write a
commit message or create a branch for you.

This tool was created as a use case to study **Rust** and explore how to build CLI
applications. While it may not be perfect, it **fits** my daily usage and helps streamline
my workflow.

## How to use it?

```sh
# check the help
quati -h

# Basic usage
quati <action> <options>
```

### Basic commands

| Command  | Description                                                                                                          |
| :------- | :------------------------------------------------------------------------------------------------------------------- |
| `start`  | Creates and switches to a new branch named `wip/{branch_name}`. If no name is provided, a default name is generated. |
| `save`   | Performs a local commit using AI-assisted message generation.                                                        |
| `update` | Performs an AI-assisted commit and automatically pushes the changes to the origin.                                   |

## How it works?

The workflow is straightforward. When you run `quati save`, the tool automatically:

1. **Stages** all current changes (`git add .`).
2. **Captures** the `git diff` of your changes.
3. **Processes** the diff through an AI prompt to generate a descriptive commit message.
4. **Commits** the changes using the AI-generated message.

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
