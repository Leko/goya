import { DataGrid } from "@mui/x-data-grid";
import React from "react";

type Props = {
  rows: Record<string, string>[];
};

const base = { flex: 1, sortable: false };

export function Table(props: Props) {
  const columns = [
    { field: "surface_form", headerName: "表層形", ...base },
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
  // See morphological_analysis/src/vocabulary.rs
  const rows = props.rows.map((word, i) => ({
    id: i,
    ...word,
    feature_0: word.feature[0],
    feature_1: word.feature[1],
    feature_2: word.feature[2],
    feature_3: word.feature[3],
    feature_4: word.feature[4],
    feature_5: word.feature[5],
    feature_6: word.feature[6],
    feature_7: word.feature[7],
    feature_8: word.feature[8],
  }));

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
