# web-streaming-parser-example

An example showing the use of nom streaming parsers with an Electron GUI

<img src="assets/screenshot.png" />

## Status

Working.

## Requirements

- [npm](https://nodejs.org/en/download/)
- [rustup](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [perl](https://www.perl.org/get.html) 😿 (only until [this](https://github.com/rustwasm/wasm-bindgen/pull/1928) gets merged)

## Building

```
$ npm ci
$ npm run build
$ npm start
```

## Usage

Input identifiers (e.g., "foo", "bar", "baz", without quotes) and parentheses (e.g., "(" or ")" without quotes) into the prompt field and press the `<enter>` key. This will feed data into the streaming token parser one line at a time. Identifiers will be parsed as symbols, delimited either by parentheses or spaces. Data passed into the parser will be displayed in the `Chunks` field. Parsed tokens will be displayed in the `Tokens` field.