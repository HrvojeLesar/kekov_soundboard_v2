import {
    AppShell,
    ColorScheme,
    ColorSchemeProvider,
    LoadingOverlay,
    MantineProvider,
    Paper,
} from "@mantine/core";
import { useDocumentTitle, useLocalStorage } from "@mantine/hooks";
import { NotificationsProvider } from "@mantine/notifications";
import React from "react";
import { CookiesProvider } from "react-cookie";
import {
    BrowserRouter,
    Navigate,
    Outlet,
    Route,
    Routes,
} from "react-router-dom";
import AuthProvider from "./auth/AuthProvider";
import ProtectedRoutes from "./auth/ProtectedRoutes";
import { Login } from "./Login";
import LoginCallback from "./LoginCallback";
import NotFound from "./views/NotFound";

const Guild = React.lazy(() => import("./views/Guild"));
const Upload = React.lazy(() => import("./views/Upload"));
const UserFiles = React.lazy(() => import("./views/UserFiles"));
const Sidebar = React.lazy(() => import("./components/Sidebar"));
const PublicFiles = React.lazy(() => import("./views/PublicFiles"));

export default function App() {
    const [colorScheme, setColorScheme] = useLocalStorage<ColorScheme>({
        key: "color-scheme",
        defaultValue: "dark",
        getInitialValueInEffect: true,
    });
    useDocumentTitle("Kekov Soundboard v2. Beta.");

    const toggleColorScheme = (value?: ColorScheme) => {
        setColorScheme(value || (colorScheme === "dark" ? "light" : "dark"));
    };

    return (
        <ColorSchemeProvider
            colorScheme={colorScheme}
            toggleColorScheme={toggleColorScheme}
        >
            <MantineProvider
                theme={{
                    // colors: customColors,
                    colorScheme: colorScheme,
                    primaryColor: "violet",
                    primaryShade: { dark: 7, light: 5 },
                }}
            >
                <BrowserRouter>
                    <React.Suspense
                        fallback={<LoadingOverlay visible={true} />}
                    >
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
                                                                    ? theme
                                                                          .colors
                                                                          .dark[6]
                                                                    : theme
                                                                          .colors
                                                                          .gray[0],
                                                        },
                                                    })}
                                                />
                                            </NotificationsProvider>
                                        }
                                    >
                                        <Route
                                            path="/"
                                            element={
                                                <Navigate to="/user" replace />
                                            }
                                        />
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
                                        <Route
                                            path="/public"
                                            element={<PublicFiles />}
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
            </MantineProvider>
        </ColorSchemeProvider>
    );
}
