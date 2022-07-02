import {
    Text,
    createStyles,
    CSSObject,
    MantineTheme,
    Paper,
    UnstyledButton,
    LoadingOverlay,
} from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import { useState } from "react";
import { useCookies } from "react-cookie";
import { TbCheck, TbPlayerPlay, TbX } from "react-icons/tb";
import { COOKIE_NAMES } from "../auth/AuthProvider";
import {
    ApiRequest,
    convertClientErrorToString,
    GuildFile,
    LOADINGOVERLAY_ZINDEX,
    PlayOpCodeEnum,
    PlayPayload,
    primaryShade,
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

const useStyles = createStyles((theme) => {
    const shade = primaryShade(theme);
    return {
        container: {
            width: "200px",
            height: "150px",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            cursor: "pointer",
            position: "relative",
            overflow: "hidden",
            backgroundColor:
                theme.colorScheme === "dark"
                    ? theme.colors.dark[8]
                    : theme.white,

            "&:hover": {
                backgroundColor: theme.fn.rgba(
                    theme.colors[theme.primaryColor][shade],
                    0.3
                ),
                transition: ".2s",
            },

            "&:active": {
                transform: "translateY(1px)",
            },
        },
        textStyle: {
            textOverflow: "ellipsis",
            overflow: "hidden",
            userSelect: "none",
        },
        playerPlayIconStyle: {
            strokeWidth: "1",
            position: "absolute",
            opacity: "0.05",
            width: "100%",
            height: "100%",
        },
        buttonStyle: {
            position: "absolute",
            width: "100%",
            height: "100%",
        },
    };
});

type PlayControlProps = {
    file: GuildFile;
    guildId: string;
    selectedChannelId: string | undefined;
};

export function PlayControl({ file, guildId, selectedChannelId }: PlayControlProps) {
    const { classes } = useStyles();
    const [cookies] = useCookies(COOKIE_NAMES);
    const [isSendingReq, setIsSendingReq] = useState(false);

    const playFunc = async (fileId: string) => {
        if (cookies.access_token && guildId) {
            try {
                setIsSendingReq(true);
                console.log(selectedChannelId);
                let payload: PlayPayload = {
                    guild_id: guildId,
                    file_id: fileId,
                    channel_id: selectedChannelId,
                };
                const resp = await ApiRequest.controlsPlay(
                    payload,
                    cookies.access_token
                );
                if (resp.data.op !== PlayOpCodeEnum.Error) {
                    showNotification({
                        title: "Success",
                        message:
                            resp.data.op === PlayOpCodeEnum.PlayResponse
                                ? "Playing"
                                : "Added to queue",
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
                setIsSendingReq(false);
            } catch (e) {
                // TODO: Handle
                console.log(e);
                showNotification({
                    title: "Error",
                    message: "Failed to play or add sound to queue!",
                    autoClose: 5000,
                    color: "red",
                    icon: <TbX />,
                });
                setIsSendingReq(false);
            }
        }
    };

    return (
        <Paper
            radius="md"
            withBorder
            shadow="sm"
            p="sm"
            className={classes.container}
            title={file.sound_file.display_name}
        >
            <LoadingOverlay
                zIndex={LOADINGOVERLAY_ZINDEX}
                visible={isSendingReq}
            />
            <Text
                weight="bold"
                align="center"
                mx="xl"
                className={classes.textStyle}
            >
                {file.sound_file.display_name}
            </Text>
            <TbPlayerPlay className={classes.playerPlayIconStyle} />
            <UnstyledButton
                className={classes.buttonStyle}
                onClick={() => {
                    playFunc(file.file_id);
                }}
            />
        </Paper>
    );
}
