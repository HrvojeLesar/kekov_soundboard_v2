import { Button, createStyles, Group, Paper, Title } from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import { useState } from "react";
import { useCookies } from "react-cookie";
import {
    Check,
    ClearAll,
    PlayerSkipForward,
    PlayerStop,
    X,
} from "tabler-icons-react";
import { COOKIE_NAMES } from "../../auth/AuthProvider";
import {
    ApiRequest,
    convertClientErrorToString,
    PlayOpCodeEnum,
} from "../../utils/utils";
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
    const [isQueueLoading, setIsQueueLoading] = useState(false);
    const [isSkipLoading, setIsSkipLoading] = useState(false);
    const [isStopLoading, setIsStopLoading] = useState(false);
    const { classes } = useStyles();

    const handleGetQueue = () => {
        setIsQueueLoading(true);
        ApiRequest.controlsGetQueue(guildId, cookies.access_token)
            .then((resp) => {
                console.log(resp);
            })
            .catch((e) => {
                console.log(e);
            })
            .finally(() => {
                setIsQueueLoading(false);
            });
    };

    const handleSkip = () => {
        setIsSkipLoading(true);
        ApiRequest.controlsSkip(guildId, cookies.access_token)
            .then((resp) => {
                if (resp.data.op !== PlayOpCodeEnum.Error) {
                    showNotification({
                        title: "Success",
                        message: "Skipped",
                        autoClose: 1000,
                        color: "green",
                        icon: <Check />,
                    });
                } else {
                    showNotification({
                        title: "Error",
                        message: resp.data.client_error
                            ? convertClientErrorToString(resp.data.client_error)
                            : "Unknown error occured",
                        autoClose: 3000,
                        color: "red",
                        icon: <X />,
                    });
                }
            })
            .catch((e) => {
                console.log(e);
            })
            .finally(() => {
                setIsSkipLoading(false);
            });
    };

    const handleStop = () => {
        setIsStopLoading(true);
        ApiRequest.controlsStop(guildId, cookies.access_token)
            .then((resp) => {
                if (resp.data.op !== PlayOpCodeEnum.Error) {
                    showNotification({
                        title: "Success",
                        message: "Stopped",
                        autoClose: 1000,
                        color: "green",
                        icon: <Check />,
                    });
                } else {
                    showNotification({
                        title: "Error",
                        message: resp.data.client_error
                            ? convertClientErrorToString(resp.data.client_error)
                            : "Unknown error occured",
                        autoClose: 3000,
                        color: "red",
                        icon: <X />,
                    });
                }
            })
            .catch((e) => {
                console.log(e);
            })
            .finally(() => {
                setIsStopLoading(false);
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
                    leftIcon={<ClearAll />}
                    loading={isQueueLoading}
                >
                    Get Queue
                </Button>
                <Button
                    title="Skip"
                    onClick={() => handleSkip()}
                    className={classes.buttonWidth}
                    leftIcon={<PlayerSkipForward />}
                    loading={isSkipLoading}
                >
                    Skip
                </Button>
                <Button
                    title="Stop"
                    onClick={() => handleStop()}
                    color="red"
                    className={classes.buttonWidth}
                    leftIcon={<PlayerStop />}
                    loading={isStopLoading}
                >
                    Stop
                </Button>
            </Group>
        </Paper>
    );
}
