# SW

**v0.4.3 :** [Read changelog](./CHANGELOG.md) <!-- x-release-please-version -->

> A little solution wiper. Search code scopes containing a matching comment and wipe the content of the scope.
>
> **Goal :** Remove solution from code exercises to allow students to complete them.

## How it works

Sw is a simple tool that searches some specified comments in a codebase.
Then, he removes the content of the surrounding scope.

SW uses :

- [fd](https://github.com/sharkdp/fd) to select the code files we want to search for solutions to wipe.
- Remove the code betwwen lines containing à `--sw-wipe--` keyword inclusive.

### Defaults

- SW only looks in the src folder
- respects the .ignore and .gitignore files as [fd](https://github.com/sharkdp/fd) does.
- Search for these languages (by file extension) :
  - Rust : `.rs` (also add the `todo!()` macro to allow compilation)
  - PHP : `.php`
  - JavaScript : `.js`
  - TypeScript : `.ts`
  - Java : `.java`
- Wipe the characters between lines containing `--sw-wipe--`.

### Usage

See `sw --help`
