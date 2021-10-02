import React, { useEffect, useState } from "react";
import Tab from "@mui/material/Tab";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import { Table } from "./Table";
import { Dot } from "./Dot";

enum ResultTab {
  Table = "Table",
  Dot = "Dot",
}

type Props = {
  dot: string;
  best: unknown[] | null;
};

export function Result(props: Props) {
  const { dot, best } = props;
  const [tab, setTab] = useState(ResultTab.Table);

  const handleChangeTab = (_, newValue) => {
    setTab(newValue);
  };

  return (
    <>
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
