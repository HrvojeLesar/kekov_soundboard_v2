import {
    Box,
    createStyles,
    LoadingOverlay,
    Paper,
    ScrollArea,
    Title,
} from "@mantine/core";
import { MdVolumeUp } from "react-icons/md";
import { useEffect, useState } from "react";
import useWebSocket, { ReadyState } from "react-use-websocket";
import { DISCORD_CND_USER_AVATAR, WEBSOCKET_URL } from "../../api/ApiRoutes";
import { LOADINGOVERLAY_ZINDEX, primaryShade } from "../../utils/utils";
import { useCookies } from "react-cookie";
import { COOKIE_NAMES } from "../../auth/AuthProvider";

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
            paddingTop: "5px",
            paddingBottom: "5px",

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
            marginLeft: 24,
            paddingLeft: theme.spacing.xs,
            display: "flex",
            alignItems: "center",
            gap: 2,
            paddingTop: "5px",
            paddingBottom: "5px",

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
                            alt={`${user?.username}'s profile`}
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
    const { sendJsonMessage, lastJsonMessage, lastMessage, readyState } =
        useWebSocket(WEBSOCKET_URL);
    const [cookies] = useCookies(COOKIE_NAMES);

    const [channels, setChannels] = useState<Channel[] | undefined>(undefined);
    const [isIdentified, setIsIdentified] = useState(false);

    const { classes } = useStyle();

    useEffect(() => {
        if (isIdentified) {
            sendJsonMessage({ op: "Subscribe", guild_id: guildId });
        }
    }, [guildId, sendJsonMessage, isIdentified]);

    useEffect(() => {
        if (readyState === ReadyState.OPEN) {
            sendJsonMessage({
                op: "Identify",
                access_token: cookies.access_token,
            });
        }
    }, [readyState, cookies.access_token, sendJsonMessage]);

    useEffect(() => {
        switch (lastMessage?.data) {
            case "Identified": {
                setIsIdentified(true);
                break;
            }
            case "Reidentify": {
                sendJsonMessage({
                    op: "Identify",
                    access_token: cookies.access_token,
                });
                break;
            }
        }
    }, [lastMessage, cookies.access_token, sendJsonMessage]);

    useEffect(() => {
        if (
            lastJsonMessage === null ||
            !(lastJsonMessage as ChannelsResponse)?.channels
        ) {
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
                            return usera === userb ? 0 : usera > userb ? 1 : -1;
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
            setChannels(sorted);
        } else {
            setChannels(undefined);
        }
    }, [lastJsonMessage]);

    return (
        <Paper withBorder shadow="sm" p="sm" className={classes.paperStyle}>
            <LoadingOverlay
                zIndex={LOADINGOVERLAY_ZINDEX}
                visible={readyState === ReadyState.CONNECTING}
            />
            <Title title="Quick enable files" order={3} pb="xs">
                Live voice channel preview
            </Title>
            <ScrollArea>
                {readyState !== ReadyState.CLOSED ? (
                    channels && channels.length > 0 ? (
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
