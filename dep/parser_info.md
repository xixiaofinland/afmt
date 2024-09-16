# Parser usage

[Tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) is depended.

## Update parser version

1. Download the parser repo locally, go to its path.
2. Run `tree-sitter gen ./apex/grammar.js` to generate `./src`.
3. Copy `parser.c` and `tree-sitter` sub folder into `afmt/dep` to replace
