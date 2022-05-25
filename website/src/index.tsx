import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";
import { BrowserRouter, Outlet, Route, Routes } from "react-router-dom";
import LoginCallback from "./LoginCallback";
import AuthProvider from "./auth/AuthProvider";
import ProtectedRoutes from "./auth/ProtectedRoutes";
import { Login } from "./Login";
import { Guild } from "./views/Guild";
import { AppShell, MantineProvider } from "@mantine/core";
import Sidebar from "./components/Sidebar";
import Upload from "./views/Upload";
import UserFiles from "./views/UserFiles";
import { NotificationsProvider } from "@mantine/notifications";
import NotFound from "./views/NotFound";
import { CookiesProvider } from "react-cookie";
import React from "react";

const root = ReactDOM.createRoot(
    document.getElementById("root") as HTMLElement
);

root.render(
    <React.StrictMode>
        <BrowserRouter>
            <CookiesProvider>
                <Routes>
                    <Route path="/*" element={<NotFound />} />
                    <Route
                        element={
                            <AuthProvider>
                                <ProtectedRoutes />
                            </AuthProvider>
                        }
                    >
                        <Route
                            element={
                                <MantineProvider>
                                    <NotificationsProvider>
                                        <AppShell
                                            fixed
                                            children={<Outlet />}
                                            navbar={<Sidebar />}
                                        />
                                    </NotificationsProvider>
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
                    <Route path="/login-callback" element={<LoginCallback />} />
                    <Route path="/login" element={<Login />} />
                </Routes>
            </CookiesProvider>
        </BrowserRouter>
    </React.StrictMode>
);
