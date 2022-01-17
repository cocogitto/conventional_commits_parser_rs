# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.9.4](https://github.com/oknozor/conventional_commits_parser_rs/compare/0.9.3..0.9.4) - 2022-01-17
#### Bug Fixes
- handle windows new lines escape sequence in footer separators - ([1df053f](https://github.com/oknozor/conventional_commits_parser_rs/commit/1df053fb3133ca36ca4127cf48231bf7c77b3d33)) - [@oknozor](https://github.com/oknozor)
#### Tests
- add reproducer test for #cocogitto/169 - ([2263236](https://github.com/oknozor/conventional_commits_parser_rs/commit/22632366d71819ad8cfa7bff82a2ac290686c043)) - [@oknozor](https://github.com/oknozor)
- - -

## [0.9.3](https://github.com/oknozor/conventional_commits_parser_rs/compare/0.9.2..0.9.3) - 2021-11-21
#### Bug Fixes
- add clone impl for ParseError - ([69ec82f](https://github.com/oknozor/conventional_commits_parser_rs/commit/69ec82f30f3f50bbb0b82080742b2173c11984e7)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- update cog.toml - ([4e7e5ea](https://github.com/oknozor/conventional_commits_parser_rs/commit/4e7e5ea819d7dbe4fe2658a676f780893148c35c)) - [@oknozor](https://github.com/oknozor)
- - -

## 0.9.2 - 2021-11-04


### Bug Fixes

0015e7 - scope with whitespace fail to parse - Paul Delafosse


- - -
## 0.9.1 - 2021-11-03


### Bug Fixes

bf7237 - allow multiline footer content - Paul Delafosse


### Miscellaneous Chores

e3a2e8 - use speculoos for test assertions - Paul Delafosse


- - -## Unreleased ([cb20195..b0a509a](https://github.com/oknozor/conventional_commits_parser_rs/compare/cb20195..b0a509a))
#### Features
- expose footer token separator - ([b0a509a](https://github.com/oknozor/conventional_commits_parser_rs/commit/b0a509aa190ba30178a56bd309d4f736989ba036)) - [@oknozor](https://github.com/@oknozor)
- - -
## 0.5.2 - 2021-10-19


### Features

[6d9429](https://github.com/oknozor/conventional_commits_parser_rs/commit/6d94294418163cd0677b12568bc77841cf481679) - derive Ord for commit types - [oknozor](https://github.com/oknozor)


- - -
## 0.4.2 - 2021-09-12


### Bug Fixes

[702698](https://github.com/oknozor/conventional_commits_parser_rs/commit/702698fdc0e86216a19d2d59c466181614b04fbc) - change rule name to 'no_parenthesis' for error formating - [oknozor](https://github.com/oknozor)


- - -
## 0.4.1 - 2021-09-12


### Bug Fixes

[ce4eac](https://github.com/oknozor/conventional_commits_parser_rs/commit/ce4eac57bbf3ccc09f2a26798227859d123418ce) - fix to_string with breaking changes footer - [oknozor](https://github.com/oknozor)


- - -
## 0.4.0 - 2021-09-12


### Features

[a70d28](https://github.com/oknozor/conventional_commits_parser_rs/commit/a70d28d76ab34cd074254b09b0d131c1a181d0c6) - better error reporting and dedicated parse functions - [oknozor](https://github.com/oknozor)


- - -
## 0.3.0 - 2021-09-12


### Features

[77b851](https://github.com/oknozor/conventional_commits_parser_rs/commit/77b851da8a9f7015cece7ca0cabb1ebb6b20ebc7) - get rid of anyhow - [oknozor](https://github.com/oknozor)


- - -
## 0.2.0 - 2021-09-11


### Documentation

[0c2259](https://github.com/oknozor/conventional_commits_parser_rs/commit/0c2259ad875f2f3fd0599d9d87f3aeedcd5dcb7b) - add docs and crate badge - [oknozor](https://github.com/oknozor)


### Features

[0a21e6](https://github.com/oknozor/conventional_commits_parser_rs/commit/0a21e6afc1ba63dd112c7078d3282c0aa14b5017) - add convenient str/string convertion function - [oknozor](https://github.com/oknozor)


- - -
## 0.1.0 - 2021-09-09


### Miscellaneous Chores

[df9299](https://github.com/oknozor/conventional_commits_parser_rs/commit/df92996f786c57570648ed5c5e2594b11457bae3) - add cog config - [oknozor](https://github.com/oknozor)

[519961](https://github.com/oknozor/conventional_commits_parser_rs/commit/519961525ecd9f2e6ebbbeeb2691c058f5c43229) - add LICENSE - [oknozor](https://github.com/oknozor)


### Documentation

[7b6886](https://github.com/oknozor/conventional_commits_parser_rs/commit/7b68869288242e0a7c6bd603293aca94fbba424b) - add crate documentation - [oknozor](https://github.com/oknozor)

[b1d7c2](https://github.com/oknozor/conventional_commits_parser_rs/commit/b1d7c275414d9a56392a0f5a1521e783cec47732) - add README.md - [oknozor](https://github.com/oknozor)


### Bug Fixes

[d8ab56](https://github.com/oknozor/conventional_commits_parser_rs/commit/d8ab5602efc168f3eff9c1b737bed0869ae76859) - allow blank line in body - [oknozor](https://github.com/oknozor)


### Tests

[f7a2a3](https://github.com/oknozor/conventional_commits_parser_rs/commit/f7a2a3af3592a63530877f3f298364682aeffae6) - breaking change with dash - [oknozor](https://github.com/oknozor)

[27f8b4](https://github.com/oknozor/conventional_commits_parser_rs/commit/27f8b42c1d4f77bf686fd673b27671a0ff9c106c) - add full specifiaction tests - [oknozor](https://github.com/oknozor)


### Features

[b15df0](https://github.com/oknozor/conventional_commits_parser_rs/commit/b15df069cbd3143c9d6e19e51187115e5068a005) - change unknown error message - [oknozor](https://github.com/oknozor)

[ff3541](https://github.com/oknozor/conventional_commits_parser_rs/commit/ff3541d8342e74d1a7d7b7c2742e51b9a994344e) - insensitive commit type - [oknozor](https://github.com/oknozor)

[9c68cc](https://github.com/oknozor/conventional_commits_parser_rs/commit/9c68ccde2904b1720a086d823798e98ae64cc504) - fail on lower case breaking change - [oknozor](https://github.com/oknozor)

[26d84c](https://github.com/oknozor/conventional_commits_parser_rs/commit/26d84c6c401b244ad21e934ecc730a5384de35a8) - implement multiline footer - [oknozor](https://github.com/oknozor)

[566dba](https://github.com/oknozor/conventional_commits_parser_rs/commit/566dba108941151923dc7bd6e313c351ad29ddd2) - add parsing for boody and footer - [oknozor](https://github.com/oknozor)


- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).