use git2::{
    AnnotatedCommit, BranchType, Direction, FetchOptions, IndexAddOption, Remote, RemoteCallbacks,
    Repository,
};
use std::io::{self, Write};

use std::path::Path;

struct GitRepoRemote {
    name: String,
    url: String,
}

pub struct GitRepo {
    repo: Repository,
    remote: Option<GitRepoRemote>,
    current_branch: Option<String>,
}

impl GitRepo {
    pub fn init(dir_path: Option<&str>, default_branch: Option<&str>) -> GitRepo {
        let repo_path = dir_path.unwrap_or(".");
        let repo = match Repository::init(repo_path) {
            Ok(repo) => repo,
            Err(err) => panic!(
                "Failed to initialise repo '{}'. Error: {:?}",
                &repo_path, err
            ),
        };

        if let Some(branch_name) = default_branch {
            let mut index = repo.index().unwrap();

            // Use "." as the pathspec to match all files in the current working directory and subdirectories
            let pathspec = [Path::new(".")];

            // Use IndexAddOption::All to stage all changes, including new, modified, and deleted files
            // The None callback is used here, but a custom callback could be used for filtering or logging
            index
                .add_all(["*"].iter(), IndexAddOption::all(), None)
                .unwrap();

            // The modified in-memory index needs to be flushed back to disk
            index.write().unwrap();

            let tree_oid = index.write_tree().unwrap();
            let tree = repo.find_tree(tree_oid).unwrap();

            let head = repo.head().unwrap();
            let oid = head.target().unwrap();
            let parent_commit = repo.find_commit(oid).unwrap(); // Unwrap is safe here as HEAD is valid
            let parents = vec![&parent_commit];

            let signature = repo.signature().unwrap();

            let new_commit_oid = repo
                .commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    "Initial commit",
                    &tree,
                    &parents,
                )
                .unwrap();

            let new_commit = repo.find_commit(new_commit_oid).unwrap();
            let _ = repo.branch(branch_name, &new_commit, false).unwrap();
            // let _ = repo
            //     .set_head(&format!("refs/heads/{}", branch_name))
            //     .unwrap();
        }

