import React from "react";
import { useMemo } from "react";
import MaterialReactTable from "material-react-table";

const formatDate = (date) => {
    return new Date(parseInt(date)).toLocaleString("en-US");
};

const DataTable = ({ data, isLoading }) => {
    // map data to table columns
    let data_map = [];
    for (const [, value] of Object.entries(data.data)) {
        const addr = Object.keys(value)[0];
        const time = Object.values(value)[0];
        data_map.push({
            addr: addr,
            time: formatDate(time),
        });
    }

    data_map.sort((a, b) => {
        return b.time - a.time;
    });

    const columns = useMemo(
        () => [
            {
                header: "Bluetooth ID (MD5 Hash)",
                accessorKey: "addr",
            },
            {
                header: "Last Seen",
                accessorKey: "time",
            },
        ],
        []
    );

    return (
        <MaterialReactTable
            columns={columns}
            data={data ? data_map : []}
            enableColumnActions={false}
            enableColumnFilters={false}
            enableTopToolbar={false}
            enableStickyHeader
            muiTableContainerProps={{
                sx: { maxHeight: "25rem" },
            }}
            muiTableHeadCellProps={{
                sx: {
                    fontWeight: "bold",
                    fontSize: "16px",
                },
            }}
            muiTableBodyCellProps={{
                sx: {
                    fontSize: "15px",
                },
            }}
            state={{ isLoading: isLoading }}
        />
    );
};

export default DataTable;
