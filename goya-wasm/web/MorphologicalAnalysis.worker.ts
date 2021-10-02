import * as Comlink from "comlink";
import type { WasmLattice } from "../pkg";

async function parse(text: string): Promise<{ dot: string; best: string }> {
  const mod = await import("../pkg");
  const lattice = await mod.parse(text);
  const encoder = new TextEncoder();
  const dot = encoder.encode(lattice.as_dot());
  const best = encoder.encode(lattice.find_best());
  return {
    dot: Comlink.transfer(dot.buffer, [dot.buffer]),
    best: Comlink.transfer(best.buffer, [best.buffer]),
  };
}

Comlink.expose({ parse });
