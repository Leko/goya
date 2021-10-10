import * as Comlink from "comlink";

export type Stats = {
  loadWasm: number;
  loadDict: number;
  parse: number;
};

const kLoad = "loadWasm";
const kDict = "loadDict";
const kParse = "parse";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

const contextReady = Promise.all([
  import("../../wasm-core/pkg"),
  Promise.all([
    import("../../wasm-core/__generated__/da.json"),
    import("../../wasm-core/__generated__/dict.json"),
    import("../../wasm-core/__generated__/features.json"),
  ]).then((mods) =>
    Promise.all(mods.map((mod) => fetch(mod.default).then((res) => res.text())))
  ),
]).then(
  ([{ GoyaContext }, [da, dict, features]]) =>
    new GoyaContext(da, dict, features)
);

async function parse(input: ArrayBufferLike): Promise<ArrayBufferLike> {
  performance.mark(kLoad);
  const mod = await import("../../wasm-core/pkg");
  performance.mark(kDict);
  const ctx = await contextReady;
  performance.mark(kParse);
  const lattice = mod.parse(decoder.decode(input), ctx);

  const res = encoder.encode(
    JSON.stringify({
      stats: {
        loadWasm: performance.measure("loadWasm", kLoad, kDict).duration,
        loadDict: performance.measure("loadDict", kDict, kParse).duration,
        parse: performance.measure("parse", kParse).duration,
      },
      dot: lattice.as_dot(ctx),
      wakachi: lattice.wakachi(ctx),
      best: lattice.find_best(ctx),
    })
  );
  return Comlink.transfer(res, [res.buffer]);
}

async function getFeatures(payload: ArrayBufferLike): Promise<ArrayBufferLike> {
  const mod = await import("../../wasm-core/pkg");
  const ctx = await contextReady;
  const features = mod.get_features(decoder.decode(payload), ctx);
  const res = encoder.encode(JSON.stringify(features));
  return Comlink.transfer(res, [res.buffer]);
}

Comlink.expose({ parse, getFeatures });
