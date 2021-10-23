# Goya benchmarks

## Getting started

```
npm i
./scripts/setup # Generate ita-corpus.txt

# Run whole process benchmark
node goya.js < ita-corpus.txt
node kuromoji.js < ita-corpus.txt

# Run morphological analysis benchmark
node bench.js < ita-corpus.txt
```
