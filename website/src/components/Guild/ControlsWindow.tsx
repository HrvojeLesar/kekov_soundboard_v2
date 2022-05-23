import { Button, createStyles, Group, Paper, Title } from "@mantine/core";
import axios from "axios";
import { useContext } from "react";
import { useCookies } from "react-cookie";
import { API_URL, ControlsRoute } from "../../api/ApiRoutes";
import { AuthContext, COOKIE_NAMES } from "../../auth/AuthProvider";
const useStyles = createStyles((theme) => {
    return {
        paperStyling: {
            width: "100%",
        },
    };
});

type ControlsWindowProps = {
    guildId?: string;
};

export default function ControlsWindow({ guildId }: ControlsWindowProps) {
    const [cookies] = useCookies(COOKIE_NAMES);
    const { classes } = useStyles();

    const handleGetQueue = () => {
        axios
            .post(
                `${API_URL}${ControlsRoute.postQueue}`,
                { guild_id: guildId },
                {
                    headers: {
                        Authorization: `${cookies.access_token}`,
                    },
                }
            )
            .then((resp) => {
                console.log(resp);
            })
            .catch((e) => {
                console.log(e);
            });
    };

    const handleSkip = () => {
        axios
            .post(
                `${API_URL}${ControlsRoute.postSkip}`,
                { guild_id: guildId },
                {
                    headers: {
                        Authorization: `${cookies.access_token}`,
                    },
                }
            )
            .then((resp) => {
                console.log(resp);
            })
            .catch((e) => {
                console.log(e);
            });
    };

    const handleStop = () => {
        axios
            .post(
                `${API_URL}${ControlsRoute.postStop}`,
                { guild_id: guildId },
                {
                    headers: {
                        Authorization: `${cookies.access_token}`,
                    },
                }
            )
            .then((resp) => {
                console.log(resp);
            })
            .catch((e) => {
                console.log(e);
            });
    };

    return (
        <Paper withBorder shadow="sm" p="sm" className={classes.paperStyling}>
            <Title
                order={3}
                pb="xs"
                title="Controls"
                style={{
                    textOverflow: "ellipsis",
                    overflow: "hidden",
                    whiteSpace: "nowrap",
                }}
            >
                Controls
            </Title>
            <Group position="center">
                <Button
                    title="Get Queue"
                    onClick={() => handleGetQueue()}
                >
                    Get Queue
                </Button>
                <Button
                    title="Skip"
                    onClick={() => handleSkip()}
                >
                    Skip
                </Button>
                <Button
                    title="Stop"
                    onClick={() => handleStop()}
                    color="red"
                >
                    Stop
                </Button>
            </Group>
        </Paper>
    );
}
