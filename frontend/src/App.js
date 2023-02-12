import React from "react";
import { createTheme, ThemeProvider } from "@mui/material";
import Topbar from "./components/toolbar";
import Description from "./components/description";
import Data from "./components/data";
import Footer from "./components/footer";

const topbarTheme = createTheme({
    typography: {
        fontFamily: ["Roboto Slab"].join(","),
    },
});

const webTheme = createTheme({
    typography: {
        fontFamily: ["Rubik"].join(","),
    },
});

const App = () => {
    return (
        <div className="App">
            <ThemeProvider theme={topbarTheme}>
                <Topbar />
            </ThemeProvider>
            <ThemeProvider theme={webTheme}>
                <Description />
                <Data />
                <Footer />
            </ThemeProvider>
        </div>
    );
};

export default App;
