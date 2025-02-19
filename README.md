# SW

**v0.3.6 :** [Read changelog](./CHANGELOG.md) <!-- x-release-please-version -->

> A little solution wiper. Search code scopes containing a matching comment and wipe the content of the scope.
>
> **Goal :** Remove solution from code exercises to allow students to complete them.

## How it works

Sw is a simple tool that searches some specified comments in a codebase.
Then, he removes the content of the surrounding scope.

SW uses :
- [fd](https://github.com/sharkdp/fd) to select the code files we want to search for solutions to wipe.
- [ast-grep](https://github.com/ast-grep/ast-grep) to find patterns in comments with abstract syntax tree.
- Remove the code next to the comment and in the same code scope with simple rust.

### Defaults

- SW only looks in the src folder
- respects the .ignore and .gitignore files as [fd](https://github.com/sharkdp/fd) does.
- Search for these languages (by file extension) :
  - Rust : `.rs` (also add the `todo!()` macro to allow compilation)
  - PHP : `.php`
  - JavaScript : `.js`
  - TypeScript : `.ts`
  - Java : `.java`
- Wipe the characters between a line or block comment marching `Your code goes here` to the end of the code scope containing the comment. *If your comment is on the first line of a function, it will wipe the function body.*

### Configure

Arguments available :

!TODO
