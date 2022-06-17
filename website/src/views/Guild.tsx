import { Box, Button, createStyles, Grid, Paper, Title } from "@mantine/core";
import { useDocumentTitle } from "@mantine/hooks";
import axios, { AxiosError, CanceledError } from "axios";
import { CSSProperties, useContext, useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { useParams } from "react-router-dom";
import { AuthContext, COOKIE_NAMES } from "../auth/AuthProvider";
import ControlsWindow from "../components/Guild/ControlsWindow";
import QuickEnableWindow, {
    EnabledUserFile,
} from "../components/Guild/QuickEnableWindow";
import ServerSoundsWindow from "../components/Guild/ServerSoundsWindow";
import { ApiRequest, GuildFile } from "../utils/utils";

export const guildMaximumWindowHeight: CSSProperties = {
    height: "calc(100vh - 34px)",
};

const useStyles = createStyles((theme) => {
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
    useDocumentTitle(`KSv2 - ${guilds.find((g) => g.id === guildId)?.name}`);

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
                    return Date.parse(a.time_added) - Date.parse(b.time_added);
                });
                setGuildFiles(data);
                setIsUpdating(false);
            } catch (e: any | AxiosError) {
                console.log(typeof e);
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

    const quickEnableFilesCallback = (file: EnabledUserFile) => {
        const foundFile = guildFiles.find((f) => {
            return f.id === file.sound_file.id;
        });
        if (foundFile) {
            setGuildFiles([
                ...guildFiles.filter((f) => {
                    return f.id !== foundFile.id;
                }),
            ]);
        } else {
            setGuildFiles([...guildFiles, file.sound_file]);
        }
    };

    const toggleAdminMode = () => {
        setAdminMode(!adminMode);
    };

    useEffect(() => {
        setInvalidServer(false);
        abortController?.abort();
        setIsUpdating(true);
        fetchGuildFiles();
    }, [guildId]);

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
                <Grid.Col xs={9}>
                    <ServerSoundsWindow
                        isUpdating={isUpdating}
                        adminMode={adminMode}
                        toggleAdminMode={toggleAdminMode}
                        guildId={guildId ?? "1"}
                        guildFiles={guildFiles}
                        classes={classes}
                        setGuildFiles={setGuildFiles}
                    />
                </Grid.Col>
                <Grid.Col xs={3}>
                    {adminMode ? (
                        <></>
                    ) : (
                        <Box className={classes.sideWindowsStyle}>
                            <ControlsWindow guildId={guildId ?? "1"} />
                            <QuickEnableWindow
                                guildId={guildId ?? "1"}
                                enableCallback={quickEnableFilesCallback}
                            />
                        </Box>
                    )}
                </Grid.Col>
            </Grid>
        </>
    );
}
