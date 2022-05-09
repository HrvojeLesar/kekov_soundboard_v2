import { useContext } from "react";
import { Navigate } from "react-router-dom";
import { AuthContext } from "./AuthProvider";

function ProtectedRoutes({ children }: { children: JSX.Element }) {
    let auth = useContext(AuthContext);

    if (!auth.tokens) {
        return <Navigate to="/login" replace />
    }
    return children;
}

export default ProtectedRoutes;
