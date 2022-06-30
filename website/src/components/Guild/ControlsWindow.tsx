import { Button, createStyles, Group, Paper, Title, Text, Box } from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import { useState } from "react";
import { useCookies } from "react-cookie";
import {
    TbCheck,
    TbClearAll,
    TbPlayerSkipForward,
    TbPlayerStop,
    TbX,
} from "react-icons/tb";
import { COOKIE_NAMES } from "../../auth/AuthProvider";
import { windowTitleOverflow } from "../../GlobalStyles";
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
            ...windowTitleOverflow,
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

    // WARN: A longer queue can overflow out of element
    const handleGetQueue = () => {
        setIsQueueLoading(true);
        ApiRequest.controlsGetQueue(guildId, cookies.access_token)
            .then(({ data }) => {
                showNotification({
                    title: "Queue",
                    message:
                        data.length === 0
                            ? "Queue is empty!"
                            : data.map((q, index) => {
                                  return index === 0 ? (
                                      <Text weight="bold" key={index}>
                                          Currently playing:
                                          <Text
                                              weight={500}
                                              component="span"
                                          >{` ${q.display_name}`}</Text>
                                      </Text>
                                  ) : (
                                      <Text
                                          key={index}
                                      >{`${index}. ${q.display_name}`}</Text>
                                  );
                              }),
                    autoClose: 5000,
                    color: "green",
                });
            })
            .catch((e) => {
                showNotification({
                    title: "Error",
                    message: "Failed to fetch queue!",
                    autoClose: 3000,
                    color: "red",
                    icon: <TbX size={24} />,
                });
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
                        icon: <TbCheck size={24} />,
                    });
                } else {
                    showNotification({
                        title: "Error",
                        message: resp.data.client_error
                            ? convertClientErrorToString(resp.data.client_error)
                            : "Unknown error occured",
                        autoClose: 3000,
                        color: "red",
                        icon: <TbX size={24} />,
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
                        icon: <TbCheck size={24} />,
                    });
                } else {
                    showNotification({
                        title: "Error",
                        message: resp.data.client_error
                            ? convertClientErrorToString(resp.data.client_error)
                            : "Unknown error occured",
                        autoClose: 3000,
                        color: "red",
                        icon: <TbX size={24} />,
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
            <Box>
                <Title
                    order={3}
                    pb="xs"
                    title="Controls"
                    className={classes.titleStyle}
                >
                    Controls
                </Title>
            </Box>
            <Group position="center">
                <Button
                    title="Get Queue"
                    onClick={() => handleGetQueue()}
                    className={classes.buttonWidth}
                    leftIcon={<TbClearAll size={24} />}
                    loading={isQueueLoading}
                >
                    Queue
                </Button>
                <Button
                    title="Skip"
                    onClick={() => handleSkip()}
                    className={classes.buttonWidth}
                    leftIcon={<TbPlayerSkipForward size={24} />}
                    loading={isSkipLoading}
                >
                    Skip
                </Button>
                <Button
                    title="Stop"
                    onClick={() => handleStop()}
                    color="red"
                    className={classes.buttonWidth}
                    leftIcon={<TbPlayerStop size={24} />}
                    loading={isStopLoading}
                >
                    Stop
                </Button>
            </Group>
        </Paper>
    );
}
