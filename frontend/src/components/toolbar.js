import React from "react";
import { AppBar, Toolbar, Typography } from "@mui/material";

const Topbar = () => {
    return (
        <AppBar position="static">
            <Toolbar>
                <Typography variant="h5" component="div" sx={{ flexGrow: 1 }}>
                    BlueSense
                </Typography>
            </Toolbar>
        </AppBar>
    );
};

export default Topbar;
