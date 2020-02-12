# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].

## Unreleased

## v0.1.2 - 2020-02-12

### Changed

- Don't update records if their contents won't change

## v0.1.1 - 2020-02-11

### Fixed

- Use DNS record identifier when updating Cloudflare records

## v0.1.0 - 2020-02-11

### Added

- Determine IP address (with ipify or any other plain-text endpoint)
- Create or update `A` record on Cloudflare with retrieved address

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
