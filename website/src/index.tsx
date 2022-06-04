import ReactDOM from "react-dom/client";
import "./index.css";
import { BrowserRouter, Outlet, Route, Routes } from "react-router-dom";
import LoginCallback from "./LoginCallback";
import AuthProvider from "./auth/AuthProvider";
import ProtectedRoutes from "./auth/ProtectedRoutes";
import { Login } from "./Login";
import {
    AppShell,
    LoadingOverlay,
    MantineProvider,
} from "@mantine/core";
import { NotificationsProvider } from "@mantine/notifications";
import NotFound from "./views/NotFound";
import { CookiesProvider } from "react-cookie";
import React from "react";
import { customColors } from "./customTheme";

const App = React.lazy(() => import("./App"));
const Guild = React.lazy(() => import("./views/Guild"));
const Upload = React.lazy(() => import("./views/Upload"));
const UserFiles = React.lazy(() => import("./views/UserFiles"));
const Sidebar = React.lazy(() => import("./components/Sidebar"));

const root = ReactDOM.createRoot(
    document.getElementById("root") as HTMLElement
);

root.render(
    <MantineProvider
        theme={{
            // colors: customColors,
            colorScheme: "dark",
            primaryColor: "violet",
            primaryShade: { dark: 7, light: 5 }
        }}
    >
        <React.StrictMode>
            <BrowserRouter>
                <React.Suspense fallback={<LoadingOverlay visible={true} />}>
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
                                        <NotificationsProvider>
                                            <AppShell
                                                fixed
                                                children={<Outlet />}
                                                navbar={<Sidebar />}
                                                styles={(theme) => ({
                                                    main: {
                                                        backgroundColor:
                                                            theme.colorScheme ===
                                                            "dark"
                                                                ? theme.colors
                                                                      .dark[6]
                                                                : theme.colors
                                                                      .gray[0],
                                                    },
                                                })}
                                            />
                                        </NotificationsProvider>
                                    }
                                >
                                    <Route path="/" element={<App />} />
                                    {/* TODO: check if route is valid, guild exists, user is in guild... */}
                                    <Route
                                        path="/guilds/:guildId"
                                        element={<Guild />}
                                    />
                                    <Route
                                        path="/upload"
                                        element={<Upload />}
                                    />
                                    <Route
                                        path="/user"
                                        element={<UserFiles />}
                                    />
                                </Route>
                            </Route>
                            <Route
                                path="/login-callback"
                                element={<LoginCallback />}
                            />
                            <Route path="/login" element={<Login />} />
                        </Routes>
                    </CookiesProvider>
                </React.Suspense>
            </BrowserRouter>
        </React.StrictMode>
    </MantineProvider>
);
