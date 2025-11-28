# 1:TASK: Intialize CLI and create the helper

AS a developer
I WANT to see what I can do with the CLI
SO I want to run the cli and see the helper texts

## Description

The command structure is: `quati [OPTIONS]`

Table of the commands:

| Argument | Optional          | Descripion        | Example    |
| -------- | ----------------- | ----------------- | ---------- |
|          | `-V`, `--verions` | Print the version | `quati -V` |
|          | `-h, --help`      | Print the helper  | `quati -h` |

## Use cases

- GIVEN the user runs `quati -V` or `quati --version`,
    WHEN the user wants to know the installed Quati version,
    THEN print the tool version.
- GIVEN the user runs `quati -h` or `quati --help`,
    WHEN the user needs global CLI usage information,
    THEN show global help.
