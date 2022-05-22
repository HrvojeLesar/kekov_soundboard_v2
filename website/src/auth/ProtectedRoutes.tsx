import { useContext } from "react";
import { Navigate, Outlet } from "react-router-dom";
import { AuthContext } from "./AuthProvider";

export default function ProtectedRoutes() {
    let auth = useContext(AuthContext);

    if (!auth.tokens) {
        return <Navigate to="/login" replace />;
    }
    return <Outlet />;
}
