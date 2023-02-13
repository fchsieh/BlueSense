import { useState, useEffect } from "react";
import config from "../api.json";

const API_URL = config.API_URL;

const FETCH_INTERVAL = 5 * 1000; // 5 seconds

const useTableSize = () => {
    const [time, setTime] = useState(0);
    const [tableSize, setTableSize] = useState(0);

    useEffect(() => {
        const interval = setInterval(() => {
            setTime(Date.now());
        }, FETCH_INTERVAL);
        return () => clearInterval(interval);
    }, [time]);

    useEffect(() => {
        const fetchTableSize = async () => {
            const response = await fetch(`${API_URL}/size`);
            const data = await response.json();
            setTableSize(data.length);
        };
        fetchTableSize();
    }, [time]);

    return tableSize;
};

const useTableData = () => {
    const [time, setTime] = useState(0);
    const [tableData, setTableData] = useState([]);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        const interval = setInterval(() => {
            setTime(Date.now());
        }, FETCH_INTERVAL);
        return () => clearInterval(interval);
    }, [time]);

    useEffect(() => {
        const fetchTableData = async () => {
            const response = await fetch(`${API_URL}/data`);
            const data = await response.json();
            if (data) setIsLoading(false);
            setTableData(data);
        };
        fetchTableData();
    }, [time]);

    return { tableData, isLoading };
};

export { useTableSize, useTableData };
