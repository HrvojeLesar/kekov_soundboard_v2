import { useContext, useEffect, useState } from "react";
import {
    Box,
    createStyles,
    LoadingOverlay,
    Paper,
    ScrollArea,
    Text,
    Title,
} from "@mantine/core";
import { CanceledError } from "axios";
import { AuthContext, COOKIE_NAMES } from "../../auth/AuthProvider";
import { GuildToggle } from "../GuildToggle";
import { useCookies } from "react-cookie";
import {
    ApiRequest,
    GuildsWithFile,
    LOADINGOVERLAY_ZINDEX,
    SoundFile,
} from "../../utils/utils";

type ServerSelectProps = {
    file?: SoundFile;
};

const useStyle = createStyles((_theme) => {
    return {
        quickServerEnableStyle: {
            flexGrow: 1,
            width: "100%",
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
            position: "relative",
            height: "calc(100vh - 34px)",
        },
    };
});

let abortController: AbortController | undefined = undefined;

export default function ServerSelect({ file }: ServerSelectProps) {
    const [cookies] = useCookies(COOKIE_NAMES);
    const { guilds: globalGuilds } = useContext(AuthContext);
    const [guilds, setGuilds] = useState<GuildsWithFile[]>([]);
    const [isFetchingGuilds, setIsFetchingGuilds] = useState(false);

    const { classes } = useStyle();

    useEffect(() => {
        const fetchGuilds = async () => {
            if (cookies.access_token && file) {
                try {
                    abortController = new AbortController();
                    const { data } = await ApiRequest.fetchGuildsWithFile(
                        file.id,
                        abortController,
                        cookies.access_token
                    );
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

        if (file) {
            handleFetch();
        }
    }, [file, cookies.access_token, globalGuilds]);

    return (
        <Paper
            withBorder
            shadow="sm"
            p="sm"
            className={classes.quickServerEnableStyle}
        >
            <Title order={3} pb="xs">
                Servers
            </Title>
            <LoadingOverlay
                zIndex={LOADINGOVERLAY_ZINDEX}
                visible={isFetchingGuilds}
            />
            {file !== undefined ? (
                <ScrollArea>
                    {guilds.length > 0 ? (
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
                    ) : (
                        <Text size="xl" weight="bold">
                            You don't share any server with bot.
                        </Text>
                    )}
                </ScrollArea>
            ) : (
                <Text size="xl" weight="bold">
                    Select a file to display servers.
                </Text>
            )}
        </Paper>
    );
}
