import { useCookies } from "react-cookie";
import { Navigate } from "react-router-dom";
import { API_URL, AuthRoute } from "./api/ApiRoutes";
import { COOKIE_NAMES } from "./auth/AuthProvider";

export function Login() {
    const [cookies] = useCookies(COOKIE_NAMES);

    if (cookies.access_token && cookies.refresh_token && cookies.expires) {
        return <Navigate to="/" replace/>;
    }

    return <a href={`${API_URL}${AuthRoute.getInit}`}>LOGIN</a>
}
