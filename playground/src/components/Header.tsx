import { Box, Toolbar, Button, AppBar } from "@material-ui/core";

import { sp_format } from "../../../pkg/sp_format";

interface HeaderProps {
  readonly code: string;
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
          <Button
            color="primary"
            variant="contained"
            style={{ backgroundColor: "grey", marginLeft: "auto" }}
            onClick={(e) => {
              sp_format(props.code)
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
