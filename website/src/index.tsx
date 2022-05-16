import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";
import reportWebVitals from "./reportWebVitals";
import { BrowserRouter, Outlet, Route, Routes } from "react-router-dom";
import LoginCallback from "./LoginCallback";
import AuthProvider from "./auth/AuthProvider";
import ProtectedRoutes from "./auth/ProtectedRoutes";
import { Login } from "./Login";
import { Guild } from "./views/Guild";
import { AppShell, Footer, MantineProvider } from "@mantine/core";
import Sidebar from "./components/Sidebar";
import Upload from "./views/Upload";
import UserFiles from "./views/UserFiles";
import { ModalsProvider } from "@mantine/modals";

const root = ReactDOM.createRoot(
    document.getElementById("root") as HTMLElement
);

root.render(
    // TODO: Setup proper routing
    <React.StrictMode>
        <AuthProvider>
            <BrowserRouter>
                <Routes>
                    <Route element={<ProtectedRoutes />}>
                        <Route
                            element={
                                <MantineProvider>
                                    <ModalsProvider>
                                        <AppShell
                                            fixed
                                            children={<Outlet />}
                                            navbar={<Sidebar />}
                                        />
                                    </ModalsProvider>
                                </MantineProvider>
                            }
                        >
                            <Route path="/" element={<App />} />
                            {/* TODO: check if route is valid, guild exists, user is in guild... */}
                            <Route
                                path="/guilds/:guildId"
                                element={<Guild />}
                            />
                            <Route path="/upload" element={<Upload />} />
                            <Route path="/user" element={<UserFiles />} />
                        </Route>
                    </Route>
                    <Route path="/login" element={<Login />} />
                    <Route path="/login-callback" element={<LoginCallback />} />
                </Routes>
            </BrowserRouter>
        </AuthProvider>
    </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
console.log(reportWebVitals());