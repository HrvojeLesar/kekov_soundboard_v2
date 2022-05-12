import axios from "axios";
import { useContext, useEffect } from "react";
import { useCookies } from "react-cookie";
import { useNavigate, useSearchParams } from "react-router-dom";
import { CookieSetOptions } from "universal-cookie";
import { AuthContext } from "./auth/AuthProvider";

type Guild = {
    id: string,
    name: string,
    icon?: string,
    icon_hash?: string,
}

type LoginResponse = {
    access_token: string,
    expires_in: number,
    guild?: Guild,
    refresh_token: string,
    scope: string,
    token_type: string
}

function LoginCallback() {
    let [searchParams] = useSearchParams();
    let [_cookies, setCookie] = useCookies(['access_token', 'refresh_token', 'expires']);
    let navigate = useNavigate();
    let authContext = useContext(AuthContext);

    useEffect(() => {
        console.log("LoginCallback");
        const code = searchParams.get('code');
        const state = searchParams.get('state');
        const error = searchParams.get('error');
        if (!error && code && state) {
            (async () => {
                try {
                    // TODO: change url
                    const { data } = await axios.get<LoginResponse>(`http://localhost:8080/v1/auth/callback?code=${code}&state=${state}`);

                    authContext.login({
                        access_token: data.access_token, refresh_token: data.refresh_token, expires: Date.now() + data.expires_in,
                    });

                    let options: CookieSetOptions = { maxAge: data.expires_in };
                    setCookie('access_token', data.access_token, options);
                    setCookie('refresh_token', data.refresh_token, options);
                    setCookie('expires', Date.now() + data.expires_in, options);
                    navigate("/", { replace: true });
                } catch (e) {
                    console.log(e);
                }
            })();
        } else {
            // TODO: Something else
            const error_description = searchParams.get("error_description");
            console.log(error_description);
        }

        return () => { };
    }, []);

    return (<>Logging you in...</>);
}

export default LoginCallback;
