# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [0.1.1] - 2024-02-27

## Added

## Changed

## Fixed

- Fixed an issue where some multi-line doc strings containing placeholders did not compile

# [0.1.0] - 2024-02-26

## Added

- Forked off of `displaydoc`
- `{}` placeholders can now contain arbitrary expressions. The first identifier refers to a struct/enum variant field
- Multi-line doc comments are now collected, except when disabled with `#[ignore_extra_doc_attributes]`
- `#[ignore_extra_doc_attributes]` can now be placed on top of any field where doc comments are expected

## Changed

- Renamed `#[displaydoc()]` to `#[display()]`
