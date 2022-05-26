import {
    createStyles,
    Grid,
    Group,
    Paper,
    ScrollArea,
    Title,
} from "@mantine/core";
import { useDocumentTitle } from "@mantine/hooks";
import axios, { CanceledError } from "axios";
import { CSSProperties, useContext, useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { useParams } from "react-router-dom";
import {
    API_URL,
    ControlsRoute,
    GuildRoute,
    UserRoute,
} from "../api/ApiRoutes";
import { AuthContext, COOKIE_NAMES } from "../auth/AuthProvider";
import ControlsWindow from "../components/Guild/ControlsWindow";
import { PlayControl } from "../components/PlayControl";
import { UserFile } from "./UserFiles";

export type GuildFile = {
    id: string;
    display_name?: string;
    owner?: string;
};

type PlayPayload = {
    guild_id: string;
    file_id: string;
    channel_id?: string;
};

export const guildMaximumWindowHeight: CSSProperties = {
    height: "calc(100vh - 34px)",
};

const useStyles = createStyles((_theme) => {
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
        groupStyle: {
            width: "100%",
            ...guildMaximumWindowHeight,
        },
        quickEnablePaper: {
            flexGrow: 1,
            width: "100%",
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
        },
    };
});

let abortController: AbortController | undefined = undefined;

export default function Guild() {
    const { guilds } = useContext(AuthContext);
    const { guildId } = useParams();
    const [cookies] = useCookies(COOKIE_NAMES);
    const [guildFiles, setGuildFiles] = useState<GuildFile[]>([]);
    const [userFiles, setUserFiles] = useState<UserFile[]>([]);
    const [isUpdating, setIsUpdating] = useState(false);
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

    const fetchUserFiles = async () => {
        if (cookies.access_token) {
            try {
                const { data } = await axios.get<UserFile[]>(
                    `${API_URL}${UserRoute.getFiles}`,
                    {
                        headers: { authorization: `${cookies.access_token}` },
                    }
                );
                console.log("files: ", data);
                setUserFiles(data);
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    // TODO: move into play control
    const playFunc = async (fileId: string) => {
        if (cookies.access_token && guildId) {
            try {
                let payload: PlayPayload = {
                    guild_id: guildId,
                    file_id: fileId,
                };
                await axios.post<PlayPayload>(
                    `${API_URL}${ControlsRoute.postPlay}`,
                    payload,
                    { headers: { Authorization: `${cookies.access_token}` } }
                );
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
            console.log(fileId);
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
                        <Paper
                            withBorder
                            shadow="sm"
                            p="sm"
                            className={classes.serverSoundsPaper}
                        >
                            <Title title="Server sounds" order={3} pb="xs">
                                Server sounds
                            </Title>
                            <ScrollArea className={classes.scollAreaStyle}>
                                <Group>
                                    {guildFiles.map((file) => {
                                        return (
                                            <PlayControl
                                                key={file.id}
                                                file={file}
                                                playFunc={playFunc}
                                            />
                                        );
                                    })}
                                </Group>
                            </ScrollArea>
                        </Paper>
                    </Grid.Col>
                    <Grid.Col xs={3}>
                        <Group
                            direction="column"
                            className={classes.groupStyle}
                        >
                            <ControlsWindow guildId={guildId} />
                            <Paper
                                withBorder
                                shadow="sm"
                                p="sm"
                                className={classes.quickEnablePaper}
                            >
                                <Title
                                    title="Quick enable files"
                                    order={3}
                                    pb="xs"
                                >
                                    Quick enable files
                                </Title>
                                <ScrollArea>
                                    <Group></Group>
                                </ScrollArea>
                            </Paper>
                        </Group>
                    </Grid.Col>
                </Grid>
            )}
        </>
    );
}
