import axios from "axios";
import { useContext, useEffect } from "react";
import { useCookies } from "react-cookie";
import { useNavigate, useSearchParams } from "react-router-dom";
import { CookieSetOptions } from "universal-cookie";
import { AuthContext, COOKIE_NAMES, LoginResponse } from "./auth/AuthProvider";

function LoginCallback() {
    let [searchParams] = useSearchParams();
    let [_cookies, setCookie] = useCookies(COOKIE_NAMES);
    let navigate = useNavigate();
    let { login } = useContext(AuthContext);

    useEffect(() => {
        const code = searchParams.get("code");
        const state = searchParams.get("state");
        const error = searchParams.get("error");
        if (!error && code && state) {
            // TODO: change url
            axios
                .get<LoginResponse>(
                    `http://localhost:8080/v1/auth/callback?code=${code}&state=${state}`
                )
                .then(({ data }) => {
                    let options: CookieSetOptions = { maxAge: data.expires_in };
                    setCookie("access_token", data.access_token, options);
                    setCookie("refresh_token", data.refresh_token, options);
                    setCookie("expires", Date.now() + data.expires_in * 1000, options);
                    login(data);
                    navigate("/", { replace: true });
                })
                .catch((e) => {
                    console.log(e);
                    navigate("/", { replace: true });
                });
        } else {
            const error_description = searchParams.get("error_description");
            console.log(error_description);
            navigate("/", { replace: true });
        }
    }, []);

    return <>Logging you in...</>;
}

export default LoginCallback;
