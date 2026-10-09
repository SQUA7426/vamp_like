# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/SQUA7426/vamp_like/compare/v0.3.5...v0.4.0) - 2026-10-09

### Other

- added a simple player-attack, exp and level system
- release v0.3.5

## [0.3.5](https://github.com/SQUA7426/vamp_like/compare/v0.3.4...v0.3.5) - 2026-10-08

### Changed

- changed miri's chronicle time test

### Fixed

- spawn_enemies()
- spawnpoint()
- make enemies spawn at given distance

### Other

- increased the version from v.0.3.4 -> v.0.3.5
- added PlayingDecayRate for the play-speed; set it to 2x for testing purpose
- added a very simple health and damage dealing system
- added player rotate with <wasd> and added a dummy-stick
- added character trait and shortend enemy spawn function

## [0.3.4](https://github.com/SQUA7426/vamp_like/compare/v0.3.3...v0.3.4) - 2026-10-06

### Fixed

- made enemy despawn near player and made 1 enemy test
- made enemy chasing player with constant speed and only spawn as many as max_enemy resource has as limit

### Removed

- removed github.ref from release-plz.yml

## [0.3.3](https://github.com/SQUA7426/vamp_like/compare/v0.3.2...v0.3.3) - 2026-10-06

### Fixed

- added install dependencies for miri and a dummy enemey->chase player thats half-functionally working

## [0.3.0](https://github.com/SQUA7426/vamp_like/compare/v0.2.0...v0.3.0) - 2026-10-05

### Fixed

- shortened the button_action inside src/components/menu.rs

## [0.2.0](https://github.com/SQUA7426/vamp_like/compare/v0.1.0...v0.2.0) - 2026-10-04

### Added

- added part1 of inventory ui

### Fixed

- added libudev-dev to release-plz.yml
- added libasound2-dev to release-plz.yml
- added libsound2-dev to release-plz.yml
- added required libwayland dependencies for release-plz release and pr
