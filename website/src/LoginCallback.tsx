import { useEffect } from "react";
import { useCookies } from "react-cookie";
import { useNavigate, useSearchParams } from "react-router-dom";
import { COOKIE_NAMES } from "./auth/AuthProvider";
import { loginContainerUseStyle } from "./Login";
import { ApiRequest, cookieOptions } from "./utils/utils";

function LoginCallback() {
    const { classes } = loginContainerUseStyle();
    let [searchParams] = useSearchParams();
    let [_cookies, setCookie] = useCookies(COOKIE_NAMES);
    let navigate = useNavigate();

    useEffect(() => {
        const code = searchParams.get("code");
        const state = searchParams.get("state");
        const error = searchParams.get("error");
        if (!error && code && state) {
            ApiRequest.loginCallback(code, state)
                .then(({ data }) => {
                    const options = cookieOptions(data);
                    setCookie("access_token", data.access_token, options);
                    setCookie("refresh_token", data.refresh_token, options);
                    setCookie(
                        "expires",
                        Date.now() + data.expires_in * 1000,
                        options
                    );
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
    }, [navigate, searchParams, setCookie]);

    return <div className={classes.loginContainer}>Logging you in...</div>;
}

export default LoginCallback;
