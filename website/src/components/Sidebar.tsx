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
import { useContext, useEffect } from "react";
import { Logout, Upload } from "tabler-icons-react";
import { Icon } from "@iconify/react";
import discordIcon from "@iconify/icons-simple-icons/discord";
import { API_URL, AuthRoute, DISCORD_CND_USER_AVATAR } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import BaseSidebarButton, { baseSidebarButtonStyle } from "./BaseSidebarButton";
import GuildLinkButton from "./GuildLinkButton";

const useStyles = createStyles((theme) => ({
    navbarHeader: {
        padding: theme.spacing.xs,
        borderBottom: `3px solid ${
            theme.colorScheme === "dark"
                ? theme.colors.dark[5]
                : theme.colors.gray[3]
        }`,
    },

    navbarFooter: {
        padding: theme.spacing.xs,
        borderTop: `3px solid ${
            theme.colorScheme === "dark"
                ? theme.colors.dark[5]
                : theme.colors.gray[3]
        }`,
    },

    botInviteButton: {
        ...baseSidebarButtonStyle(theme),
        backgroundColor: "#5865f2",

        "&:hover": {
            backgroundColor: "#5865f2",
            borderRadius: "40%",
            transition: ".2s",
        },
    },

    logoutButton: {
        ...baseSidebarButtonStyle(theme),
        backgroundColor: theme.colors.red[5],
        height: "25px",
        width: "25px",

        "&:hover": {
            backgroundColor: theme.colors.red[8],
            borderRadius: "40%",
            transition: ".2s",
        },
    },

    userImg: {
        width: "100%",
        height: "100%",
    },

    sidebarBottomGroup: {
        alignItems: "center",
    },
}));

export default function Sidebar() {
    const { user, guilds, fetchGuilds, logout } = useContext(AuthContext);
    const { classes } = useStyles();

    const spawnSkeletons = () => {
        let skeletons = [];
        for (let i = 0; i < 5; i++) {
            skeletons.push(<Skeleton key={i} height={45} circle mb="xs" />);
        }
        return skeletons;
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
        <Navbar height="100vh" width={{ base: 80 }} p="sm">
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
                                alt=""
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
                my="xs"
                component={ScrollArea}
                offsetScrollbars
                scrollbarSize={0}
                mx="-xs"
            >
                <Group direction="column" align="center" spacing="xs">
                    {/*TODO: Handle a situation when there is no guilds to show*/}
                    {guilds.length > 0 ? renderGuilds() : spawnSkeletons()}
                </Group>
            </Navbar.Section>
            <Navbar.Section className={classes.navbarFooter}>
                <Center>
                    <Group
                        direction="column"
                        className={classes.sidebarBottomGroup}
                    >
                        <Tooltip
                            label="Invite bot to server"
                            position="right"
                            withArrow
                        >
                            <UnstyledButton
                                className={classes.botInviteButton}
                                component={"a"}
                                href={`${API_URL}${AuthRoute.getBotInvite}`}
                            >
                                <Icon width="70%" icon={discordIcon} />
                            </UnstyledButton>
                        </Tooltip>
                        <Tooltip label="Logout" position="right" withArrow>
                            <UnstyledButton
                                onClick={() => logout()}
                                className={classes.logoutButton}
                            >
                                <Logout />
                            </UnstyledButton>
                        </Tooltip>
                    </Group>
                </Center>
            </Navbar.Section>
        </Navbar>
    );
}
