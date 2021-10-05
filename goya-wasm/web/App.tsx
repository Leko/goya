import React, { useCallback, useState } from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import IconButton from "@mui/material/IconButton";
import Container from "@mui/material/Container";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import TextField from "@mui/material/TextField";
import GitHubIcon from "@mui/icons-material/GitHub";
import { useDebounce } from "react-use";
import * as Comlink from "comlink";
import type { Stats } from "./MorphologicalAnalysis.worker";
import { Result } from "./Result";

interface ProxyAPI {
  parse: (input: ArrayBufferLike) => Promise<ArrayBufferLike>;
}
const proxy = Comlink.wrap<ProxyAPI>(
  new Worker(new URL("./MorphologicalAnalysis.worker.ts", import.meta.url))
);
const encoder = new TextEncoder();
const decoder = new TextDecoder();

export function App() {
  const [text, setText] = useState("すもももももももものうち");
  const [result, setResult] = useState<{
    dot: string;
    best: unknown[];
    stats: Stats;
  } | null>(null);

  const handleChangeText = useCallback(
    (event) => {
      setText(event.target.value.trim());
    },
    [setText]
  );
  useDebounce(
    () => {
      if (text.length === 0) {
        setResult(null);
      } else {
        const input = encoder.encode(text);
        proxy
          .parse(Comlink.transfer(input, [input.buffer]))
          .then((res) => decoder.decode(res))
          .then((res) => JSON.parse(res))
          .then(({ dot, best, stats }) => {
            setResult({
              stats,
              dot,
              best: JSON.parse(best),
            });
          });
      }
    },
    200,
    [text]
  );

  return (
    <>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" component="h1" sx={{ flexGrow: 1 }}>
            Goya playground
          </Typography>
          <IconButton
            size="large"
            color="inherit"
            aria-label="GitHub"
            href="https://github.com/Leko/goya"
            rel="noreferer"
            target="_blank"
            sx={{ mr: 2 }}
          >
            <GitHubIcon />
          </IconButton>
        </Toolbar>
      </AppBar>
      <Container>
        <Box mt={4}>
          <Typography variant="h4" component="h2" sx={{ flexGrow: 1 }}>
            Goya: Yet another morphological analyzer for Rust and WebAssembly
          </Typography>
          <Typography variant="body1" component="p" sx={{ flexGrow: 1 }}>
            Goya: WebAssemblyで利用可能なRust製形態素解析ライブラリ
          </Typography>
        </Box>
        <Box mt={2}>
          <TextField
            label="文章を入力"
            margin="dense"
            multiline
            rows={4}
            fullWidth
            value={text}
            onChange={handleChangeText}
          />
        </Box>
        <Box mt={2}>
          <Result {...(result ?? {})} />
        </Box>
      </Container>
    </>
  );
}
