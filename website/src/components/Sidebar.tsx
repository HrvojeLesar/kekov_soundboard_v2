import {
    Center,
    createStyles,
    Group,
    Navbar,
    ScrollArea,
    Skeleton,
    Tooltip,
    UnstyledButton,
} from "@mantine/core";
import axios from "axios";
import { useContext, useEffect, useState } from "react";
import { Upload } from "tabler-icons-react";
import { Icon } from "@iconify/react"
import discordIcon from "@iconify/icons-simple-icons/discord";
import { API_URL, DISCORD_CND_USER_AVATAR, UserRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import BaseSidebarButton from "./BaseSidebarButton";
import GuildLinkButton from "./GuildLinkButton";

const useStyles = createStyles((theme) => ({
    navbarHeader: {
        padding: theme.spacing.xs,
        borderBottom: `3px solid ${theme.colorScheme === "dark"
                ? theme.colors.dark[4]
                : theme.colors.gray[3]
            }`,
    },

    navbarFooter: {
        padding: theme.spacing.xs,
        borderTop: `3px solid ${theme.colorScheme === "dark"
                ? theme.colors.dark[4]
                : theme.colors.gray[3]
            }`,
    },

    botInviteButton: {
        height: "48px",
        width: "48px",
        color: theme.colors.gray[0],
        backgroundColor: "#5865f2",
        borderRadius: "50%",
        overflow: "hidden",
        display: "flex",
        textAlign: "center",
        alignItems: "center",
        justifyContent: "center",

        "&:hover": {
            backgroundColor: "#5865f2",
            borderRadius: "40%",
            transition: ".2s",
        },
    },

    userImg: {
        width: "100%",
        height: "100%",
    },
}));

export type Guild = {
    id: string;
    name: string;
    icon?: string;
    icon_hash?: string;
};

export default function Sidebar() {
    let { tokens, user } = useContext(AuthContext);
    const [guilds, setGuilds] = useState<Guild[]>([]);
    const { classes } = useStyles();

    const spawnSkeletons = () => {
        let skeletons = [];
        for (let i = 0; i < 5; i++) {
            skeletons.push(<Skeleton key={i} height={50} circle mb="xl" />);
        }
        return skeletons;
    };

    const fetchGuilds = async () => {
        try {
            if (tokens) {
                let { data } = await axios.get<Guild[]>(
                    `${API_URL}${UserRoute.getGuilds}`,
                    {
                        headers: {
                            Authorization: `${tokens.access_token}`,
                        },
                    }
                );
                console.log(data);
                setGuilds(data);
            }
        } catch (e) {
            // TODO: HANDLE
            console.log(e);
        }
    };

    const renderGuilds = () => {
        return guilds.map((guild) => {
            return <GuildLinkButton key={guild.id} guild={guild} />;
        });
    };

    useEffect(() => {
        fetchGuilds();
    }, []);

    return (
        <Navbar height="100vh" width={{ base: 80 }} p="md">
            <Navbar.Section className={classes.navbarHeader}>
                <Center>
                    <Group direction="column">
                        <BaseSidebarButton label="User files" route="/user">
                            <img
                                className={classes.userImg}
                                src={DISCORD_CND_USER_AVATAR(
                                    user?.id,
                                    user?.avatar,
                                    user?.discriminator
                                )}
                            />
                        </BaseSidebarButton>
                        <BaseSidebarButton label="Upload" route="/upload">
                            <Upload />
                        </BaseSidebarButton>
                    </Group>
                </Center>
            </Navbar.Section>
            <Navbar.Section
                grow
                mt={50}
                component={ScrollArea}
                offsetScrollbars
                scrollbarSize={0}
                mx="-xs"
            >
                <Group direction="column" align="center" spacing={2}>
                    {guilds.length > 0 ? renderGuilds() : spawnSkeletons()}
                </Group>
            </Navbar.Section>
            <Navbar.Section className={classes.navbarFooter}>
                <Center>
                    <Group direction="column">
                        <Tooltip
                            label="Invite bot to server"
                            position="right"
                            withArrow
                        >
                            <UnstyledButton
                                className={classes.botInviteButton}
                                component={"a"}
                                href="http://localhost:8080/v1/auth/botinvite"
                            >
                                <Icon width="70%" icon={discordIcon} />
                            </UnstyledButton>
                        </Tooltip>
                    </Group>
                </Center>
            </Navbar.Section>
        </Navbar>
    );
}
