import {
    Text,
    createStyles,
    CSSObject,
    MantineTheme,
    Paper,
    UnstyledButton,
} from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import axios, { AxiosResponse } from "axios";
import { useState } from "react";
import { useCookies } from "react-cookie";
import { Check, PlayerPlay, X } from "tabler-icons-react";
import { API_URL, ControlsRoute } from "../api/ApiRoutes";
import { COOKIE_NAMES } from "../auth/AuthProvider";
import {
    ClientErrorEnum,
    convertClientErrorToString,
    PlayOpCodeEnum,
} from "../utils/utils";
import { GuildFile } from "../views/Guild";

const playButtonStyle = (theme: MantineTheme): CSSObject => ({
    width: "50px",
    height: "50px",
    borderRadius: "50%",
    display: "flex",
    textAlign: "center",
    justifyContent: "center",
    alignItems: "center",
    backgroundColor: theme.colors.blue[6],
    color: theme.colors.gray[0],

    "&:hover": {
        backgroundColor: theme.colors.blue[7],
    },

    ":active": {
        transform: "translateY(1px)",
    },
});

const useStyles = createStyles((theme) => ({
    playButtonStyle: {
        ...playButtonStyle(theme),
    },

    container: {
        width: "200px",
        overflow: "hidden",

        "&:hover": {
            backgroundColor: theme.colors.gray[0],
            transition: ".2s",
        },
    },
}));

type PlayPayload = {
    guild_id: string;
    file_id: string;
    channel_id?: string;
};

type PlayControlProps = {
    file: GuildFile;
    guildId: string;
};

type PlayResponse = {
    client_error?: ClientErrorEnum;
    op: PlayOpCodeEnum;
};

export function PlayControl({ file, guildId }: PlayControlProps) {
    const { classes } = useStyles();
    const [cookies] = useCookies(COOKIE_NAMES);
    const [isSendingReq, setIsSendingReq] = useState(false);

    // TODO: Add separate messages for playing and adding to queue
    const playFunc = async (fileId: string) => {
        if (cookies.access_token && guildId) {
            try {
                setIsSendingReq(true);
                let payload: PlayPayload = {
                    guild_id: guildId,
                    file_id: fileId,
                };
                const resp = await axios.post<
                    PlayPayload,
                    AxiosResponse<PlayResponse>
                >(`${API_URL}${ControlsRoute.postPlay}`, payload, {
                    headers: { Authorization: `${cookies.access_token}` },
                });
                console.log(resp.data);
                if (resp.data.op !== PlayOpCodeEnum.Error) {
                    showNotification({
                        title: "Success",
                        message:
                            resp.data.op === PlayOpCodeEnum.PlayResponse
                                ? "Playing"
                                : "Added to queue",
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
                setIsSendingReq(false);
            } catch (e) {
                // TODO: Handle
                console.log(e);
                showNotification({
                    title: "Error",
                    message: "Failed to play or add sound to queue!",
                    autoClose: 5000,
                    color: "red",
                    icon: <X />,
                });
                setIsSendingReq(false);
            }
            console.log(fileId);
        }
    };

    return (
        <Paper
            radius="md"
            withBorder
            shadow="sm"
            p="sm"
            className={classes.container}
        >
            <Text
                title={file.display_name}
                lineClamp={1}
                weight="bold"
                align="center"
                mb="sm"
                mx="xl"
            >
                {file.display_name}
            </Text>
            {isSendingReq ? (
                <div>Please wait</div>
            ) : (
                <UnstyledButton
                    mx="auto"
                    className={classes.playButtonStyle}
                    onClick={() => {
                        playFunc(file.id);
                    }}
                >
                    <PlayerPlay />
                </UnstyledButton>
            )}
        </Paper>
    );
}
