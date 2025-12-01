# 2:TASK: Create start command

AS a developer
I WANT to start a new branch
SO I want to run the command `quati start` and create a new branch locally and on origin

## Use Case

| id  | GIVEN                                                                                 | WHEN                                               | THEN                                                                                                                                                                                                                               |
| --- | ------------------------------------------------------------------------------------- | -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | a git repository with an `origin` remote configured and the user has push permissions | the developer runs `quati start` without arguments | a new branch named using the configured prefix and the current branch name (e.g. `wip/<current_branch>`) is created locally and pushed to origin with upstream set, and the CLI prints the created branch and remote tracking info |
| 2   | a git repository with an `origin` remote configured and the user has push permissions | the developer runs `quati start BRANCH_NAME`       | a new branch `BRANCH_NAME` is created locally and pushed to origin with upstream set, and the CLI prints the created branch and remote tracking info                                                                               |
| 3   | the target branch already exists locally                                              | the developer runs `quati start [BRANCH]`          | the CLI switches to the existing branch, attempts to sync with origin, and prints a clear message indicating the branch already existed and what actions were taken                                                                |
| 4   | there is no `origin` remote or the push fails (network/auth)                          | the developer runs any `quati start` variant       | the CLI reports a clear error explaining the problem and suggests remediation (e.g. add remote, check network, retry push, or run manual `git push -u origin <branch>`)                                                            |

## State Diagram

```mermaid
stateDiagram-v2
  state if_start <<choice>>
    [*] --> if_start
    if_start --> default_branch: create default branch
    default_branch --> [*]: quati start
    note left of default_branch
        Example:
        git checkout -B wip/main
        git push origin wip/main
    end note

    if_start --> default_prefix_branch_name: create with a custom branhc name using default prefix
    default_prefix_branch_name --> [*]: quati start my-branch
    note left of default_prefix_branch_name
        Example:
        git checkout -B wip/my-branch
        git push origin wip/my-branch
    end note

    if_start --> no_prefix_branch: create a branch name with a prefix defined
    no_prefix_branch --> [*]: quati start my-branch --no-prefix
    note right of no_prefix_branch
      Example:
          git checkout -B my-branch
          git push origin my-branch
    end note
```
