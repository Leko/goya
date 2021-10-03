import React, { useEffect, useState } from "react";
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import Chip from "@mui/material/Chip";
import Tab from "@mui/material/Tab";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import type { Stats } from "./MorphologicalAnalysis.worker";
import { Table } from "./Table";
import { Dot } from "./Dot";

enum ResultTab {
  Table = "Table",
  Dot = "Dot",
}

type Props = {
  dot: string;
  best: unknown[] | null;
  stats: Stats;
};

export function Result(props: Props) {
  const { dot, best, stats } = props;
  const [tab, setTab] = useState(ResultTab.Table);

  const handleChangeTab = (_, newValue) => {
    setTab(newValue);
  };

  return (
    <>
      <Box mb={1}>
        <Stack direction="row" spacing={1}>
          <Chip
            size="small"
            label={`load wasm: ${stats?.loadWasm.toFixed(0) ?? "- "}ms`}
          />
          <Chip
            size="small"
            label={`load dictionary: ${stats?.loadDict.toFixed(0) ?? "- "}ms`}
          />
          <Chip
            size="small"
            label={`parse: ${stats?.parse.toFixed(0) ?? "- "}ms`}
          />
        </Stack>
      </Box>
      <TabContext value={tab}>
        <TabList onChange={handleChangeTab} aria-label="解析結果">
          <Tab label="形態素" value={ResultTab.Table} disabled={!best} />
          <Tab label="ラティス" value={ResultTab.Dot} disabled={!best} />
        </TabList>
        <TabPanel value={ResultTab.Table}>
          <Table rows={best ?? []} />
        </TabPanel>
        <TabPanel value={ResultTab.Dot}>
          {dot ? <Dot dot={dot} /> : null}
        </TabPanel>
      </TabContext>
    </>
  );
}
