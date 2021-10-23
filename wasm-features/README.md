## Getting started

```ts
import core from "goya-core";
import { get_features } from "wasm-features";

// Mecab IPA辞書のデフォルトでは品詞(Part of Speech)は添字0
const INDEX_POS = 0;

const lattice = core.parse("すもももももももものうち");
const morphemes = lattice.find_best();
// widの配列から素性の配列を得る
const features = get_features(morphemes.map((morph) => morph.wid));
// 1要素ずつ取得してもいいが、まとめて取得する方がオーバーヘッドが少なく高速
get_features([morphemes[0].wid]);

morphemes.forEach(({ surface_form }, i) => {
  const feature = features[i]; // 渡したwid通りの順序で素性が得られる
  const line = surface_form + "\t" + feature.join(",");
  console.log(line); // => "すもも\t名詞,一般,*,*,*,*,すもも,スモモ,スモモ"
  console.log(feature[INDEX_POS]); // => "名詞"
});
```
