import { useContext } from "react";
import { useCookies } from "react-cookie";
import { Navigate, Outlet } from "react-router-dom";
import { AuthContext, COOKIE_NAMES } from "./AuthProvider";

export default function ProtectedRoutes() {
    const { isFetching } = useContext(AuthContext);
    const [cookies] = useCookies(COOKIE_NAMES);

    if (isFetching) {
        return <div />;
    }

    if (!cookies.access_token || !cookies.refresh_token || !cookies.expires) {
        return <Navigate to="/login" replace />;
    }

    return <Outlet />;
}
