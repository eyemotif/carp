# carp
A CLI dependency manager for rust's Cargo.

## Commands

### list
`carp list`: Lists all dependencies and their installed versions in the current Cargo workspace.

```
% carp list
crates_io_api (0.5.1)  
toml (0.5.6)
```
### add
`carp add <crate name> [crate version]`: Adds a crate to the dependencies in the current Cargo workspace. If no version is specified, the latest version is used.

```
% carp add rand
+ rand (0.7.3)
% carp list
toml (0.5.6)
rand (0.7.3)
crates_io_api (0.5.1)
```
### rem
`carp add <dependency name>`: Removes a dependency from the current Cargo workspace.

```
% carp rem rand
- rand
% carp list
toml (0.5.6)
crates_io_api (0.5.1)
```
### change
`carp change <dependency name> <crate version>`: Changes the version of a dependcy in the current Cargo workspace..

```
% carp change toml 0.5.5
* toml (0.5.6) -> (0.5.5)
% carp list
toml (0.5.5)
crates_io_api (0.5.1)
```
### check
`carp check [crate name]`: Checks if a dependency is up to date. If no dependency is specified, all dependencies installed are checked.

```
% carp check
! toml (0.5.5): (0.5.6)
```

### update
`carp update [crate name]`: Updates a dependency if it is not up to date. If no dependency is specified, all dependencies installed are updated.

```
% carp update
* toml (0.5.5) -> (0.5.6)
* crates_io_api (0.5.0) -> (0.5.1)
```
## Info
- All parameters with "crate" in their name check the crates.io registry.
- Carp does not change anything in your Rust projects other than its Cargo.toml file. All other functionality is covered by Cargo itself.
