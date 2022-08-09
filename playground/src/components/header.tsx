import {
  Box,
  Toolbar,
  Button,
  AppBar,
  Typography,
  Snackbar,
} from "@mui/material";
import MuiAlert, { AlertProps } from "@mui/material/Alert";
import packageJson from "../../package.json";
import { useState, forwardRef } from "react";

import { sp_format } from "sp_format";
import { Settings } from "../interfaces";

const Alert = forwardRef<HTMLDivElement, AlertProps>(function Alert(
  props,
  ref
) {
  return <MuiAlert elevation={6} ref={ref} variant="filled" {...props} />;
});

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
          <FormatButton {...props} />
        </Toolbar>
      </AppBar>
    </Box>
  );
}

function FormatButton(props: HeaderProps) {
  const [showError, setShowError] = useState(false);

  const handleClose = (
    event?: React.SyntheticEvent | Event,
    reason?: string
  ) => {
    if (reason === "clickaway") {
      return;
    }
    setShowError(false);
  };

  return (
    <>
      <Button
        color="primary"
        variant="contained"
        style={{ backgroundColor: "grey", marginLeft: "auto" }}
        onClick={(e) => {
          sp_format(props.code, props.settings as Settings)
            .then((res) => {
              if (res.length == 0 && props.code.trim().length > 0) {
                setShowError(true);
              } else {
                props.setCode(res);
              }
            })
            .catch((err) => console.log(err));
        }}
      >
        Format
      </Button>
      <Snackbar
        open={showError}
        autoHideDuration={6000}
        onClose={handleClose}
        anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
      >
        <Alert onClose={handleClose} severity="error" sx={{ width: "100%" }}>
          There is an error in your syntax.
        </Alert>
      </Snackbar>
    </>
  );
}
