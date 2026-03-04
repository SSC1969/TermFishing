# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/SSC1969/TermFishing/compare/v0.1.1...v0.1.2) - 2026-03-04

### Fixed

- actually properly fixed token generating step in release workflow
- properly added workflow token generating step to release

### Other

- Downgrade version from 0.1.3 to 0.1.2
- release v0.1.3
- Rename binary from 'my-bin' to 'TermFishing'
- release v0.1.2

## [0.1.3](https://github.com/SSC1969/TermFishing/compare/v0.1.2...v0.1.3) - 2026-03-04

### Other

- Rename binary from 'my-bin' to 'TermFishing'

## [0.1.2](https://github.com/SSC1969/TermFishing/compare/v0.1.1...v0.1.2) - 2026-03-04

### Fixed

- actually properly fixed token generating step in release workflow
- properly added workflow token generating step to release

## [0.1.1](https://github.com/SSC1969/TermFishing/compare/v0.1.0...v0.1.1) - 2026-03-04

### Other

- Fix id-token permission in release workflow
- Reset version to 0.1.0
- added app to allow generating secret keys so cd workflow can be triggered by release-plz
- added cd workflow to run after release-plz published a release
- added (properly working) release-plz workflow

## [0.9.0](https://github.com/SSC1969/Mountain-Madness-2026/releases/tag/v0.9.0) - 2026-03-04

### Other

- Updated package metadata
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