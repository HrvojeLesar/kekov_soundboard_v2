import { createStyles } from "@mantine/core";
import { useCookies } from "react-cookie";
import { Navigate } from "react-router-dom";
import { COOKIE_NAMES } from "./auth/AuthProvider";
import LoginWithDiscordButton from "./components/LoginWithDiscordButton";

const useStyle = createStyles((theme) => {
    return {
        loginContainer: {
            width: "100vw",
            height: "100vh",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",

            backgroundColor:
                theme.colorScheme === "dark"
                    ? theme.colors.dark[6]
                    : theme.colors.gray[0],
        },
    };
});

export function Login() {
    const { classes } = useStyle();
    const [cookies] = useCookies(COOKIE_NAMES);

    if (cookies.access_token && cookies.refresh_token && cookies.expires) {
        return <Navigate to="/" replace />;
    }

    return (
        <div className={classes.loginContainer}>
            <LoginWithDiscordButton />
        </div>
    );
}
