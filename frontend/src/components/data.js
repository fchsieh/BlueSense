import React from "react";
import { Grid, Card, CardContent, Typography, Stack } from "@mui/material";
import DevicesIcon from "@mui/icons-material/Devices";

import { useTableSize, useTableData } from "../api/fetchdata";
import DataTable from "./datatable";

const Table = () => {
    const { tableData, isLoading } = useTableData();
    let data = [];
    if (tableData) {
        data = Object.values(tableData)
            .flat()
            .filter((item) => item !== null && item !== undefined);
    }
    return (
        <div>
            <DataTable data={{ data: data }} isLoading={isLoading} />
        </div>
    );
};

const Data = () => {
    const tableSize = useTableSize();

    return (
        <Grid
            container
            spacing={0}
            direction="column"
            alignItems="center"
            justifyContent="center"
            style={{ minHeight: "20vh", marginTop: "2rem" }}
        >
            <Card sx={{ minWidth: 275 }}>
                <CardContent>
                    <Stack direction="row" alignItems="center" gap={1}>
                        <Typography variant="h5" component="div">
                            Nearby devices
                        </Typography>

                        <DevicesIcon />
                    </Stack>
                    <Typography variant="body" color="text.secondary">
                        {tableSize}
                    </Typography>
                </CardContent>
            </Card>
            <div
                style={{
                    width: "40%",
                    margin: "auto",
                    alignItems: "center",
                    justifyContent: "center",
                    marginTop: "2rem",
                    marginBottom: "2rem",
                }}
            >
                <Table />
            </div>
        </Grid>
    );
};

export default Data;
