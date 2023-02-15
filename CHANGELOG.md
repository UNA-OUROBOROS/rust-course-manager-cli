# Changelog

## [Unreleased]

### changed

- the `completed` status has been renamed to `approved`

## [0.0.3] - 2023-02-13

### changed

- changed the executable name from `course-manager-cli` to `course-manager`

### added

- added the capacity to list multiple filters in the `list` subcommand

### fixed

- fixed a bug where `aprove <course>` would not validate if the course had the requirments met

## [0.0.2] - 2023-02-01

### added

- Added the `--recursive` argument to the `approve` subcommand
- Added the `--recursive` argument to the `reject` subcommand
- Added the `--force` argument to the `approve` subcommand
- Added the `--force` argument to the `reject` subcommand

### removed

- Removed the `--cascade` argument from the `approve` subcommand
- Removed the `--cascade` argument from the `reject` subcommand

### changed

- Renamed the subcommands to be more simple example: `init-courses` to `init`

# [0.0.1] - 2023-02-01

Initial release.
