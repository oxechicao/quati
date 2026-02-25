use git2::{
    Cred, Diff, DiffFormat, DiffOptions, Error, IndexAddOption, Oid, PushOptions, RemoteCallbacks,
    Repository,
};
use std::{error::Error as StdError, path::Path};

use crate::{environments::get_ssh_key_path, logger::Logger};

pub fn get_repo(path: Option<&Path>) -> Repository {
    let target = path.unwrap_or_else(|| Path::new("./"));
    Repository::open(target).unwrap_or_else(|e| {
        eprintln!("Error opening repository: {}", e);
        std::process::exit(1);
    })
}

pub fn get_diff(repo: &Repository) -> Result<Diff<'_>, Error> {
    let mut opts = DiffOptions::new();
    opts.include_untracked(true);
    let repo_diff = repo.diff_index_to_workdir(None, Some(&mut opts))?;
    Ok(repo_diff)
}

pub fn git_add_all(repo: &Repository) -> Result<(), Box<dyn StdError>> {
    let mut index = repo.index()?;
    index.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

pub fn git_diff_as_string(repo: &Repository) -> Result<String, Error> {
    let diff = get_diff(repo)?;
    let mut buf = String::new();

    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        let origin = line.origin();
        if origin != '>' && origin != '<' && origin != 'F' && origin != 'H' {
            buf.push(origin);
        }

        let content = str::from_utf8(line.content()).unwrap_or_default();
        buf.push_str(content);
        // Return true to continue the iteration
        true
    })?;

    Ok(buf)
}

pub fn do_commit(repo: &Repository, message: &str) -> Result<Oid, git2::Error> {
    // 1. Preparar o Index (git add .)
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // 2. Gravar a Tree (snapshot do estado atual)
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // 3. Definir o Autor e Committer
    let signature = repo.signature()?; // Usa configs globais do git

    // 4. Obter o Commit Pai (se existir)
    let mut parents = Vec::new();
    if let Ok(head) = repo.head() {
        parents.push(head.peel_to_commit()?);
    }

    // 5. Criar o Commit
    let parents_refs: Vec<&git2::Commit> = parents.iter().collect();
    let commit_oid = repo.commit(
        Some("HEAD"),  // Atualiza o HEAD para este novo commit
        &signature,    // Autor
        &signature,    // Committer
        message,       // Mensagem
        &tree,         // Árvore de arquivos
        &parents_refs, // Lista de pais
    )?;

    Ok(commit_oid)
}

pub fn create_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let head = repo.head()?;
    let target = head
        .target()
        .ok_or_else(|| git2::Error::from_str("Failed to get target OID from HEAD"))?;

    // Criar a nova branch
    repo.branch(branch_name, &repo.find_commit(target)?, false)?;

    Ok(())
}

pub fn push_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    Logger.info(&format!(
        "Pushing branch '{}' to remote 'origin'",
        branch_name
    ));
    let mut remote = repo.find_remote("origin")?;
    let url = remote.url().unwrap_or("");

    // If the URL uses the alias, we manually point it to the right place
    if url.contains("git@ftgit:") {
        let real_url = url.replace("git@ftgit:", "git@github.com:"); // Use your real HostName here
        remote = repo.remote_anonymous(&real_url)?;
    }
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, user, _types| {
        let username = user.unwrap_or("git");
        let key_path = get_ssh_key_path();
        Cred::ssh_key(
            username, None,      // Public key (git2 can usually derive it)
            &key_path, // Private key from .env or default
            None,      // Passphrase
        )
    });
    let mut options = PushOptions::new();
    options.remote_callbacks(callbacks);

    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);

    remote.push(&[&refspec], Some(&mut options))?;
    Ok(())
}

pub fn get_current_branch_name(repo: &Repository) -> Result<String, git2::Error> {
    let head = repo.head()?;
    let branch_name = head
        .shorthand()
        .ok_or_else(|| git2::Error::from_str("Failed to get branch name from HEAD"))?;
    Ok(branch_name.to_string())
}
