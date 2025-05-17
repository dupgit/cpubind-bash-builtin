[private]
default:
  @just --list

name := "cpubind_bash_builtin"

bump patch:
    # Checking that we do not have any untracked or uncommitted file
    git status -s | wc -l | grep '0'

    # Updating all dependencies
    cargo update

    # Bumping release version upon what has been asked on command line (major, minor or patch)
    cargo release version {{patch}} --no-confirm --execute

    # Building, testing and building doc to ensure one can build with these dependencies
    cargo build --release
    cargo test --release
    cargo doc --no-deps

    # Generetaing a Software Bills of Materials in SPDX∘format (sorting will reduce the diff size and allow one to figure out what has really changed)
    cargo sbom | jq --sort-keys | jq '.files = (.files| sort_by(.SPDXID))' | jq '.packages = (.packages| sort_by(.SPDXID))' | jq '.relationships = (.relationships| sort_by(.spdxElementId, .relatedSpdxElement))'>{{name}}.sbom.spdx.json

    # Creating the release
    git add Cargo.toml Cargo.lock {{name}}.sbom.spdx.json
    cargo release commit --no-confirm --execute
    cargo release tag --no-confirm --execute

document:
    cargo doc --no-deps --open

git-publish:
    git push
    git push --tags

rust-publish:
    cargo publish

publish: git-publish rust-publish
