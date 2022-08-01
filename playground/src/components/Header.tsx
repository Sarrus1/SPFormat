import { Box, Toolbar, Button, AppBar, Typography } from "@material-ui/core";
import packageJson from "../../package.json";

import { sp_format } from "sp_format";
import { Settings } from "../interfaces";

interface HeaderProps {
  readonly code: string;
  settings: Settings;
  setCode: React.Dispatch<React.SetStateAction<string>>;
}

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
          <Button
            color="primary"
            variant="contained"
            style={{ backgroundColor: "grey", marginLeft: "auto" }}
            onClick={(e) => {
              sp_format(props.code, props.settings as Settings)
                .then((res) => props.setCode(res))
                .catch((err) => console.log(err));
            }}
          >
            Format
          </Button>
        </Toolbar>
      </AppBar>
    </Box>
  );
}
