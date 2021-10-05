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
  const mod = await import("../pkg");
  performance.mark(kDict);
  mod.ready();
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
      best: lattice.find_best(),
    })
  );
  return Comlink.transfer(res, [res.buffer]);
}

Comlink.expose({ parse });
