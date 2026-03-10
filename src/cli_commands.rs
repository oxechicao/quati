use crate::{
    cli_signature::{SaveArgs, StartArgs, UpdateArgs},
    environments::Env,
    git::{
        create_branch, do_commit, get_current_branch_name, get_repo, git_add_all,
        git_diff_as_string, push_branch,
    },
    logger::Logger,
    prompt::{CodexAgent, GeneratePromptArgs, generate_commit_message, generate_prompt},
};
use git2::Error;
use std::path::Path;

pub fn start(args: StartArgs, path: Option<&Path>) -> Result<(), Error> {
    Logger.info("Executing start");
    let prefix_branc = if args.no_prefix {
        String::new()
    } else {
        args.prefix.unwrap()
    };

    let branch_name = format!(
        "{}{}",
        prefix_branc,
        args.branch
            .unwrap_or_else(|| "feature/new-branch".to_string()),
    );

    Logger.info(&format!("Branch name: {}", branch_name));

    let repo = get_repo(path);
    let current_branch = get_current_branch_name(&repo).unwrap_or_else(|e| {
        Logger.warn(format!("Error getting current branch: {}", e).as_str());
        String::new()
    });

    if current_branch == branch_name {
        Logger.warn(&format!(
            "Already on branch '{}', skipping branch creation",
            branch_name
        ));
        return Ok(());
    }

    create_branch(&repo, &branch_name)?;

    if args.no_push {
        return Ok(());
    }

    push_branch(&repo, &branch_name)?;

    Ok(())
}

pub fn save(args: SaveArgs, path: Option<&Path>) -> Result<(), String> {
    Logger.info("Executing save");
    let repo = get_repo(path);
    if args.all || Env::get_stage_all() {
        Logger.info("Stagging all changes");
        let _ = git_add_all(&repo);
    }
    let diff = git_diff_as_string(&repo).map_err(|e| format!("Error generating diff: {}", e))?;
    Logger.info("Git diff generated successfully");
    let no_emojis_args = !args.emojis && !args.no_emojis;
    let prompt_message = generate_prompt(GeneratePromptArgs {
        context: args.context.as_deref().unwrap_or("").to_string(),
        scope: args.scope.as_deref().unwrap_or("").to_string(),
        diff,
        show_emojis: (!no_emojis_args && (args.emojis || !args.no_emojis))
            || (no_emojis_args && Env::get_gitmoji()),
    });
    let agent = CodexAgent;
    let message = generate_commit_message(&prompt_message, agent);
    Logger.info("Saving changes in commit");
    let _ = do_commit(&repo, &message);
    Logger.info("Done!");
    Ok(())
}
pub fn update(args: UpdateArgs, path: Option<&Path>) -> Result<(), String> {
    Logger.info("executing update");
    let save_args = SaveArgs {
        context: args.context,
        scope: args.scope,
        all: args.all,
        emojis: args.emojis,
        no_emojis: args.no_emojis,
    };
    save(save_args, path)?;
    let repo = get_repo(path);
    let branch_name = get_current_branch_name(&repo)
        .map_err(|e| format!("Error getting current branch name: {}", e))?;
    let result = push_branch(&repo, &branch_name);

    match result {
        Ok(_) => Logger.info(&format!("Branch '{}' pushed successfully", branch_name)),
        Err(e) => Logger.error(format!("Error pushing branch: {}", e).as_str()),
    }
    Ok(())
}
