import { createStyles, Paper } from "@mantine/core";
import { FaDiscord } from "react-icons/fa";
import { API_URL, AuthRoute } from "../api/ApiRoutes";

const useStyles = createStyles((theme) => {
    return {
        paperStyle: {
            width: "220px",
            display: "flex",
            flexDirection: "row",
            flexWrap: "wrap",
            justifyContent: "center",
            alignItems: "center",
            backgroundColor: "#5865f2",
            columnGap: "10px",
            color: theme.colors.gray[0],
        },
    };
});

export default function LoginWithDiscordButton() {
    const { classes } = useStyles();
    return (
        <Paper
            p="xs"
            className={classes.paperStyle}
            component="a"
            href={`${API_URL}${AuthRoute.getInit}`}
        >
            <FaDiscord size={46} />
            Login with discord
        </Paper>
    );
}
