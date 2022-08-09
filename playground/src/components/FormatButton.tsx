import { Button, Snackbar } from "@mui/material";
import { useState, forwardRef } from "react";
import { sp_format } from "sp_format";
import MuiAlert, { AlertProps } from "@mui/material/Alert";

import { HeaderProps, Settings } from "../interfaces";

export const Alert = forwardRef<HTMLDivElement, AlertProps>(function Alert(
  props,
  ref
) {
  return <MuiAlert elevation={6} ref={ref} variant="filled" {...props} />;
});

export function FormatButton(props: HeaderProps) {
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
