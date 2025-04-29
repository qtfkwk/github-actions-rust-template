# About

Example GitHub repository with default library & binary Rust crate and GitHub Actions workflows

# Workflows

## `check`

- Runs `cargo outdated` [^1] and `cargo audit` [^2] to report updated dependencies and security
  issues, respectively
- Runs on schedule: daily at 12:00 UTC
    - This workflow can also be triggered via `workflow_dispatch` which enables external manual or
      automated triggering via the [GitHub CLI]:
      `gh workflow -R qtfkwk/github-actions-rust-template run check.yml` [^3]
- [`check.yml`]

## `dev`

- Runs `cargo clippy`, `cargo test`, and `cargo build` across Linux, macOS, and Windows and Rust
  stable, beta, and nightly
- Runs on push to `dev` branch
- [`dev.yml`]

## `publish`

- Runs `cargo outdated` [^1], `cargo audit` [^2], `cargo clippy`, `cargo test`, `cargo build`,
  `cargo build --release`, and `cargo publish`
- Runs on push to `main` branch
- [`publish.yml`]

## `release`

- Archives Linux, macOS, and Windows release binaries and creates a GitHub release
- Runs on push of a version tag `[0-9]+.[0-9]+.[0-9]+`
- [`release.yml`]

# Usage

1. Prerequisites:

    - Create a GitHub account
    - Log into [crates.io] with your GitHub account

   Optional:

    - Install the [GitHub CLI] and login
    - Install Rust (<https://rustup.rs>) [^4]

2. Create a new repository at <https://github.com/new>, clone it, create a new Rust project
   skeleton, and/or ensure it has `dev` and `main` branches (or customize the workflows during step
   3 below)

    ```bash
    # Clone your repository
    git clone git@github.com:your-username/repository.git

    # Create a dev branch
    cd repository
    git checkout -b dev

    # Create a binary crate
    cargo init
    cargo run

    # Create a library crate
    cargo init --lib
    cargo test

    # Add changes
    git add .

    # Initial commit to dev branch
    git commit -m initial

    # Push changes
    git push --set-upstream origin dev

    # Create a main branch
    git checkout -b main

    # Merge changes from dev branch
    git merge dev
    # and commit the merge commit

    # Initial commit to main branch
    git commit -m 0.1.0 --allow-empty

    # Tag the commit
    git tag -a 0.1.0 -m 0.1.0

    # Push the commits and tag
    git push --set-upstream origin main
    git push --tags
    ```

3. Create a [crates.io] API Token with the name of your crate, desired expiration, `publish-new`
   and `publish-update` scopes, pattern with your crate's name at
   <https://crates.io/settings/tokens>

   Add the secret to your repository with the name `CRATESIO_TOKEN` at
   <https://github.com/qtfkwk/github-actions-rust-template/settings/secrets/actions/new> [^3]

4. Create a GitHub personal access token with the desired name, expiration, repository/ies, and
   **Read and Write** access to **Contents** at <https://github.com/settings/personal-access-tokens>

   Add the secret to your repository with the name `RELEASE_TOKEN` at
   <https://github.com/qtfkwk/github-actions-rust-template/settings/secrets/actions/new> [^3]

5. Copy the `.github` directory from this project to your Rust project, modify if needed / as
   desired, then add, commit, and push

    ```bash
    cd ..
    git clone git@github.com:qtfkwk/github-actions-rust-template.git
    cd -
    git checkout dev
    cp -R ../github-actions-rust-template/.github .
    # Edit `.github/workflows/*.yml` if needed
    # If your crate is a library only crate, you should probably `rm .github/workflows/release.yml`
    # since it doesn't have any binaries
    git add .github
    git commit -m 'feat(ci): add workflows'
    git push
    ```

6. Edit your source code, tests, etc

7. Add and commit changes to `dev` branch and push (repeat steps 6-7 as desired)

    ```bash
    git add .
    git commit -m 'feat: ...' # or 'fix: ...'
    git push
    ```

8. Check the "Dev" workflow run(s)
    - <https://github.com/qtfkwk/github-actions-rust-template/actions/workflows/dev.yml> [^3]

9. When you are ready to publish and release, merge changes from the `dev` branch to the `main`
   branch:

    ```bash
    git checkout main
    git merge dev
    ```

10. Edit your `Cargo.toml` (minimally increment the version according to [semver.org]) and make any
    other release-related changes [^5]

    Increment major version (1.0.0) if steps 6-7 included breaking changes, minor version (0.2.0) if
    non-breaking features, or patch version (0.1.1) if all fixes.

11. Add and commit changes to `main` branch [^6] and push

    ```bash
    git add .
    git commit -m $VERSION
    git push
    ```

12. Check the "Publish" workflow run
    - <https://github.com/qtfkwk/github-actions-rust-template/actions/workflows/publish.yml> [^3]

13. Tag the commit [^6] and push it

    ```bash
    git tag -a $VERSION -m $VERSION
    git push --tags
    ```

14. Check the "Release" workflow run
    - <https://github.com/qtfkwk/github-actions-rust-template/actions/workflows/release.yml> [^3]

15. Check the "Check" workflow run
    (daily after 12:00 UTC or after running
    `gh workflow -R qtfkwk/github-actions-rust-template run check.yml` [^3])
    - <https://github.com/qtfkwk/github-actions-rust-template/actions/workflows/check.yml> [^3]

[^1]: See also [`qtfkwk/cargo-outdated`].

[^2]: See also [`qtfkwk/cargo-audit`].

[^3]: Replace `qtfkwk/github-actions-rust-template` with `your-username/repository`

[^4]: These workflows make installing Rust locally an *optional dependency*, since they do
everything *technically* needed to develop in Rust, including testing, building/compiling,
publishing, and packaging, as well as some optional steps like checking for updated dependencies and
security issues.
There are still some specific gaps that should be considered, for instance, workflows generally
require a commit and a push first; you probably don't really want to do your entire development
cycle in public... but you *could*.

[^5]: This repository doesn't actually publish to [crates.io] and does not have all the appropriate
metadata fields necessary to do so
([ref](https://github.com/qtfkwk/github-actions-rust-template/actions/runs/14708310595/job/41273914888#step:11:25)).
Ensure that your `Cargo.toml` has `description`, `license`, and any other desired fields so the
`cargo publish` command succeeds.
See the [Before publishing a new crate] section in [The Cargo Book] for details.

[^6]: Replace `$VERSION` with the actual version

[crates.io]: https://crates.io/
[`qtfkwk/cargo-outdated`]: https://github.com/qtfkwk/cargo-outdated
[`qtfkwk/cargo-audit`]: https://github.com/qtfkwk/cargo-audit
[`check.yml`]: .github/workflows/check.yml
[`dev.yml`]: .github/workflows/dev.yml
[`publish.yml`]: .github/workflows/publish.yml
[`release.yml`]: .github/workflows/release.yml
[Before publishing a new crate]: https://doc.rust-lang.org/cargo/reference/publishing.html#before-publishing-a-new-crate
[The Cargo Book]: https://doc.rust-lang.org/cargo/
[GitHub CLI]: https://cli.github.com/
[semver.org]: https://semver.org/

