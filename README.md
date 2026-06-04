# Starter Kit Example

This is a local directory illustrating the structure of a `starterRepo`.
It is NOT pushed anywhere automatically, but you can initialize a Git repository here and push it to GitHub/GitLab.

Then, you can use its `.git` URL (e.g. `https://github.com/YourUser/rust-starter.git`) as the `starterRepoUrl` in the admin panel for the Rust language configuration of a given challenge.

## Structure

The starter repository MUST follow this structure:

```
/ [languageSlug]     <-- e.g., "rust", "python", "go"
  / base             <-- (Optional) Base files copied into the worktree for all stages
    Cargo.toml
    src/main.rs
  / stage-1          <-- Files copied over the base specifically when starting Stage 1
    src/main.rs
  / stage-2          <-- Files copied over specifically when starting Stage 2
    src/main.rs
```

When a user starts `stage-1` (with `CLEAN_SLATE` transition mode), the system will:
1. Delete everything inside their repository (except `.git`).
2. Copy everything from `/rust/base/` into the root of their repository.
3. Copy everything from `/rust/stage-1/` into the root, overwriting anything from base.
4. Commit and push the result as the `stage-1-start` checkpoint.

If `INCREMENTAL` transition mode is used, the system will *not* delete the user's existing files, and will just copy `/rust/stage-N/` files over their existing work.
