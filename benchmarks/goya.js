import { EOL } from "os";
import fs from "fs";
import core from "wasm-core";
import features from "wasm-features";

const lines = fs.readFileSync("/dev/stdin", "utf8").trim().split(EOL);
for (const line of lines) {
  const lattice = core.parse(line);
  const best = lattice.find_best().map(({ wid }) => wid);
  features.get_features(best);
}

console.log(process.memoryUsage());
