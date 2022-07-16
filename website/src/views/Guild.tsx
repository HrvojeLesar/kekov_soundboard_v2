import { Box, createStyles, Grid, Paper, Title } from "@mantine/core";
import { useDocumentTitle } from "@mantine/hooks";
import axios, { AxiosError, CanceledError } from "axios";
import { CSSProperties, useContext, useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { useParams } from "react-router-dom";
import { AuthContext, COOKIE_NAMES } from "../auth/AuthProvider";
import ControlsWindow from "../components/Guild/ControlsWindow";
import DiscordChannelsWindow from "../components/Guild/DiscordChannelsWindow";
import QuickEnableWindow from "../components/Guild/QuickEnableWindow";
import ServerSoundsWindow from "../components/Guild/ServerSoundsWindow";
import { windowTitleOverflow } from "../GlobalStyles";
import { ApiRequest, GuildFile, primaryShade } from "../utils/utils";

export const guildMaximumWindowHeight: CSSProperties = {
    height: "calc(100vh - 34px)",
};

const useStyles = createStyles((theme) => {
    const shade = primaryShade(theme);
    return {
        serverSoundsPaper: {
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
            position: "relative",
            ...guildMaximumWindowHeight,
        },
        scollAreaStyle: {
            height: "100%",
        },
        sideWindowsStyle: {
            display: "flex",
            flexDirection: "column",
            gap: theme.spacing.sm,
            ...guildMaximumWindowHeight,
        },
        invalidPaperStyle: {
            width: "100%",
            height: "100%",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            flexDirection: "column",
        },
        titleStyle: {
            ...windowTitleOverflow,
        },
        button: {
            width: "250px",
            overflow: "hidden",
            display: "flex",
            alignItems: "center",
            transition: "background-color 150ms ease, border-color 150ms ease",
            border: `1px solid ${
                theme.colorScheme === "dark"
                    ? theme.colors.dark[shade]
                    : theme.colors.gray[shade]
            }`,
            borderRadius: theme.radius.sm,
            padding: 0,
            backgroundColor:
                theme.colorScheme === "dark"
                    ? theme.colors.dark[8]
                    : theme.white,

            "&:hover": {
                transition: "150ms ease",
                backgroundColor:
                    theme.colorScheme === "dark"
                        ? theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          )
                        : theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          ),
            },
        },
        unstyledButtonStyle: { width: "100%", height: "100%" },
        textStyle: {
            overflow: "hidden",
            textOverflow: "ellipsis",
            whiteSpace: "nowrap",
        },
        iconStyle: {
            flexShrink: 0,
        },
    };
});

let abortController: AbortController | undefined = undefined;

export default function Guild() {
    const { guilds } = useContext(AuthContext);
    const { guildId } = useParams();
    const [cookies] = useCookies(COOKIE_NAMES);
    const [guildFiles, setGuildFiles] = useState<GuildFile[]>([]);
    const [isUpdating, setIsUpdating] = useState(true);
    // TODO: admin
    const [adminMode, setAdminMode] = useState(false);
    const { classes } = useStyles();
    const [invalidServer, setInvalidServer] = useState(false);

    const [selectedChannelId, setSelectedChannelId] = useState<
        string | undefined
    >(undefined);

    useDocumentTitle(
        `KSv2 - ${guilds.find((g) => g.id === guildId)?.name ?? ""}`
    );

    const quickEnableFilesCallback = (file: GuildFile) => {
        const foundFile = guildFiles.find((f) => {
            return f.file_id === file.sound_file.id;
        });
        if (foundFile) {
            setGuildFiles([
                ...guildFiles.filter((f) => {
                    return f.file_id !== foundFile.file_id;
                }),
            ]);
        } else {
            setGuildFiles([...guildFiles, file]);
        }
    };

    const toggleAdminMode = () => {
        setAdminMode(!adminMode);
    };

    useEffect(() => {
        const fetchGuildFiles = async () => {
            if (cookies?.access_token && guildId) {
                try {
                    abortController = new AbortController();
                    const { data } = await ApiRequest.fetchGuildFiles(
                        guildId,
                        abortController,
                        cookies.access_token
                    );
                    data.sort((a, b) => {
                        return (
                            Date.parse(a.time_added) - Date.parse(b.time_added)
                        );
                    });
                    setGuildFiles(data);
                    setIsUpdating(false);
                } catch (e: any | AxiosError) {
                    console.log(e);
                    if (e instanceof CanceledError) {
                        return;
                    }
                    if (axios.isAxiosError(e)) {
                        if (
                            e.response?.status === 401 ||
                            e.response?.status === 404
                        ) {
                            setInvalidServer(true);
                        }
                        return;
                    }
                    setIsUpdating(false);
                }
            }
        };

        setInvalidServer(false);
        abortController?.abort();
        setIsUpdating(true);
        fetchGuildFiles();
    }, [guildId, cookies.access_token]);

    return invalidServer ? (
        <Paper
            withBorder
            shadow="sm"
            p="sm"
            className={classes.invalidPaperStyle}
        >
            <Title>
                You don't belong to this server or server ID is invalid!
            </Title>
            <Title order={4}>Try selecting another server!</Title>
        </Paper>
    ) : (
        <>
            <Grid>
                <Grid.Col xs={!adminMode ? 9 : 12}>
                    <ServerSoundsWindow
                        isUpdating={isUpdating}
                        adminMode={adminMode}
                        toggleAdminMode={toggleAdminMode}
                        guildId={guildId ?? "1"}
                        guildFiles={guildFiles}
                        classes={classes}
                        setGuildFiles={setGuildFiles}
                        selectedChannelId={selectedChannelId}
                    />
                </Grid.Col>
                {adminMode ? (
                    <></>
                ) : (
                    <Grid.Col xs={3}>
                        <Box className={classes.sideWindowsStyle}>
                            <ControlsWindow guildId={guildId ?? "1"} />
                            <QuickEnableWindow
                                guildId={guildId ?? "1"}
                                enableCallback={quickEnableFilesCallback}
                            />
                            <DiscordChannelsWindow
                                guildId={guildId ?? "1"}
                                selectChannelCallback={(channelId) => {
                                    setSelectedChannelId(channelId);
                                }}
                                selectedChannelId={selectedChannelId}
                            />
                        </Box>
                    </Grid.Col>
                )}
            </Grid>
        </>
    );
}
