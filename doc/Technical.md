# Technical Parts

## Parser

[Tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) is depended.

### Update parser version

Check `dep/update_parser_version.sh`. Steps can be also done manually as:

1. have `tree-sitter` and `node-js` installed locally
2. Download the parser repo to `dep/` folder, go to its path
3. Run `tree-sitter gen ./apex/grammar.js` to generate `./src`
4. Copy `parser.c` and `tree-sitter` sub folder into `/dep` to replace
5. Remove the parser repo downloaded in #2

## Test

Afmt is heavily guarded by test scripts in `tests` folder

### Assert testing

`cargo test --test test --  --show-output`

### Battle testing

Download the lists of Apex repo and format them.

`sh tests/battle_test/download.sh`
`sh tests/battle_test/format.sh`
