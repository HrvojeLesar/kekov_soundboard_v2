import { Button, Group, Stack, Text, TextInput } from "@mantine/core";
import axios from "axios";
import { useContext, useEffect, useState } from "react";
import { API_URL, UserRoute } from "../api/ApiRoutes";
import { AuthContext, Guild } from "../auth/AuthProvider";
import { GuildToggle } from "./GuildToggle";
import { UserFile } from "../views/UserFiles";

type AddModalBodyProps = {
    file: UserFile;
};

type GuildsWithFile = {
    guild: Guild;
    has_file: boolean;
};

export default function AddModalBody({ file }: AddModalBodyProps) {
    const { tokens } = useContext(AuthContext);
    const [guilds, setGuilds] = useState<GuildsWithFile[]>([]);

    const fetchGuilds = async () => {
        if (tokens?.access_token) {
            try {
                const { data } = await axios.get<GuildsWithFile[]>(
                    `${API_URL}${UserRoute.getGuildsWithFile}${file?.id}`,
                    {
                        headers: { authorization: `${tokens.access_token}` },
                    }
                );
                console.log(data);
                setGuilds(data);
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    useEffect(() => {
        fetchGuilds();
    }, []);

    return (
        <>
        </>
    );
}
