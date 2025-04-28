# About

Example GitHub repository with default library Rust crate and GitHub Actions workflows

# Workflows

## `dev`

- Summary: Runs `cargo clippy`, `cargo test`, and `cargo build` on push to `dev` branch across
  Linux, macOS, and Windows and Rust stable, beta, and nightly
- File: [`dev.yml`](.github/workflows/dev.yml)

## `publish`

- Summary: Runs `cargo outdated`, `cargo audit`, `cargo clippy`, `cargo test`, `cargo build`,
  `cargo build --release`, and `cargo publish` on push to `main` branch
- File: [`publish.yml`](.github/workflows/publish.yml)
- See also:
    - [`qtfkwk/cargo-outdated`](https://github.com/qtfkwk/cargo-outdated)
    - [`qtfkwk/cargo-audit`](https://github.com/qtfkwk/cargo-audit)

## `release`

- Summary: Archives Linux, macOS, and Windows release binaries and creates a GitHub release on push
  of a version tag `[0-9]+.[0-9]+.[0-9]+`
- File: [`release.yml`](.github/workflows/release.yml)

# Usage

1. Prerequisites

    - Install Rust (<https://rustup.rs>)
    - Create GitHub account
    - Log into [`crates.io`] with your GitHub account

2. Ensure your repository has `dev` and `main` branches
   (or customize the workflows accordingly during step 3 below)

3. Create a [`crates.io`] API Token with the name of your crate, desired expiration, `publish-new`
   and `publish-update` scopes, pattern with your crate's name at
   <https://crates.io/settings/tokens>;
   then add the secret with the name `CRATESIO_TOKEN` GitHub Actions secret at
   <https://github.com/qtfkwk/github-actions-rust-template/settings/secrets/actions/new>
   (replace `qtfkwk/github-actions-rust-template` with `your-username/repository`)

4. Create a GitHub personal access token with the desired expiration, access to your repository,
   with Read and Write access to Contents at <https://github.com/settings/personal-access-tokens>;
   then add the secret with the name `RELEASE_TOKEN` GitHub Actions secret at
   <https://github.com/qtfkwk/github-actions-rust-template/settings/secrets/actions/new>
   (replace `qtfkwk/github-actions-rust-template` with `your-username/repository`)

5. Copy `.github/workflows/{dev,publish}.yml` from this project to your Rust project,
   modify if needed / as desired, then add and commit
    - `git checkout dev && git add ... && git commit -m ... && git push`

6. Edit your source code, tests, etc

7. Add and commit changes to `dev` branch and push
    - `git add ... && git commit -m '...' && git push`

8. Check the "Dev" workflow run
    - <https://github.com/qtfkwk/github-actions-rust-template/actions/workflows/dev.yml>
      (replace `qtfkwk/github-actions-rust-template` with `your-username/repository`)

9. Merge `dev` branch to `main` via `git checkout main && git merge dev`

10. Edit your `Cargo.toml` (minimally increment the version) and make any other release-related
    changes

    - **NOTE**: This repository doesn't actually publish to [`crates.io`] and does not have all the
      appropriate metadata fields necessary to do so
      ([ref](https://github.com/qtfkwk/github-actions-rust-template/actions/runs/14708310595/job/41273914888#step:11:25)).
      Ensure that your `Cargo.toml` has `description`, `license`, and any other desired fields so
      the `cargo publish` command succeeds.

11. Add and commit changes to `main` branch, tag the commit, and push
    - `git add ... && git commit -m '...' && git push && git tag -a ... -m ... && git push --tags`

12. Check the "Publish" workflow run
    - <https://github.com/qtfkwk/github-actions-rust-template/actions/workflows/publish.yml>
      (replace `qtfkwk/github-actions-rust-template` with `your-username/repository`)

13. Check the "Release" workflow run
    - <https://github.com/qtfkwk/github-actions-rust-template/actions/workflows/release.yml>
      (replace `qtfkwk/github-actions-rust-template` with `your-username/repository`)

[`crates.io`]: https://crates.io

