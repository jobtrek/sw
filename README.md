# SW

> A little solution wiper. Search code scopes containing a matching comment and wipe the content of the scope.
>
> **Goal :** Remove solution from code exercises to allow students to complete them.

## How it works

Sw is a simple tool that search some specified comments in a codebase.
Then, he removes the content of the surrounding scope.

It uses :
- [fd](https://github.com/sharkdp/fd) to select the files we want to search for solutions to wipe.
- [ast-grep](https://github.com/ast-grep/ast-grep) to find comment pattern with abstract syntax tree.

