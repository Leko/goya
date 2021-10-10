import React, { useCallback, useEffect, useRef, useState } from "react";
import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Viz from "viz.js";
import workerURL from "viz.js/full.render.js";

type Props = {
  dot: string;
};

const viz = new Viz({ workerURL });

export default function Dot(props: Props) {
  const { dot } = props;
  const [svg, setSVG] = useState<string>("");

  const handleDownload = useCallback(() => {
    const a = document.createElement("a");
    a.download = "lattice.svg";
    a.href = `data://image/svg+xml,${encodeURIComponent(svg)}`;
    a.click();
  }, [svg]);

  useEffect(() => {
    if (!dot || dot.trim().length === 0) {
      return;
    }
    viz.renderSVGElement(dot).then((svg: SVGSVGElement) => {
      svg.style.width = "100%";
      svg.style.height = "100%";
      setSVG(svg.outerHTML);
    });
  }, [dot, setSVG]);

  if (!svg) {
    return null;
  }
  return (
    <Box>
      <Button variant="contained" onClick={handleDownload}>
        Download as SVG
      </Button>
      <div dangerouslySetInnerHTML={{ __html: svg }} />
    </Box>
  );
}
