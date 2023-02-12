import React from "react";
import { Paper, Typography, Link } from "@mui/material";

export default function Footer() {
    return (
        <Paper
            sx={{
                padding: "8px 16px",
                marginTop: "0.5rem",
                position: "fixed",
                left: 0,
                bottom: 0,
                width: "100%",
                backgroundColor: "#f5f5f5",
            }}
        >
            <Typography variant="body2" align="center" color="textSecondary">
                Made with{" "}
                <Typography display="inline" color="red">
                    <span role="img" aria-label="Love">
                        ❤️
                    </span>
                </Typography>{" "}
                by{" "}
                <Link
                    underline="hover"
                    href="https://github.com/fchsieh"
                    rel="noreferrer"
                    target="_blank"
                >
                    F.C. Hsieh
                </Link>{" "}
                @ {new Date().getFullYear()} - BlueSense
            </Typography>
        </Paper>
    );
}
