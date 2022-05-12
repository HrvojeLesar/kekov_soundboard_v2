import { useContext } from "react";
import { Navigate } from "react-router-dom";
import { AuthContext } from "./auth/AuthProvider";

export function Login() {
    let auth = useContext(AuthContext);

    if (auth.tokens) {
        return <Navigate to="/" />
    }

    return (
        <a href="http://localhost:8080/v1/auth/init">LOGIN</a>
    );
}
