import { EOL } from "os";
import path from "path";
import fs from "fs";
import Benchmark from "benchmark";
import kuromoji from "kuromoji";
import core from "wasm-core";
import features from "wasm-features";

const suite = new Benchmark.Suite();

const [, , tokenizer] = await Promise.all([
  core.ready(),
  features.ready(),
  new Promise((resolve, reject) => {
    kuromoji
      .builder({
        dicPath: path.join(
          path.dirname(new URL(import.meta.url).pathname),
          "node_modules",
          "kuromoji",
          "dict"
        ),
      })
      .build((err, tokenizer) => {
        if (err) {
          return reject(err);
        }
        resolve(tokenizer);
      });
  }),
]);

const lines = fs.readFileSync("/dev/stdin", "utf8").trim().split(EOL);
suite
  .add("goya", () => {
    for (const line of lines) {
      const lattice = core.parse(line);
      features.get_features(lattice.find_best().map(({ wid }) => wid));
    }
  })
  .add("kuromoji", () => {
    for (const line of lines) {
      tokenizer.tokenize(line);
    }
  })
  .on("cycle", (event) => {
    console.log(String(event.target));
  })
  .on("complete", function () {
    console.log("Fastest is " + this.filter("fastest").map("name"));
  })
  .run({ async: true });
