import {
    Text,
    createStyles,
    CSSObject,
    MantineTheme,
    Paper,
    UnstyledButton,
} from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import { useState } from "react";
import { useCookies } from "react-cookie";
import { Check, PlayerPlay, X } from "tabler-icons-react";
import { COOKIE_NAMES } from "../auth/AuthProvider";
import {
    ApiRequest,
    convertClientErrorToString,
    GuildFile,
    PlayOpCodeEnum,
    PlayPayload,
} from "../utils/utils";

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

    "&:active": {
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

type PlayControlProps = {
    file: GuildFile;
    guildId: string;
};

export function PlayControl({ file, guildId }: PlayControlProps) {
    const { classes } = useStyles();
    const [cookies] = useCookies(COOKIE_NAMES);
    const [isSendingReq, setIsSendingReq] = useState(false);

    const playFunc = async (fileId: string) => {
        if (cookies.access_token && guildId) {
            try {
                setIsSendingReq(true);
                let payload: PlayPayload = {
                    guild_id: guildId,
                    file_id: fileId,
                };
                const resp = await ApiRequest.controlsPlay(
                    payload,
                    cookies.access_token
                );
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
