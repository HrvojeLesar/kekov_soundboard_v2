import {
    Box,
    Center,
    createStyles,
    LoadingOverlay,
    Paper,
    ScrollArea,
    Title,
} from "@mantine/core";
import { MdVolumeUp } from "react-icons/md";
import { useEffect, useState } from "react";
import useWebSocket from "react-use-websocket";
import { DISCORD_CND_USER_AVATAR, WEBSOCKET_URL } from "../../api/ApiRoutes";
import { LOADINGOVERLAY_ZINDEX, primaryShade } from "../../utils/utils";

type DiscordChannelsWindowProps = {
    guildId: string;
};

type User = {
    id: string;
    discriminator: string;
    avatar_hash?: string;
    nickname?: string;
    username: string;
};

type Channel = {
    id: string;
    channel_name: string;
    users: User[];
};

type ChannelsResponse = {
    channels: Record<string, Channel>;
};

const useStyle = createStyles((theme) => {
    const shade = primaryShade(theme);
    return {
        paperStyle: {
            width: "100%",
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
            flexGrow: 1,
            position: "relative",
            minHeight: "30%",
        },
        channelTitleStyle: {
            display: "flex",
            alignItems: "center",

            "&:hover": {
                cursor: "default",
                transition: "150ms ease",
                borderRadius: "3px",
                backgroundColor:
                    theme.colorScheme === "dark"
                        ? theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          )
                        : theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          ),
            },
        },
        usersStyle: {
            marginLeft: 24 + theme.spacing.xs,
            display: "flex",
            alignItems: "center",
            gap: 2,
            paddingTop: 5,
            paddingBottom: 5,

            "&:hover": {
                cursor: "default",
                transition: "150ms ease",
                borderRadius: "3px",
                backgroundColor:
                    theme.colorScheme === "dark"
                        ? theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          )
                        : theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          ),
            },
        },
        volumeIconStyle: {
            paddingRight: theme.spacing.xs,
        },
        imgStyle: {
            borderRadius: "50%",
            width: 24,
            height: 24,
        },
    };
});

type DiscordChannelProps = {
    channel: Channel;
};

function DiscordChannel({ channel }: DiscordChannelProps) {
    const { classes } = useStyle();
    return (
        <Box mb="xs">
            <Title order={4} className={classes.channelTitleStyle}>
                <MdVolumeUp size={24} className={classes.volumeIconStyle} />
                {channel.channel_name}
            </Title>
            {channel.users.map((user) => {
                return (
                    <Box key={user.id} className={classes.usersStyle}>
                        <img
                            className={classes.imgStyle}
                            src={DISCORD_CND_USER_AVATAR(
                                user.id,
                                user.avatar_hash,
                                user.discriminator
                            )}
                            alt={`${user?.username}'s profile image`}
                        />
                        {user.nickname ?? user.username}
                    </Box>
                );
            })}
        </Box>
    );
}

export default function DiscordChannelsWindow({
    guildId,
}: DiscordChannelsWindowProps) {
    const [channels, setChannels] = useState<Channel[] | undefined>(undefined);
    const { sendJsonMessage, lastJsonMessage, lastMessage } =
        useWebSocket(WEBSOCKET_URL);

    const { classes } = useStyle();

    useEffect(() => {
        sendJsonMessage({ guild_id: guildId });
    }, [guildId]);

    useEffect(() => {
        if (lastJsonMessage === null) {
            return;
        }
        let newChannels = lastJsonMessage as ChannelsResponse;
        if (newChannels.channels) {
                const sorted = Object.keys(newChannels.channels)
                    .map((o) => {
                        return newChannels.channels[o];
                    })
                    .filter((channel) => {
                        if (channel.users.length > 0) {
                            channel.users.sort((a, b) => {
                                const usera = a.nickname ?? a.username;
                                const userb = b.nickname ?? b.username;
                                return usera === userb
                                    ? 0
                                    : usera > userb
                                    ? 1
                                    : -1;
                            });
                            return true;
                        }
                        return false;
                    })
                    .sort((a, b) => {
                        return a.channel_name === b.channel_name
                            ? 0
                            : a.channel_name > b.channel_name
                            ? 1
                            : -1;
                    });
                    console.log(sorted);
            setChannels(sorted);
        } else {
            console.log(lastMessage?.data);
            setChannels(undefined);
        }
    }, [lastJsonMessage]);

    return (
        <Paper withBorder shadow="sm" p="sm" className={classes.paperStyle}>
            <LoadingOverlay zIndex={LOADINGOVERLAY_ZINDEX} visible={false} />
            <Title title="Quick enable files" order={3} pb="xs">
                Live voice channel preview
            </Title>
            <ScrollArea>
                {channels !== undefined ? (
                    channels.length > 0 ? (
                        channels.map((channel) => {
                            return (
                                <DiscordChannel
                                    key={channel.id}
                                    channel={channel}
                                />
                            );
                        })
                    ) : (
                        <Box>All channels are empty</Box>
                    )
                ) : (
                    <Box>Disconnected... Try refreshing</Box>
                )}
            </ScrollArea>
        </Paper>
    );
}
