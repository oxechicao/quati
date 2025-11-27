# 2:TASK: Create start command

AS a developer
I WANT to start a new branch
SO I want to run the command `quati save` and create a new branch local and origin

## Use Case

GIVEN a git repository with an `origin` remote configured and the user has push permissions  
WHEN the developer runs `quati start` without arguments  
THEN a new branch named using the configured prefix and the current branch name (e.g. `wip/<current_branch>`) is created locally and pushed to origin with upstream set, and the CLI prints the created branch and remote tracking info.

GIVEN a git repository with an `origin` remote configured and the user has push permissions  
WHEN the developer runs `quati start BRANCH_NAME`  
THEN a new branch `BRANCH_NAME` is created locally and pushed to origin with upstream set, and the CLI prints the created branch and remote tracking info.

GIVEN a git repository with an `origin` remote configured and the user has push permissions  
WHEN the developer runs `quati start --wip 123`  
THEN a new branch prefixed with the configured prefix (e.g. `wip/123`) is created locally and pushed to origin with upstream set, and the CLI prints the created branch and remote tracking info.

GIVEN the target branch already exists locally  
WHEN the developer runs `quati start [BRANCH]`  
THEN the CLI switches to the existing branch, attempts to sync with origin, and prints a clear message indicating the branch already existed and what actions were taken.

GIVEN there is no `origin` remote or the push fails (network/auth)  
WHEN the developer runs any `quati start` variant  
THEN the CLI reports a clear error explaining the problem and suggests remediation (e.g. add remote, check network, retry push, or run manual `git push -u origin <branch>`).

Acceptance criteria:
- `quati start` creates and pushes `wip/<current_branch>` when preconditions met.
- `quati start BRANCH_NAME` creates and pushes the specified branch.
- `quati start --wip 123` creates and pushes `wip/123`.
- Existing-branch, no-origin, and push-failure cases produce clear, actionable messages.