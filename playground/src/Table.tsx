import { DataGrid } from "@mui/x-data-grid";
import React, { useEffect, useState } from "react";
import { wrap, transfer } from "comlink";

interface GoyaFeaturesAPI {
  getFeatures: (input: ArrayBufferLike) => Promise<ArrayBufferLike>;
}
type Props = {
  rows: Record<string, string>[];
};

const encoder = new TextEncoder();
const decoder = new TextDecoder();
const worker = wrap<GoyaFeaturesAPI>(
  new Worker(new URL("./goya.worker.ts", import.meta.url))
);
const base = { flex: 1, sortable: false };

export default function Table(props: Props) {
  const [features, setFeatures] = useState([]);

  const columns = [
    { field: "surface_form", headerName: "表層形", ...base },
    { field: "is_known", headerName: "既知語", ...base },
    { field: "feature_0", headerName: "品詞", ...base },
    { field: "feature_1", headerName: "品詞細分類1", ...base },
    { field: "feature_2", headerName: "品詞細分類2", ...base },
    { field: "feature_3", headerName: "品詞細分類3", ...base },
    { field: "feature_4", headerName: "活用型", ...base },
    { field: "feature_5", headerName: "活用形", ...base },
    { field: "feature_6", headerName: "原形", ...base },
    { field: "feature_7", headerName: "読み", ...base },
    { field: "feature_8", headerName: "発音", ...base },
  ];
  const rows = props.rows.map((row, i) => ({
    id: i,
    ...row,
    feature_0: features[i]?.[0],
    feature_1: features[i]?.[1],
    feature_2: features[i]?.[2],
    feature_3: features[i]?.[3],
    feature_4: features[i]?.[4],
    feature_5: features[i]?.[5],
    feature_6: features[i]?.[6],
    feature_7: features[i]?.[7],
    feature_8: features[i]?.[8],
  }));

  useEffect(() => {
    setFeatures([]);
    if (!props.rows) {
      return;
    }
    const wids = props.rows.map((m) => m.wid);
    const payload = encoder.encode(JSON.stringify(wids));
    worker
      .getFeatures(transfer(payload, [payload.buffer]))
      .then((res) => JSON.parse(decoder.decode(res)))
      .then(setFeatures);
  }, [props.rows]);

  return (
    <DataGrid
      autoHeight
      disableColumnMenu
      disableSelectionOnClick
      rows={rows}
      columns={columns}
    />
  );
}
