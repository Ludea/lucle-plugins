import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useState, useEffect } from "react";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import TableRow from "@mui/material/TableRow";
import TableHead from "@mui/material/TableHead";
import TableCell from "@mui/material/TableCell";
import Table from "@mui/material/Table";
import TableContainer from "@mui/material/TableContainer";
import InputAdornment from "@mui/material/InputAdornment";
import Paper from "@mui/material/Paper";
import IconButton from "@mui/material/IconButton";
import { DropzoneArea } from "mui-file-dropzone";
// Icons
import AddCircleIcon from "@mui/icons-material/AddCircle";
import DeleteIcon from "@mui/icons-material/Delete";
// api
import { init, status, unregisterVersion, registerPackage, } from "utils/rpc";
function Speedupdate() {
    const [repoInit, setRepoInit] = useState(false);
    const [url, setUrl] = useState(localStorage.getItem("url") || "");
    const [currentVersion, setCurrentVersion] = useState("");
    const [pack, setPack] = useState();
    const [version, setVersion] = useState();
    const [listPackages, setListPackages] = useState([]);
    const [listVersions, setListVersions] = useState([]);
    const [path, setPath] = useState(localStorage.getItem("path") || "");
    const [client, setClient] = useState();
    const [fileObjects, setFileObjects] = useState();
    const [error, setError] = useState("");
    useEffect(() => {
        status(client, path).then((repo) => {
            setRepoInit(true);
            setCurrentVersion(repo.currentVersion);
            setListVersions(repo.listVersion);
            setListPackages(repo.packages);
        });
    }, [url, path]);
    return (_jsxs("div", { children: [_jsx(TextField, { id: "outlined-required", label: "url", value: url, onChange: (e) => {
                    setUrl(e.currentTarget.value);
                    localStorage.setItem("url", e.currentTarget.value);
                } }), _jsx(TextField, { id: "outlined-required", label: "path", value: path, onChange: (e) => {
                    setPath(e.currentTarget.value);
                    localStorage.setItem("path", e.currentTarget.value);
                } }), !repoInit ? (_jsxs("div", { children: [_jsx(Button, { variant: "contained", onClick: () => {
                            init(client, path).catch((error) => {
                                setRepoInit(false);
                                setError(error);
                            });
                        }, children: "Initialize repository" }), error] })) : (_jsxs("div", { children: [_jsx(Paper, { sx: { width: "65%", mb: 2 }, children: _jsx(TableContainer, { children: _jsxs(Table, { sx: { width: "100%" }, children: [_jsx(TableHead, { children: _jsxs(TableRow, { children: [_jsx(TableCell, { children: "Versions" }), _jsx(TableCell, {})] }) }), listVersions.map((current_version) => (_jsxs(TableRow, { children: [_jsx(TableCell, { children: current_version }), _jsx(TableCell, { children: _jsx(IconButton, { onClick: () => unregisterVersion(client, path, version), children: _jsx(DeleteIcon, {}) }) })] }, current_version)))] }) }) }), _jsx(TextField, { id: "input-with-icon-textfield", label: "Add new version", value: version, onChange: (e) => setVersion(e.currentTarget.value), InputProps: {
                            endAdornment: (_jsx(InputAdornment, { onClick: () => {
                                    // register_version(client, path, version);
                                    setVersion("");
                                }, position: "end", children: _jsx(AddCircleIcon, { color: "success" }) })),
                        }, variant: "standard" }), _jsx(Paper, { sx: { width: "65%", mb: 2 }, children: _jsx(TableContainer, { children: _jsxs(Table, { sx: { width: "100%" }, children: [_jsx(TableHead, { children: _jsxs(TableRow, { children: [_jsx(TableCell, { children: "Packages" }), _jsx(TableCell, {})] }) }), listPackages.map((bin) => (_jsxs(TableRow, { children: [_jsx(TableCell, { children: bin }), _jsx(TableCell, { children: _jsx(IconButton, { children: _jsx(DeleteIcon, {}) }) })] }, bin)))] }) }) }), _jsx(TextField, { id: "input-with-icon-textfield", label: "Add new package", InputProps: {
                            endAdornment: (_jsx(InputAdornment, { onClick: () => registerPackage(client, path, pack), position: "end", children: _jsx(AddCircleIcon, { color: "success" }) })),
                        }, variant: "standard" })] })), "Upload Binaries", _jsx(DropzoneArea, { fileObjects: fileObjects }), _jsx(Button, { color: "primary", sx: {
                    position: "absolute",
                    right: "0",
                }, children: "Submit" })] }));
}
export default Speedupdate;
