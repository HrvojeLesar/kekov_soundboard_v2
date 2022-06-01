import { useContext, useEffect, useState } from "react";
import { Box } from "@mantine/core";
import { CanceledError } from "axios";
import { AuthContext, COOKIE_NAMES } from "../../auth/AuthProvider";
import { GuildToggle } from "../GuildToggle";
import { useCookies } from "react-cookie";
import { ApiRequest, GuildsWithFile, UserFile } from "../../utils/utils";

type ServerSelectProps = {
    file: UserFile;
};

let abortController: AbortController | undefined = undefined;

export default function ServerSelect({ file }: ServerSelectProps) {
    const [cookies] = useCookies(COOKIE_NAMES);
    const { guilds: globalGuilds } = useContext(AuthContext);
    const [guilds, setGuilds] = useState<GuildsWithFile[]>([]);
    const [isFetchingGuilds, setIsFetchingGuilds] = useState(false);

    const fetchGuilds = async () => {
        if (cookies.access_token) {
            try {
                abortController = new AbortController();
                const { data } = await ApiRequest.fetchGuildsWithFile(file.id, abortController, cookies.access_token);
                setGuilds(
                    data.map((guild) => {
                        const globalGuild = globalGuilds.find(
                            (g) => g.id === guild.guild.id
                        );
                        if (globalGuild) {
                            guild.guild.icon = globalGuild.icon;
                            guild.guild.icon_hash = globalGuild.icon_hash;
                        }
                        return guild;
                    })
                );
                setIsFetchingGuilds(false);
            } catch (e) {
                // TODO: Handle
                if (e instanceof CanceledError) {
                    return;
                } else {
                    setIsFetchingGuilds(false);
                }

            }
        }
    };

    const handleFetch = async () => {
        abortController?.abort();
        setIsFetchingGuilds(true);
        await fetchGuilds();
    };

    useEffect(() => {
        handleFetch();
    }, [file]);

    return (
        <>
            {isFetchingGuilds ? (
                <>Loading...</>
            ) : (
                guilds.map(({ guild, has_file }, index) => {
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
                })
            )}
        </>
    );
}
