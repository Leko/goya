import React, { Suspense, lazy, useState } from "react";
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import Chip from "@mui/material/Chip";
import Tab from "@mui/material/Tab";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import type { Stats } from "./MorphologicalAnalysis.worker";
import { Typography } from "@mui/material";

enum ResultTab {
  Wakachi = "Wakachi",
  Table = "Table",
  Dot = "Dot",
}

type Props = {
  dot?: string;
  wakachi?: string[];
  best?: unknown[] | null;
  stats?: Stats;
};

const Table = lazy(() => import(/* webpackChunkName: "table" */ "./Table"));
const Dot = lazy(() => import(/* webpackChunkName: "dot" */ "./Dot"));

export function Result(props: Props) {
  const { dot, wakachi, best, stats } = props;
  const [tab, setTab] = useState(ResultTab.Wakachi);

  const handleChangeTab = (_: unknown, newValue: ResultTab) => {
    setTab(newValue);
  };

  return (
    <>
      <Box mb={1}>
        <Stack direction="row" spacing={1}>
          <Chip
            size="small"
            label={`load wasm: ${stats?.loadWasm.toFixed(1) ?? "- "}ms`}
          />
          <Chip
            size="small"
            label={`load dictionary: ${stats?.loadDict.toFixed(1) ?? "- "}ms`}
          />
          <Chip
            size="small"
            label={`parse: ${stats?.parse.toFixed(1) ?? "- "}ms`}
          />
        </Stack>
      </Box>
      <TabContext value={tab}>
        <TabList onChange={handleChangeTab} aria-label="解析結果">
          <Tab label="分かち書き" value={ResultTab.Wakachi} disabled={!best} />
          <Tab label="形態素" value={ResultTab.Table} disabled={!best} />
          <Tab label="ラティス" value={ResultTab.Dot} disabled={!best} />
        </TabList>
        <TabPanel value={ResultTab.Wakachi}>
          <Typography>{wakachi?.join("/")}</Typography>
        </TabPanel>
        <TabPanel value={ResultTab.Table}>
          <Suspense fallback={null}>
            <Table rows={best ?? ([] as any[])} />
          </Suspense>
        </TabPanel>
        <TabPanel value={ResultTab.Dot}>
          <Suspense fallback={null}>{dot ? <Dot dot={dot} /> : null}</Suspense>
        </TabPanel>
      </TabContext>
    </>
  );
}
