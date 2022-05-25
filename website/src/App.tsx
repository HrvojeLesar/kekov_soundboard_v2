import { useDocumentTitle } from "@mantine/hooks";
import { useCookies } from "react-cookie";
import { COOKIE_NAMES } from "./auth/AuthProvider";

export default function App() {
    const [cookies] = useCookies(COOKIE_NAMES);
    useDocumentTitle("Kekov Soundboard v2. Beta.");

    const renderLoginLink = () => {
        if (!cookies.access_token && !cookies.refresh_token && !cookies.expires) {
            return <a href="http://localhost:8080/v1/auth/init">LOGIN</a>;
        }
        return <div />;
    };

    return (
        <>
            {renderLoginLink()}
            <p>TODO: Landing page</p>
        </>
    );
}
