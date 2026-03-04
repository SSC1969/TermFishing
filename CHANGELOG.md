# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/SSC1969/Mountain-Madness-2026/releases/tag/v0.1.0) - 2026-03-04

### Other

- Added configuration to avoid publishing crate
- Updated release workflow
- Configure GITHUB_TOKEN for release-plz action
- Add GitHub Actions workflow for Release-plz
- Added machete to the toolchain, and used it to find and remove unused dependencies (cargo machete --with-metadata)
- Moved chat methods into seperate handler to run on it's own thread more easily, encapsulated all player state management into player.rs, replaced rushed code with marginally better code
- Started bugfixing
- Merge branch 'main' into collection
- Updated items to use an enum-based implementation, and added proper functionality to the collection
- Started completion of collection/dex
- Added navigation to the backpack menu
- Updated inventory UI
- Fixed warnings
- Merged
- Updated menu UI
- First draft inventory UI
- implement get_all() for backpack and dex, implement dex
- add fish generation
- add species, including file for all definitions
- derive default
- Updated UI
- add missing module
- Merge branch 'main' into inventory
- quick generate, catch tests
- backpack search, add, remove implemented. player can catch_fish
- Add basic structs + inventory
- Added extra files to structure
- Added skeleton file structure
- Created file skeleton
- Fixed formatting you're welcome Adam <3, again
- Fixed formatting you're welcome Adam <3
- Networking test
