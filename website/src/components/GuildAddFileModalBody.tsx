import { Avatar, Group, Switch, Text } from "@mantine/core";
import axios from "axios";
import { useContext, useEffect, useState } from "react";
import { API_URL, GuildRoute, UserRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import { UserFile } from "../views/UserFiles";
import { FileToggle } from "./FileToggle";

type GuildAddFileModalBodyPropsType = {
    addFileCallback: (file: UserFile) => void;
    removeFileCallback: (file: UserFile) => void;
    guildId: string;
};

type EnabledFile = {
    sound_file: UserFile;
    enabled: boolean;
};

export default function GuildAddFileModalBody({
    guildId,
    addFileCallback,
    removeFileCallback,
}: GuildAddFileModalBodyPropsType) {
    const { tokens } = useContext(AuthContext);
    const [files, setFiles] = useState<EnabledFile[]>([]);

    const fetchFiles = async () => {
        if (tokens?.access_token) {
            try {
                const { data } = await axios.get<EnabledFile[]>(
                    `${API_URL}${UserRoute.getEnabledFiles}${guildId}`,
                    {
                        headers: { authorization: `${tokens.access_token}` },
                    }
                );
                console.log(data);
                setFiles(data);
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    const addToGuild = async (file: UserFile) => {
        await axios.post(
            `${API_URL}${GuildRoute.postAddSound}${guildId}/${file.id}`,
            {},
            { headers: { authorization: `${tokens?.access_token}` } }
        );
        addFileCallback(file);
        return;
    };

    const removeFromGuild = async (file: UserFile) => {
        await axios.delete(
            `${API_URL}${GuildRoute.postAddSound}${guildId}/${file.id}`,
            { headers: { authorization: `${tokens?.access_token}` } }
        );
        removeFileCallback(file);
        return;
    };

    useEffect(() => {
        fetchFiles();
    }, []);

    return (
        <>
            {files.map((file) => {
                return (
                    <FileToggle
                        key={file.sound_file.id}
                        file={file.sound_file}
                        isActive={file.enabled}
                        addCallback={(file) => addToGuild(file)}
                        removeCallback={(file) => removeFromGuild(file)}
                    />
                );
            })}
        </>
    );
}
