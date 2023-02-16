# Changelog

## [Unreleased]

### added

- when `add` filter is used in the `list` subcommand after other filters it will add the remaining filters to the list of courses to be shown and before the rest of the filters added

### fixed

- fixed multiples typos where `aprove` was used instead of `approve`

## [0.0.4] - 2023-02-14

### added

- now the `list` subcommand will order by filter when multiple `-s` or `--status` arguments are passed in their respective order

### changed

- the `completed` status has been renamed to `approved`
- the `list` subcommand now shows `no courses found` instead of an empty table

### fixed

- now the courses when approved with the `--recursive` argument will be approved forcefully

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
