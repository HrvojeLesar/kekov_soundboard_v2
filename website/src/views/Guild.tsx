import {
    Button,
    Grid,
    Group,
    Modal,
    Paper,
    ScrollArea,
    Title,
} from "@mantine/core";
import axios from "axios";
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
import Channels from "../components/Channels";
import ControlsWindow from "../components/Guild/ControlsWindow";
import GuildAddFileModalBody from "../components/GuildAddFileModalBody";
import { PlayControl } from "../components/PlayControl";
import Queue from "../components/Queue";
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

export function Guild() {
    const { guildId } = useParams();
    const [cookies] = useCookies(COOKIE_NAMES);
    const [guildFiles, setGuildFiles] = useState<GuildFile[]>([]);
    const [userFiles, setUserFiles] = useState<UserFile[]>([]);

    const fetchGuildFiles = async () => {
        if (cookies?.access_token) {
            try {
                let { data } = await axios.get<GuildFile[]>(
                    `${API_URL}${GuildRoute.getGuildSounds}${guildId}`,
                    {
                        headers: {
                            Authorization: `${cookies.access_token}`,
                        },
                    }
                );
                setGuildFiles(data);
            } catch (e) {
                // TODO: Handle
                console.log(e);
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
        fetchGuildFiles();
    }, [guildId]);

    // useEffect(() => {
    //     fetchUserFiles();
    // }, []);

    return (
        <>
            <Grid>
                <Grid.Col xs={9}>
                    <Paper
                        withBorder
                        shadow="sm"
                        p="sm"
                        style={{
                            display: "flex",
                            flexDirection: "column",
                            overflow: "hidden",
                            ...guildMaximumWindowHeight,
                        }}
                    >
                        <Title title="Server sounds" order={3} pb="xs">
                            Server sounds
                        </Title>
                        <ScrollArea style={{ height: "100%" }}>
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
                        style={{ width: "100%", ...guildMaximumWindowHeight }}
                    >
                        <ControlsWindow guildId={guildId} />
                        <Paper
                            withBorder
                            shadow="sm"
                            p="sm"
                            style={{
                                flexGrow: 1,
                                width: "100%",
                                display: "flex",
                                flexDirection: "column",
                                overflow: "hidden",
                            }}
                        >
                            <Title title="Quick enable files" order={3} pb="xs">
                                Quick enable files
                            </Title>
                            <ScrollArea>
                                <Group></Group>
                            </ScrollArea>
                        </Paper>
                    </Group>
                </Grid.Col>
            </Grid>
        </>
    );
}
