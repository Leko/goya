# Goya

[![goya at crates.io](https://img.shields.io/crates/v/goya.svg)](https://crates.io/crates/goya)
[![goya at docs.rs](https://docs.rs/goya/badge.svg)](https://docs.rs/goya)

Goya is a Japanese Morphological Analyzer written in Rust.  
The main goal is to compile to WebAssembly for morphological analysis in browsers and other JavaScript runtimes. In addition, it can be used with the CLI and Rust.

[Try Goya playground](https://goya.pages.dev/). It uses the Goya-wasm from WebWorker.

## Getting started

### Fetch the latest IPA dictionary

Download the latest IPA dictionary from [the official Mecab website](https://taku910.github.io/mecab/) and unzip it.

### Install Goya CLI

```
cargo install goya-cli
```

### Compile the IPA dictionary

Compile the IPA dictionary to generate a binary dictionary for morphological analysis. It may take a few minutes.

```
goya compile /path/to/ipadic
```

The binary dictionary will be generated in the `~/.goya` directory by default. You can change the destination with the `--dicdir` option.

```
goya --dicdir=/path/to/generated compile /path/to/ipadic
```

### Run Morphological Analysis

Goya takes input from STDIN. The easiest way is using the echo command and pipe it to the Goya.

```
$ echo すもももももももものうち | goya
すもも	名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
の	助詞,連体化,*,*,*,*,の,ノ,ノ
うち	名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
EOS
```

If you specified the `--dicdir` option when compiling the dictionary, you should also specify it when running the goya command.

```
echo すもももももももものうち | goya --dicdir=/path/to/generated
```

## Release

```
cargo release <patch|minor|major> --workspace --no-tag --skip-publish --dependent-version Upgrade
git tag v{{VERSION}}
git push origin v{{VERSION}}
```
