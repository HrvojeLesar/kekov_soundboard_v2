import { Button, createStyles, Group, Paper, Title } from "@mantine/core";
import { useCookies } from "react-cookie";
import { COOKIE_NAMES } from "../../auth/AuthProvider";
import { ApiRequest } from "../../utils/utils";
const useStyles = createStyles((_theme) => {
    return {
        paperStyle: {
            width: "100%",
        },
        titleStyle: {
            textOverflow: "ellipsis",
            overflow: "hidden",
            whiteSpace: "nowrap",
        },
        buttonWidth: {
            width: "120px",
        },
    };
});

type ControlsWindowProps = {
    guildId: string;
};

export default function ControlsWindow({ guildId }: ControlsWindowProps) {
    const [cookies] = useCookies(COOKIE_NAMES);
    const { classes } = useStyles();

    const handleGetQueue = () => {
        ApiRequest.controlsGetQueue(guildId, cookies.access_token)
            .then((resp) => {
                console.log(resp);
            })
            .catch((e) => {
                console.log(e);
            });
    };

    const handleSkip = () => {
        ApiRequest.controlsSkip(guildId, cookies.access_token)
            .then((resp) => {
                console.log(resp);
            })
            .catch((e) => {
                console.log(e);
            });
    };

    const handleStop = () => {
        ApiRequest.controlsStop(guildId, cookies.access_token)
            .then((resp) => {
                console.log(resp);
            })
            .catch((e) => {
                console.log(e);
            });
    };

    return (
        <Paper withBorder shadow="sm" p="sm" className={classes.paperStyle}>
            <Title
                order={3}
                pb="xs"
                title="Controls"
                className={classes.titleStyle}
            >
                Controls
            </Title>
            <Group position="center">
                <Button
                    title="Get Queue"
                    onClick={() => handleGetQueue()}
                    className={classes.buttonWidth}
                >
                    Get Queue
                </Button>
                <Button
                    title="Skip"
                    onClick={() => handleSkip()}
                    className={classes.buttonWidth}
                >
                    Skip
                </Button>
                <Button
                    title="Stop"
                    onClick={() => handleStop()}
                    color="red"
                    className={classes.buttonWidth}
                >
                    Stop
                </Button>
            </Group>
        </Paper>
    );
}
