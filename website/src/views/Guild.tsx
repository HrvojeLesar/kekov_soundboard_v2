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
import { useContext, useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { API_URL, ControlsRoute, GuildRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import Channels from "../components/Channels";
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

export function Guild() {
    const { guildId } = useParams();
    const { tokens } = useContext(AuthContext);
    const [guildFiles, setGuildFiles] = useState<GuildFile[]>([]);

    const fetchGuildFiles = async () => {
        if (tokens?.access_token) {
            try {
                let { data } = await axios.get<GuildFile[]>(
                    `${API_URL}${GuildRoute.getGuildSounds}${guildId}`,
                    {
                        headers: {
                            Authorization: `${tokens.access_token}`,
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

    const playFunc = async (fileId: string) => {
        if (tokens?.access_token && guildId) {
            try {
                let payload: PlayPayload = {
                    guild_id: guildId,
                    file_id: fileId,
                };
                await axios.post<PlayPayload>(
                    `${API_URL}${ControlsRoute.postPlay}`,
                    payload,
                    { headers: { Authorization: `${tokens.access_token}` } }
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

    return (
        <>
            <Grid>
                <Grid.Col span={8}>
                    <Paper
                        withBorder
                        shadow="sm"
                        p="sm"
                        style={{
                            height: "calc(100vh - 80px)",
                            display: "flex",
                            flexDirection: "column",
                            overflow: "hidden",
                        }}
                    >
                        <Title order={3} pb="xs">
                            Title
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
                <Grid.Col span={4}>
                    <Group direction="column">
                        <Queue />
                        <Channels />
                    </Group>
                </Grid.Col>
            </Grid>
        </>
    );
}