        GitRepo {
            repo,
            remote: None,
            current_branch: Some(default_branch.unwrap_or("main").to_string()),
        }
    }

    pub fn add(self, path: &str) -> Result<(), git2::Error> {
        let mut index = self.repo.index()?;

        // Stage the file with the specified path
        index.add_path(Path::new(path))?;

        // The modified in-memory index needs to be flushed back to disk
        index.write()?;

        Ok(())
    }

    pub fn set_remote(&mut self, name: &str, url: &str) {
        self.remote = Some(GitRepoRemote {
            name: String::from(name),
            url: String::from(url),
        });
    }

    pub fn ls_remote(self) -> Result<(), git2::Error> {
        if let Some(remote) = self.remote {
            // let remote = &args.arg_remote;
            let mut repo_remote = self
                .repo
                .find_remote(remote.name.as_str())
                .or_else(|_| self.repo.remote_anonymous(remote.url.as_str()))?;

            // Connect to the remote and call the printing function for each of the
            // remote references.
            let connection = repo_remote.connect_auth(Direction::Fetch, None, None)?;

            // Get the list of references on the remote and print out their name next to
            // what they point to.
            for head in connection.list()?.iter() {
                println!("{}\t{}", head.oid(), head.name());
            }

            Ok(())
        } else {
            panic!("No remote has been set. Please set a remote before proceeding.")
        }
    }

    pub fn clone(repo_url: String, repo_path: Option<String>) -> GitRepo {
        let clone_path = if let Some(repo_path) = repo_path {
            repo_path
        } else {
            ".".to_string()
        };

        let cloned = match Repository::clone(repo_url.as_str(), &clone_path) {
            Ok(repo) => repo,
            Err(err) => panic!("Failed to clone repo '{}'. Error: {:?}", &clone_path, err),
        };

        let current_branch = Some(
            cloned
                .branches(Some(BranchType::Remote))
                .unwrap()
                .nth(0)
                .unwrap()
                .unwrap()
                .0
                .name()
                .unwrap()
                .unwrap()
                .to_string(),
        );

        GitRepo {
            repo: cloned,
            remote: None,
            current_branch,
        }
    }

    pub fn set_branch() {}

    pub fn fetch(
        &self,
        refs: &[&str],
        remote: &mut Remote,
    ) -> Result<AnnotatedCommit<'_>, git2::Error> {
        let mut callback = RemoteCallbacks::new();

        callback.transfer_progress(|stats| {
            if stats.received_objects() == stats.total_objects() {
                println!(
                    "Resolving deltas {}/{}",
                    stats.indexed_deltas(),
                    stats.total_deltas()
                );
            } else if stats.total_objects() > 0 {
                println!(
                    "Received {}/{} objects ({}) in {} bytes",
                    stats.received_objects(),
                    stats.total_objects(),
                    stats.indexed_objects(),
                    stats.received_bytes()
                );
            }

            io::stdout().flush().unwrap();
            true
        });

        let mut fetch_options = FetchOptions::new();

        fetch_options.remote_callbacks(callback);

        // Always fetch all tags.
        // Perform a download and also update tips
        fetch_options.download_tags(git2::AutotagOption::All);

        println!(
            "Fetching tags from remote '{}' for repo",
            remote.name().unwrap()
        );

        remote.fetch(refs, Some(&mut fetch_options), None)?;

        // If there are local objects (we got a thin pack), then tell the user how many objects we saved from having to cross the network
        let stats = remote.stats();
        if stats.local_objects() > 0 {
            println!(
                "Received {}/{} objects in {} bytes (used {} local objects)",
                stats.indexed_objects(),
                stats.total_objects(),
                stats.received_bytes(),
                stats.local_objects()
            );
        } else {
            println!(
                "Received {}/{} objects ({}) in {} bytes",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }

        let fetch_head = &self.repo.find_reference("FETCH_HEAD")?;
        let annotated_commit = self.repo.reference_to_annotated_commit(fetch_head)?;

        Ok(annotated_commit)
    }

    pub fn fast_forward(
        &self,
        lb: &mut git2::Reference,
        rc: &git2::AnnotatedCommit,
    ) -> Result<(), git2::Error> {
        let name = match lb.name() {
            Some(s) => s.to_string(),
            None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
        };

        let msg = format!("Fast-forward: Setting {} to id: {}", name, rc.id());

        println!("{}", msg);

        lb.set_target(rc.id(), &msg)?;

        self.repo.set_head(&name)?;

        // For some reason the force is required to make the working directory actually get updated
        // I suspect we should be adding some logic to handle dirty working directory states
        // but this is just an example so maybe not.
        self.repo
            .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

        Ok(())
    }

    pub fn merge_into_local(
        &self,
        remote_branch: &str,
        fetch_commit: &git2::AnnotatedCommit,
    ) -> Result<(), git2::Error> {
        // 1. do a merge analysis
        let analysis = &self.repo.merge_analysis(&[fetch_commit])?;

        // 2. Do the appropriate merge
        if analysis.0.is_fast_forward() {
            println!("Doing a fast forward");
            // do a fast forward
            let refname = format!("refs/heads/{}", remote_branch);
            let reference = self.repo.find_reference(&refname);

            match reference {
                Ok(mut r) => {
                    GitRepo::fast_forward(self, &mut r, fetch_commit)?;
                }
                Err(_) => {
                    // The branch doesn't exist so just set the reference to the
                    // commit directly. Usually this is because you are pulling
                    // into an empty repository.
                    self.repo.reference(
                        &refname,
                        fetch_commit.id(),
                        true,
                        &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
                    )?;
                    self.repo.set_head(&refname)?;
                    self.repo.checkout_head(Some(
                        git2::build::CheckoutBuilder::default()
                            .allow_conflicts(true)
                            .conflict_style_merge(true)
                            .force(),
                    ))?;
                }
            };
        } else if analysis.0.is_normal() {
            // do a normal merge
            let head_commit = self
                .repo
                .reference_to_annotated_commit(&self.repo.head()?)?;
            normal_merge(&self.repo, &head_commit, fetch_commit)?;
        } else {
            println!("Nothing to do...");
        }

        Ok(())
    }

    pub fn pull(&self, branch_name: Option<&str>) -> Result<(), git2::Error> {
        let remote = &self.remote;
        let repo = &self.repo;
        if let Some(GitRepoRemote { name, .. }) = remote {
            let branch_name = branch_name.unwrap();
            let mut repo_remote = repo.find_remote(name.as_str())?;
            let fetch_commit = GitRepo::fetch(self, &[branch_name], &mut repo_remote)?;
            GitRepo::merge_into_local(self, branch_name, &fetch_commit)
        } else {
            panic!("No remote has been set. Please set a remote before proceeding.")
        }
    }

    pub fn push(&self) {}

    pub fn sync(&self) {
        GitRepo::pull(self, Some(self.current_branch.as_ref().unwrap().as_str()));
        GitRepo::push(self);
    }
}

fn normal_merge(
    repo: &Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        println!("Merge conflicts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    // now create the merge commit
    let msg = format!("Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;
    // Do our merge commit and set current branch head to that commit.
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;
    // Set working tree to match head.
    repo.checkout_head(None)?;
    Ok(())
}
