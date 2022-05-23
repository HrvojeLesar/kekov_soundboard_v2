import { Button } from "@mantine/core";
import { useContext } from "react";
import { AuthContext } from "../auth/AuthProvider";

export default function Logout() {
    const { logout } = useContext(AuthContext);

    return <Button onClick={() => logout()}>Logout</Button>
}
