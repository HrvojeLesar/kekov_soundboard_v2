import axios from "axios";
import { useEffect } from "react";
import { useCookies } from "react-cookie";
import { useNavigate, useSearchParams } from "react-router-dom";
import { CookieSetOptions } from "universal-cookie";
import { COOKIE_NAMES, LoginResponse } from "./auth/AuthProvider";
import { cookieOptions } from "./utils/utils";

function LoginCallback() {
    let [searchParams] = useSearchParams();
    let [_cookies, setCookie] = useCookies(COOKIE_NAMES);
    let navigate = useNavigate();

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
                    const options = cookieOptions(data);
                    setCookie("access_token", data.access_token, options);
                    setCookie("refresh_token", data.refresh_token, options);
                    setCookie("expires", Date.now() + data.expires_in * 1000, options);
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
