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

async function parse(input: ArrayBufferLike): Promise<ArrayBufferLike> {
  performance.mark(kLoad);
  const mod = await import(
    /* webpackChunkName: "core" */ "../../wasm-core/pkg"
  );
  performance.mark(kDict);
  await mod.ready();
  performance.mark(kParse);
  const lattice = mod.parse(decoder.decode(input));

  const res = encoder.encode(
    JSON.stringify({
      stats: {
        loadWasm: performance.measure("loadWasm", kLoad, kDict).duration,
        loadDict: performance.measure("loadDict", kDict, kParse).duration,
        parse: performance.measure("parse", kParse).duration,
      },
      dot: lattice.as_dot(),
      wakachi: lattice.wakachi(),
      best: lattice.find_best(),
    })
  );
  return Comlink.transfer(res, [res.buffer]);
}

async function getFeatures(payload: ArrayBufferLike): Promise<ArrayBufferLike> {
  const mod = await import(
    /* webpackChunkName: "features" */ "../../wasm-features/pkg"
  );
  const features = mod.get_features(JSON.parse(decoder.decode(payload)));
  const res = encoder.encode(JSON.stringify(features));
  return Comlink.transfer(res, [res.buffer]);
}

Comlink.expose({ parse, getFeatures });
