import { EOL } from "os";
import fs from "fs";
import path from "path";
import kuromoji from "kuromoji";

const dicPath = path.join(
  path.dirname(new URL(import.meta.url).pathname),
  "node_modules",
  "kuromoji",
  "dict"
);

new Promise((resolve, reject) => {
  kuromoji.builder({ dicPath }).build((err, tokenizer) => {
    if (err) {
      reject(err);
    } else {
      resolve(tokenizer);
    }
  });
}).then((tokenizer) => {
  const lines = fs.readFileSync("/dev/stdin", "utf8").trim().split(EOL);
  for (const line of lines) {
    tokenizer.tokenize(line);
  }
  console.log(process.memoryUsage());
});
