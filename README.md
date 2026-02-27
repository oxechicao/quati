# Project Overview

> The South American coati (Nasua nasua), or ring-tailed coati, is a diurnal,
> raccoon-family mammal found in tropical/subtropical forests from Colombia to northern
> Argentina. Known for their long, flexible snouts and striped tails, they are highly
> social, omnivorous, and agile climbers that weigh 2–7.2 kg.
>
> ![quati](./quati.jpg)

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

> Currently, the project is defined to push using ssh key. User and Password will be
> defined in the future.

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

### Custom settings

#### Custom git host and ssh key

```env
QUATI_SSH_KEY_PATH=/path/to/my/ssh/key
# Example: QUATI_SSH_KEY_PATH=/Users/username/.ssh/custom_id_rsa
QUATI_CUSTOM_LOCAL_GIT_HOST=my.custom.git.host
QUATI_CUSTOM_REMOTE_GIT_HOST=my.custom.git.host
```

```

```

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

## The commit message uses the following gitmoji

| icon | tag                         | description                                                   |
| ---- | --------------------------- | ------------------------------------------------------------- |
| 🎨   | :art:                       | Improve structure / format of the code.                       |
| ⚡️   | :zap:                       | Improve performance.                                          |
| 🔥   | :fire:                      | Remove code or files.                                         |
| 🐛   | :bug:                       | Fix a bug.                                                    |
| 🚑️   | :ambulance:                 | Critical hotfix.                                              |
| ✨   | :sparkles:                  | Introduce new features.                                       |
| 📝   | :memo:                      | Add or update documentation.                                  |
| 🚀   | :rocket:                    | Deploy stuff.                                                 |
| 💄   | :lipstick:                  | Add or update the UI and style files.                         |
| 🎉   | :tada:                      | Begin a project.                                              |
| ✅   | :white_check_mark:          | Add, update, or pass tests.                                   |
| 🔒️   | :lock:                      | Fix security or privacy issues.                               |
| 🔐   | :closed_lock_with_key:      | Add or update secrets.                                        |
| 🔖   | :bookmark:                  | Release / Version tags.                                       |
| 🚨   | :rotating_light:            | Fix compiler / linter warnings.                               |
| 🚧   | :construction:              | Work in progress.                                             |
| 💚   | :green_heart:               | Fix CI Build.                                                 |
| ⬇️   | :arrow_down:                | Downgrade dependencies.                                       |
| ⬆️   | :arrow_up:                  | Upgrade dependencies.                                         |
| 📌   | :pushpin:                   | Pin dependencies to specific versions.                        |
| 👷   | :construction_worker:       | Add or update CI build system.                                |
| 📈   | :chart_with_upwards_trend:  | Add or update analytics or track code.                        |
| ♻️   | :recycle:                   | Refactor code.                                                |
| ➕   | :heavy_plus_sign:           | Add a dependency.                                             |
| ➖   | :heavy_minus_sign:          | Remove a dependency.                                          |
| 🔧   | :wrench:                    | Add or update configuration files.                            |
| 🔨   | :hammer:                    | Add or update development scripts.                            |
| 🌐   | :globe_with_meridians:      | Internationalization and localization.                        |
| ✏️   | :pencil2:                   | Fix typos.                                                    |
| 💩   | :poop:                      | Write bad code that needs to be improved.                     |
| ⏪️   | :rewind:                    | Revert changes.                                               |
| 🔀   | :twisted_rightwards_arrows: | Merge branches.                                               |
| 📦️   | :package:                   | Add or update compiled files or packages.                     |
| 👽️   | :alien:                     | Update code due to external API changes.                      |
| 🚚   | :truck:                     | Move or rename resources (e.g.: files, paths, routes).        |
| 📄   | :page_facing_up:            | Add or update license.                                        |
| 💥   | :boom:                      | Introduce breaking changes.                                   |
| 🍱   | :bento:                     | Add or update assets.                                         |
| ♿️   | :wheelchair:                | Improve accessibility.                                        |
| 💡   | :bulb:                      | Add or update comments in source code.                        |
| 🍻   | :beers:                     | Write code drunkenly.                                         |
| 💬   | :speech_balloon:            | Add or update text and literals.                              |
| 🗃️   | :card_file_box:             | Perform database related changes.                             |
| 🔊   | :loud_sound:                | Add or update logs.                                           |
| 🔇   | :mute:                      | Remove logs.                                                  |
| 👥   | :busts_in_silhouette:       | Add or update contributor(s).                                 |
| 🚸   | :children_crossing:         | Improve user experience / usability.                          |
| 🏗️   | :building_construction:     | Make architectural changes.                                   |
| 📱   | :iphone:                    | Work on responsive design.                                    |
| 🤡   | :clown_face:                | Mock things.                                                  |
| 🥚   | :egg:                       | Add or update an easter egg.                                  |
| 🙈   | :see_no_evil:               | Add or update a .gitignore file.                              |
| 📸   | :camera_flash:              | Add or update snapshots.                                      |
| ⚗️   | :alembic:                   | Perform experiments.                                          |
| 🔍️   | :mag:                       | Improve SEO.                                                  |
| 🏷️   | :label:                     | Add or update types.                                          |
| 🌱   | :seedling:                  | Add or update seed files.                                     |
| 🚩   | :triangular_flag_on_post:   | Add, update, or remove feature flags.                         |
| 🥅   | :goal_net:                  | Catch errors.                                                 |
| 💫   | :dizzy:                     | Add or update animations and transitions.                     |
| 🗑️   | :wastebasket:               | Deprecate code that needs to be cleaned up.                   |
| 🛂   | :passport_control:          | Work on code related to authorization, roles and permissions. |
| 🩹   | :adhesive_bandage:          | Simple fix for a non-critical issue.                          |
| 🧐   | :monocle_face:              | Data exploration/inspection.                                  |
| ⚰️   | :coffin:                    | Remove dead code.                                             |
| 🧪   | :test_tube:                 | Add a failing test.                                           |
| 👔   | :necktie:                   | Add or update business logic.                                 |
| 🩺   | :stethoscope:               | Add or update healthcheck.                                    |
| 🧱   | :bricks:                    | Infrastructure related changes.                               |
| 🧑‍💻   | :technologist:              | Improve developer experience.                                 |
| 💸   | :money_with_wings:          | Add sponsorships or money related infrastructure.             |
| 🧵   | :thread:                    | Add or update code related to multithreading or concurrency.  |
| 🦺   | :safety_vest:               | Add or update code related to validation.                     |
| ✈️   | :airplane:                  | Improve offline support.                                      |
| 🦖   | :t-rex:                     | Code that adds backwards compatibility.                       |
