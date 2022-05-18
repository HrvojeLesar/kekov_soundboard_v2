import {
    Box,
    Button,
    Card,
    Divider,
    Grid,
    Group,
    List,
    Modal,
    Paper,
    ScrollArea,
    SimpleGrid,
    Skeleton,
    Text,
} from "@mantine/core";
import { useViewportSize } from "@mantine/hooks";
import axios from "axios";
import { useContext, useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import { FixedSizeList } from "react-window";
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
    const [isModalOpen, setIsModalOpen] = useState(false);

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

    const addFile = (file: UserFile) => {
        // WARN: owner not set
        setGuildFiles([
            ...guildFiles,
            { id: file.id, display_name: file.display_name },
        ]);
    };

    const removeFile = (file: UserFile) => {
        setGuildFiles(guildFiles.filter((f) => f.id !== file.id));
    };

    useEffect(() => {
        fetchGuildFiles();
    }, [guildId]);

    // const Row = ({ index, style }: { index: number, style: any }) => {
    //     return (
    //         <Box style={style}>
    //             <PlayControl
    //                 file={guildFiles[index]}
    //                 playFunc={playFunc}
    //                 key={guildFiles[index].id}
    //             />
    //         </Box>
    //     );
    // };
    //                 <FixedSizeList
    //                     height={height - 35}
    //                     width="100%"
    //                     itemCount={guildFiles.length}
    //                     itemSize={80}
    //                 >
    //                     {Row}
    //                 </FixedSizeList>

    // const { height } = useViewportSize();

    return (
        <>
            <Modal opened={isModalOpen} onClose={() => setIsModalOpen(false)}>
                <GuildAddFileModalBody
                    addFileCallback={(file) => addFile(file)}
                    removeFileCallback={(file) => removeFile(file)}
                    guildId={guildId ?? "0"}
                />
            </Modal>
            <Button onClick={() => setIsModalOpen(true)}>Open modal</Button>
            <Grid>
                <Grid.Col span={8}>
                    <Paper
                        p="sm"
                        component={ScrollArea}
                        withBorder
                        // TODO: Flexaj boxe
                        style={{
                            height: "calc(100vh - 32px)",
                        }}
                        offsetScrollbars
                    >
                        <Group position="center" >
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
