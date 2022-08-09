import { Box, Toolbar, AppBar, Typography } from "@mui/material";
import packageJson from "../../package.json";

import { HeaderProps } from "../interfaces";
import { FormatButton } from "./FormatButton";

export default function Header(props: HeaderProps) {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="static">
        <Toolbar
          style={{ backgroundColor: "rgb(0, 120, 215)" }}
          variant="dense"
        >
          <Typography variant="h6" component="div" style={{ flexGrow: 1 }}>
            SPFormat v{packageJson.version}
          </Typography>
          <FormatButton {...props} />
        </Toolbar>
      </AppBar>
    </Box>
  );
}
