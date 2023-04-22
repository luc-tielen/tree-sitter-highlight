# tree-sitter-highlight

A syntax highlighter for Node.js powered by [Tree Sitter](https://github.com/tree-sitter/tree-sitter). Written in Rust.
<strong>NOTE: This fork only supports [Eclair](https://github.com/luc-tielen/eclair-lang)</strong>

## Usage

The following will output HTML:

```js
const treeSitter = require("tree-sitter-highlight");

treeSitter.highlight("reachable(x, y) :- edge(x, y).");
// => '<span class="source">...</span>'
```

You can also output a [HAST](https://github.com/syntax-tree/hast) AST, which is useful for integrating with Markdown or MDX processors (e.g. Remark).

```js
treeSitter.highlightHast("reachable(x, y) :- edge(x, y).");
// => {type: 'element', children: [...]}
```

## Themes

The output HTML will contain CSS class names for various tokens. Here is a basic example theme:

```css
.keyword {
  color: purple;
}

.function {
  color: blue;
}

.type {
  color: pink;
}

.string {
  color: green;
}

.number {
  color: brown;
}

.operator {
  color: gray;
}

.comment {
  color: lightgray;
}
```

Inspect the generated output HTML and design your CSS accordingly.

## License

MIT
