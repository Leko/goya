## Getting started

### 分かち書き

goya-core を import して `parse` 関数を使用します。parse メソッドの戻り値から各種メソッドを呼べるようにしています。
分かち書きをするなら`wakachi`メソッドを使用します。

```ts
import core from "goya-core";

const lattice = core.parse("すもももももももものうち");
lattice.wakachi(); // => ["すもも", "も", "もも", "も", "もも", "の", "うち"]
```

### 形態素解析

形態素解析の結果を得るには`find_best`メソッドを使用します。find_best は形態素の配列を返します。各形態素はこれらのフィールドを持っています。サイズ削減のためこのオブジェクトは品詞や読み仮名などの素性を持っていません。

- wid: 語彙 ID。goya-features で使用 （後述）
- is_known: 既知後なら true、未知語なら false
- surface_form: 表層体

```ts
lattice.find_best()[0].surface_form; // => "すもも"
lattice.find_best()[0].is_known; // => true
lattice.find_best()[0].wid; // => 次項で説明
```
