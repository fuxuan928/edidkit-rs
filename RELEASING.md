# Releasing

This project publishes to `crates.io` from GitHub Actions when a `v*` tag is pushed.

## Before Releasing

1. Update the crate version in `Cargo.toml`.
2. Review `README.md` if the release changes public APIs or examples.
3. Run the local checks:

```bash
cargo fmt --check
cargo test --all-targets
cargo package
cargo publish --dry-run
```

## Release Steps

1. Commit the release changes.
2. Push the branch to GitHub.
3. Create and push a version tag that matches `Cargo.toml`:

```bash
git tag v0.1.0
git push origin master
git push origin v0.1.0
```

4. Wait for the `Publish` GitHub Actions workflow to finish.

## After Releasing

1. Confirm the new version appears on `https://crates.io/crates/edidkit`.
2. Confirm documentation appears on `https://docs.rs/crate/edidkit/latest`.

`docs.rs` builds automatically after a crate is published. It may take a while if the build queue is busy.

If docs do not appear after the crate is visible on `crates.io`, use the `Rebuild docs` action from the crate page on `crates.io`.

## GitHub Secrets

The publish workflow requires this repository secret:

- `CRATES_IO_TOKEN`
