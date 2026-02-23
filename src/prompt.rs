use crate::{
    command_runner::{CommandRunner, RealCommandRunner},
    logger::Logger,
};

const DEFAULT_PROMPT: &str = "Read the file @temp_git_changes.diff to understand the code changes.
Read the file <CONTEXT_FILE> to get more context about the feature.

No Comentaries, only the commit message.

DO:
Write a commit message that follows the conventional commit format.
The subject should be limited in 50 characters, and the body should be limited in 72 characters.
The subject in the first line should be a concise summary.
The scope that should be used in the subject is <SCOPE>.
Do a summary of changes before the sections.
Write the changes in sections.
Write a detailed list of changes for each section, with bullet points. Use hiphens.
Add one line before each section title.
Output only the commit message.

DO NOT:
Do not use markdown syntax.
No add blank lines after section title.
Do not add any comentaries or explanations
";

pub fn generate_commit_message(context: &str, scope: &str) -> Result<String, String> {
    let replace_scope = if scope.is_empty() {
        DEFAULT_PROMPT.replace("<SCOPE>", "no-card")
    } else {
        DEFAULT_PROMPT.replace("<SCOPE>", scope)
    };

    let replace_context = if context.is_empty() {
        replace_scope.replace("<CONTEXT_FILE>", "that contains the diff")
    } else {
        replace_scope.replace("<CONTEXT_FILE>", context)
    };

    let mut runner = RealCommandRunner;
    let response = runner
        .run("codex", &["exec", &replace_context])
        .map_err(|e| format!("Failed: {}", e))?;

    if response.success {
        let result = String::from_utf8_lossy(&response.stdout).to_string();
        Logger.info(&format!("Generated commit message:\n{}", result));
        return Ok(result);
    }

    let err = String::from_utf8_lossy(&response.stderr).to_string();
    Logger.warn(&format!("Error generating commit message: {}", err));
    Ok(String::new())
}
