import { DataGrid } from "@mui/x-data-grid";
import React from "react";

type Props = {
  rows: Record<string, string>[];
};

const base = { flex: 1, sortable: false };

export function Table(props: Props) {
  const columns = [
    { field: "surface_form", headerName: "表層形", ...base },
    { field: "meta_0", headerName: "品詞", ...base },
    { field: "meta_1", headerName: "品詞細分類1", ...base },
    { field: "meta_2", headerName: "品詞細分類2", ...base },
    { field: "meta_3", headerName: "品詞細分類3", ...base },
    { field: "meta_4", headerName: "活用型", ...base },
    { field: "meta_5", headerName: "活用形", ...base },
    { field: "meta_6", headerName: "原形", ...base },
    { field: "meta_7", headerName: "読み", ...base },
    { field: "meta_8", headerName: "発音", ...base },
  ];
  // See morphological_analysis/src/vocabulary.rs
  const rows = props.rows.map((word, i) => ({
    id: i,
    ...word,
    meta_0: word.meta[0],
    meta_1: word.meta[1],
    meta_2: word.meta[2],
    meta_3: word.meta[3],
    meta_4: word.meta[4],
    meta_5: word.meta[5],
    meta_6: word.meta[6],
    meta_7: word.meta[7],
    meta_8: word.meta[8],
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
