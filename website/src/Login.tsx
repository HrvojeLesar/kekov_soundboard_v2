import { useContext } from "react";
import { useCookies } from "react-cookie";
import { Navigate } from "react-router-dom";
import { AuthContext, COOKIE_NAMES } from "./auth/AuthProvider";

export function Login() {
    const [cookies] = useCookies(COOKIE_NAMES);

    if (cookies.access_token && cookies.refresh_token && cookies.expires) {
        return <Navigate to="/" replace/>;
    }

    return <a href="http://localhost:8080/v1/auth/init">LOGIN</a>
}
