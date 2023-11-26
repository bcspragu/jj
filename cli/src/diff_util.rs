use jj_lib::repo::Repo;
fn diff_content(path: &RepoPath, value: MaterializedTreeValue) -> Result<Vec<u8>, CommandError> {
    match value {
fn basic_diff_file_type(value: &MaterializedTreeValue) -> &'static str {
    match value {
        MaterializedTreeValue::Absent => {
        MaterializedTreeValue::File { executable, .. } => {
                "executable file"
                "regular file"
        MaterializedTreeValue::Symlink { .. } => "symlink",
        MaterializedTreeValue::Tree(_) => "tree",
        MaterializedTreeValue::GitSubmodule(_) => "Git submodule",
        MaterializedTreeValue::Conflict { .. } => "conflict",
    let store = workspace_command.repo().store();
            let left_value = materialize_tree_value(store, &path, left_value).block_on()?;
            let right_value = materialize_tree_value(store, &path, right_value).block_on()?;
                let right_content = diff_content(&path, right_value)?;
                let description = match (&left_value, &right_value) {
                        MaterializedTreeValue::File {
                        },
                        MaterializedTreeValue::File {
                        },
                    (
                        MaterializedTreeValue::Conflict { .. },
                        MaterializedTreeValue::Conflict { .. },
                    ) => "Modified conflict in".to_string(),
                    (MaterializedTreeValue::Conflict { .. }, _) => {
                        "Resolved conflict in".to_string()
                    }
                    (_, MaterializedTreeValue::Conflict { .. }) => {
                        "Created conflict in".to_string()
                    (
                        MaterializedTreeValue::Symlink { .. },
                        MaterializedTreeValue::Symlink { .. },
                    ) => "Symlink target changed at".to_string(),
                    (_, _) => {
                let left_content = diff_content(&path, left_value)?;
                let right_content = diff_content(&path, right_value)?;
                let left_content = diff_content(&path, left_value)?;
    value: MaterializedTreeValue,
    match value {
    let store = workspace_command.repo().store();
            let path_string = path.as_internal_file_string();
            let left_value = materialize_tree_value(store, &path, left_value).block_on()?;
            let right_value = materialize_tree_value(store, &path, right_value).block_on()?;
                let right_part = git_diff_part(&path, right_value)?;
                let left_part = git_diff_part(&path, left_value)?;
                let right_part = git_diff_part(&path, right_value)?;
                let left_part = git_diff_part(&path, left_value)?;
    let store = workspace_command.repo().store();
            let left = materialize_tree_value(store, &repo_path, left).block_on()?;
            let right = materialize_tree_value(store, &repo_path, right).block_on()?;
            let left_content = diff_content(&repo_path, left)?;
            let right_content = diff_content(&repo_path, right)?;