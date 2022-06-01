import {
    Box,
    createStyles,
    Grid,
    Group,
    Paper,
    ScrollArea,
    Title,
} from "@mantine/core";
import { useDocumentTitle } from "@mantine/hooks";
import { showNotification } from "@mantine/notifications";
import axios, { CanceledError } from "axios";
import { CSSProperties, useContext, useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { useParams } from "react-router-dom";
import { X } from "tabler-icons-react";
import {
    API_URL,
    ControlsRoute,
    GuildRoute,
    UserRoute,
} from "../api/ApiRoutes";
import { AuthContext, COOKIE_NAMES } from "../auth/AuthProvider";
import ControlsWindow from "../components/Guild/ControlsWindow";
import QuickEnableWindow, {
    EnabledUserFile,
} from "../components/Guild/QuickEnableWindow";
import ServerSoundsWindow from "../components/Guild/ServerSoundsWindow";
import { PlayControl } from "../components/PlayControl";
import { UserFile } from "./UserFiles";

export type GuildFile = {
    id: string;
    display_name?: string;
    owner?: string;
};

export const guildMaximumWindowHeight: CSSProperties = {
    height: "calc(100vh - 34px)",
};

const useStyles = createStyles((theme) => {
    return {
        serverSoundsPaper: {
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
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
    };
});

let abortController: AbortController | undefined = undefined;

export default function Guild() {
    const { guilds } = useContext(AuthContext);
    const { guildId } = useParams();
    const [cookies] = useCookies(COOKIE_NAMES);
    const [guildFiles, setGuildFiles] = useState<GuildFile[]>([]);
    const [isUpdating, setIsUpdating] = useState(true);
    const [adminMode, setAdminMode] = useState(false);
    const { classes } = useStyles();
    useDocumentTitle(`KSv2 - ${guilds.find((g) => g.id === guildId)?.name}`);

    const fetchGuildFiles = async () => {
        if (cookies?.access_token) {
            try {
                abortController = new AbortController();
                const { data } = await axios.get<GuildFile[]>(
                    `${API_URL}${GuildRoute.getGuildSounds}${guildId}`,
                    {
                        headers: {
                            Authorization: `${cookies.access_token}`,
                        },
                        signal: abortController.signal,
                    }
                );
                setGuildFiles(data);
                setIsUpdating(false);
            } catch (e) {
                // TODO: Handle
                console.log(e);
                if (e instanceof CanceledError) {
                    return;
                } else {
                    setIsUpdating(false);
                }
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

    useEffect(() => {
        abortController?.abort();
        setIsUpdating(true);
        fetchGuildFiles();
    }, [guildId]);

    return (
        <>
            {isUpdating ? (
                <>Loading...</>
            ) : (
                <Grid>
                    <Grid.Col xs={9}>
                        <ServerSoundsWindow
                            guildId={guildId}
                            guildFiles={guildFiles}
                            classes={classes}
                        />
                    </Grid.Col>
                    <Grid.Col xs={3}>
                        <Box className={classes.sideWindowsStyle}>
                            <ControlsWindow guildId={guildId} />
                            <QuickEnableWindow
                                guildId={guildId}
                                enableCallback={quickEnableFilesCallback}
                            />
                        </Box>
                    </Grid.Col>
                </Grid>
            )}
        </>
    );
}
