import * as Comlink from "comlink";
import type { WasmLattice } from "../pkg";

export type Stats = {
  loadWasm: number;
  loadDict: number;
  parse: number;
};

const kLoad = "loadWasm";
const kDict = "loadDict";
const kParse = "parse";

async function parse(text: string): Promise<{ dot: string; best: string }> {
  performance.mark(kLoad);
  const mod = await import("../pkg");
  performance.mark(kDict);
  mod.ready();
  performance.mark(kParse);
  const lattice = mod.parse(text);

  const encoder = new TextEncoder();
  const stats = encoder.encode(
    JSON.stringify({
      loadWasm: performance.measure("loadWasm", kLoad, kDict).duration,
      loadDict: performance.measure("loadDict", kDict, kParse).duration,
      parse: performance.measure("parse", kParse).duration,
    })
  );
  const dot = encoder.encode(lattice.as_dot());
  const best = encoder.encode(lattice.find_best());
  return {
    stats,
    dot: Comlink.transfer(dot.buffer, [dot.buffer]),
    best: Comlink.transfer(best.buffer, [best.buffer]),
  };
}

Comlink.expose({ parse });
