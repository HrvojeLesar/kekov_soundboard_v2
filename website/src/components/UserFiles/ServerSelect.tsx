import { useContext, useEffect, useState } from "react";
import { Box } from "@mantine/core";
import axios from "axios";
import { AuthContext, COOKIE_NAMES, Guild } from "../../auth/AuthProvider";
import { UserFile } from "../../views/UserFiles";
import { API_URL, UserRoute } from "../../api/ApiRoutes";
import { GuildToggle } from "../GuildToggle";
import { useCookies } from "react-cookie";

type GuildsWithFile = {
    guild: Guild;
    has_file: boolean;
};

type ServerSelectProps = {
    file: UserFile;
};

export default function ServerSelect({ file }: ServerSelectProps) {
    const [cookies] = useCookies(COOKIE_NAMES);
    const [guilds, setGuilds] = useState<GuildsWithFile[]>([]);

    const fetchGuilds = async () => {
        if (cookies.access_token) {
            try {
                const { data } = await axios.get<GuildsWithFile[]>(
                    `${API_URL}${UserRoute.getGuildsWithFile}${file?.id}`,
                    {
                        headers: { authorization: `${cookies.access_token}` },
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
    }, [file]);

    return (
        <>
            {guilds.map(({ guild, has_file }, index) => {
                return (
                    <Box m="sm" key={guild.id}>
                        <GuildToggle
                            toggleCallback={(state) => {
                                guilds[index].has_file = state;
                                setGuilds([...guilds]);
                            }}
                            file={file}
                            guild={guild}
                            hasFile={has_file}
                        />
                    </Box>
                );
            })}
        </>
    );
}
