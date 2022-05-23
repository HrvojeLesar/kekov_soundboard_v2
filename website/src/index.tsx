import React, { useContext } from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";
import reportWebVitals from "./reportWebVitals";
import { BrowserRouter, Outlet, Route, Routes } from "react-router-dom";
import LoginCallback from "./LoginCallback";
import AuthProvider, { AuthContext } from "./auth/AuthProvider";
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

const root = ReactDOM.createRoot(
    document.getElementById("root") as HTMLElement
);

const Main = () => {
    const { isFetching } = useContext(AuthContext);
    return isFetching ? (
        <div />
    ) : (
        <Routes>
            <Route path="/*" element={<NotFound />} />
            <Route element={<ProtectedRoutes />}>
                <Route
                    element={
                        <MantineProvider>
                            <AppShell
                                fixed
                                children={<Outlet />}
                                navbar={<Sidebar />}
                            />
                        </MantineProvider>
                    }
                >
                    <Route path="/" element={<App />} />
                    {/* TODO: check if route is valid, guild exists, user is in guild... */}
                    <Route
                        path="/guilds/:guildId"
                        element={
                            <NotificationsProvider>
                                <Guild />
                            </NotificationsProvider>
                        }
                    />
                    <Route
                        path="/upload"
                        element={
                            <NotificationsProvider>
                                <Upload />
                            </NotificationsProvider>
                        }
                    />
                    <Route
                        path="/user"
                        element={
                            <NotificationsProvider>
                                <UserFiles />
                            </NotificationsProvider>
                        }
                    />
                </Route>
            </Route>
            <Route path="/login" element={<Login />} />
            <Route path="/login-callback" element={<LoginCallback />} />
        </Routes>
    );
};

root.render(
    <BrowserRouter>
        <CookiesProvider>
            <AuthProvider>
                <Main />
            </AuthProvider>
        </CookiesProvider>
    </BrowserRouter>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
console.log(reportWebVitals());
