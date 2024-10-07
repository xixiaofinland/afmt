# Technical Parts

## Parser

[Tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) is depended.

### Update parser version

1. have `tree-sitter` and `node-js` installed locally.
2. Download the parser repo locally, go to its path.
3. Run `tree-sitter gen ./apex/grammar.js` to generate `./src`.
4. Copy `parser.c` and `tree-sitter` sub folder into `afmt/dep` to replace

## Test

Afmt is heavily guarded by test scripts in `tests` folder

### Assert testing

`cargo test --test test --  --show-output`

### Battle testing

Download the lists of Apex repo and format them.

`sh tests/battle_test/download.sh`
`sh tests/battle_test/format.sh`
