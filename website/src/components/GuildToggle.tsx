import {
    Group,
    Text,
    Switch,
    Image,
    UnstyledButton,
    createStyles,
    LoadingOverlay,
} from "@mantine/core";
import { useState } from "react";
import { useCookies } from "react-cookie";
import { COOKIE_NAMES, Guild } from "../auth/AuthProvider";
import { ApiRequest, nameToInitials, primaryShade, UserFile } from "../utils/utils";

type GuildToggleProps = {
    guild: Guild;
    file: UserFile;
    hasFile: boolean;
    toggleCallback: (state: boolean) => void;
};

const useStyles = createStyles((theme, { checked }: { checked: boolean }) => {
    const shade = primaryShade(theme);
    return {
        button: {
            display: "flex",
            position: "relative",
            alignItems: "center",
            width: "100%",
            transition: "background-color 150ms ease, border-color 150ms ease",
            border: `1px solid ${
                checked
                    ? theme.colors[theme.primaryColor][shade]
                    : theme.colorScheme === "dark"
                    ? theme.colors.dark[shade]
                    : theme.colors.gray[shade]
            }`,
            borderRadius: theme.radius.sm,
            padding: theme.spacing.sm,
            backgroundColor: checked
                ? theme.colorScheme === "dark"
                    ? theme.fn.rgba(
                          theme.colors[theme.primaryColor][shade],
                          0.3
                      )
                    : theme.colors[theme.primaryColor][shade]
                : theme.colorScheme === "dark"
                ? theme.colors.dark[8]
                : theme.white,
        },

        image: {
            border: `1px solid ${
                checked
                    ? theme.colors[theme.primaryColor][shade]
                    : theme.colorScheme === "dark"
                    ? theme.colors.gray[shade]
                    : theme.colors.dark[shade]
            }`,
            borderRadius: "50%",
            width: "42px",
            height: "42px",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            overflow: "hidden",
        },

        groupStyle: {
            flexGrow: 1,
        },

        textStyle: {
            textOverflow: "ellipsis",
            overflow: "hidden",
            whiteSpace: "nowrap",
        },
    };
});

export function GuildToggle({
    guild,
    hasFile,
    file,
    toggleCallback,
}: GuildToggleProps) {
    const [cookies] = useCookies(COOKIE_NAMES);
    const [isUpdating, setIsUpdating] = useState(false);
    const { classes } = useStyles({ checked: hasFile });

    const handleToggle = async (state: boolean) => {
        setIsUpdating(true);
        try {
            if (state) {
                await addToGuild();
            } else {
                await removeFromGuild();
            }
            toggleCallback(state);
        } catch (e) {
            // WARN: can desync state
            // post or delete gets applied on server
            // but client returns connection error
            // before server sends back the response
            console.log(e);
            toggleCallback(!state);
        } finally {
            setIsUpdating(false);
        }
    };

    const addToGuild = async () => {
        await ApiRequest.addFileToGuild(
            guild.id,
            file.id,
            cookies.access_token
        );
    };

    const removeFromGuild = async () => {
        await ApiRequest.removeFileFromGuild(
            guild.id,
            file.id,
            cookies.access_token
        );
    };

    return (
        <UnstyledButton
            className={classes.button}
            onClick={() => {
                handleToggle(!hasFile);
            }}
        >
            <LoadingOverlay visible={isUpdating} />
            <Group position="apart" className={classes.groupStyle} noWrap>
                <Group>
                    {guild.icon ? (
                        <Image
                            className={classes.image}
                            radius="xl"
                            src={`https://cdn.discordapp.com/icons/${guild.id}/${guild.icon}`}
                        />
                    ) : (
                        <Text className={classes.image} weight="bold">
                            {nameToInitials(guild.name)}
                        </Text>
                    )}
                    <Text
                        title={guild.name}
                        className={classes.textStyle}
                        lineClamp={1}
                    >
                        {guild.name}
                    </Text>
                </Group>
                <Switch
                    checked={hasFile}
                    size="lg"
                    onLabel="ON"
                    offLabel="OFF"
                    styles={{ input: { cursor: "pointer" } }}
                    readOnly
                />
            </Group>
        </UnstyledButton>
    );
}
