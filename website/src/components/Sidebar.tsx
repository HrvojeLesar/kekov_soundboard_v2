import {
    ActionIcon,
    Box,
    Center,
    createStyles,
    Divider,
    Menu,
    Navbar,
    ScrollArea,
    Skeleton,
    Stack,
    Tooltip,
    UnstyledButton,
    useMantineColorScheme,
} from "@mantine/core";
import { useContext, useEffect } from "react";
import { FaSignOutAlt, FaDiscord, FaGlobe, FaEllipsisH } from "react-icons/fa";
import { TbMoonStars, TbSun, TbUpload, TbX } from "react-icons/tb";
import { API_URL, AuthRoute, DISCORD_CND_USER_AVATAR } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import BaseSidebarButton, { baseSidebarButtonStyle } from "./BaseSidebarButton";
import GuildLinkButton from "./GuildLinkButton";

const useStyles = createStyles((theme) => ({
    navbarDivider: {
        marginTop: theme.spacing.xs,
        marginBottom: theme.spacing.xs,
        borderBottom: `3px solid ${
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

    noServers: {
        ...baseSidebarButtonStyle(theme),
        backgroundColor: theme.colors.red[8],

        "&:hover": {
            backgroundColor: theme.colors.red[9],
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

    sidebarTop: {
        marginLeft: "-10px",
        marginRight: "-10px",
        display: "flex",
        flexDirection: "column",
        paddingTop: theme.spacing.sm,
        gap: theme.spacing.xs,
    },
}));

export default function Sidebar() {
    const { user, guilds, fetchGuilds, isFetchingGuilds, logout } =
        useContext(AuthContext);
    const { colorScheme, toggleColorScheme } = useMantineColorScheme();
    const { classes } = useStyles();

    const spawnSkeletons = () => {
        let skeletons = [];
        for (let i = 0; i < 5; i++) {
            skeletons.push(<Skeleton key={i} height={45} circle mb="xs" />);
        }
        return skeletons;
    };

    const isColorSchemeDark = () => {
        return colorScheme === "dark";
    };

    useEffect(() => {
        fetchGuilds();
    }, [fetchGuilds]);

    return (
        <Navbar height="100vh" width={{ base: 80 }} p="sm">
            <Navbar.Section component={Box} className={classes.sidebarTop}>
                <BaseSidebarButton label="Your files" route="/user">
                    <img
                        className={classes.userImg}
                        src={DISCORD_CND_USER_AVATAR(
                            user?.id,
                            user?.avatar,
                            user?.discriminator
                        )}
                        alt={`${user?.username}'s profile`}
                    />
                </BaseSidebarButton>
                <BaseSidebarButton label="Upload" route="/upload">
                    <TbUpload size={24} />
                </BaseSidebarButton>
                <BaseSidebarButton label="Public Sounds" route="/public">
                    <FaGlobe size={24} />
                </BaseSidebarButton>
            </Navbar.Section>
            <Navbar.Section className={classes.navbarDivider}>
                {}
            </Navbar.Section>
            <Navbar.Section
                grow
                component={ScrollArea}
                offsetScrollbars
                scrollbarSize={0}
                mx="-xs"
            >
                <Stack align="center" spacing="xs">
                    {!isFetchingGuilds ? (
                        guilds.length > 0 ? (
                            guilds.map((guild) => {
                                return (
                                    <GuildLinkButton
                                        key={guild.id}
                                        guild={guild}
                                    />
                                );
                            })
                        ) : (
                            <Tooltip
                                label="You don't share any Discord servers with the bot, try inviting the bot to a server. Clicking here will redirect you to Discords bot invitation."
                                position="right"
                                color="red"
                                multiline
                                width={200}
                                withArrow
                                withinPortal
                            >
                                <UnstyledButton
                                    className={classes.noServers}
                                    component={"a"}
                                    href={`${API_URL}${AuthRoute.getBotInvite}`}
                                >
                                    <TbX size={32} />
                                </UnstyledButton>
                            </Tooltip>
                        )
                    ) : (
                        spawnSkeletons()
                    )}
                </Stack>
            </Navbar.Section>
            <Navbar.Section className={classes.navbarDivider}>
                {}
            </Navbar.Section>
            <Navbar.Section>
                <Center>
                    <Stack spacing="xs" className={classes.sidebarBottomGroup}>
                        <Tooltip
                            label="Invite bot to server"
                            position="right"
                            withArrow
                            withinPortal
                        >
                            <UnstyledButton
                                className={classes.botInviteButton}
                                component={"a"}
                                href={`${API_URL}${AuthRoute.getBotInvite}`}
                            >
                                <FaDiscord size={32} />
                            </UnstyledButton>
                        </Tooltip>
                        <Menu width={200}>
                            <Menu.Target>
                                <ActionIcon>
                                    <FaEllipsisH />
                                </ActionIcon>
                            </Menu.Target>
                            <Menu.Dropdown>
                                <Menu.Label>Options</Menu.Label>
                                <Menu.Item
                                    icon={
                                        isColorSchemeDark() ? (
                                            <TbSun size={14} color="yellow" />
                                        ) : (
                                            <TbMoonStars
                                                size={14}
                                                color="teal"
                                            />
                                        )
                                    }
                                    onClick={() => toggleColorScheme()}
                                >
                                    {isColorSchemeDark()
                                        ? "Switch to light mode"
                                        : "Switch to dark mode"}
                                </Menu.Item>
                                <Divider />
                                <Menu.Item
                                    color="red"
                                    icon={<FaSignOutAlt size={14} />}
                                    onClick={() => logout()}
                                >
                                    Logout
                                </Menu.Item>
                            </Menu.Dropdown>
                        </Menu>
                    </Stack>
                </Center>
            </Navbar.Section>
        </Navbar>
    );
}
