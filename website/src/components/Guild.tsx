import axios from "axios";
import { useContext, useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { API_URL, ControlsRoute, GuildRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import { PlayControl } from "./PlayControl";

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
    let { guildId } = useParams();
    let { tokens } = useContext(AuthContext);
    let [guildFiles, setGuildFiles] = useState<GuildFile[]>([]);

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
                let payload: PlayPayload = { guild_id: guildId, file_id: fileId }
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
            {guildFiles.map((file) => {
                return (
                    <PlayControl
                        file={file}
                        playFunc={playFunc}
                        key={file.id}
                    />
                );
            })}
            <div>{guildId}</div>
        </>
    );
}
