import { useContext } from "react";
import { Navigate } from "react-router-dom";
import { AuthContext } from "./auth/AuthProvider";

export function Login() {
    const auth = useContext(AuthContext);

    if (auth.tokens) {
        return <Navigate to="/" replace/>;
    }

    return <a href="http://localhost:8080/v1/auth/init">LOGIN</a>
}
