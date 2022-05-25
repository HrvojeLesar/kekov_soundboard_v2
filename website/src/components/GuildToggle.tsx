import {
    Group,
    Text,
    Switch,
    Image,
    UnstyledButton,
    createStyles,
} from "@mantine/core";
import axios from "axios";
import { useContext, useState } from "react";
import { useCookies } from "react-cookie";
import { API_URL, GuildRoute } from "../api/ApiRoutes";
import { AuthContext, COOKIE_NAMES, Guild } from "../auth/AuthProvider";
import { nameToInitials } from "../utils/utils";
import { UserFile } from "../views/UserFiles";

type GuildToggleProps = {
    guild: Guild;
    file: UserFile;
    hasFile: boolean;
    toggleCallback: (state: boolean) => void;
};

const useStyles = createStyles((theme, { checked }: { checked: boolean }) => {
    return {
        button: {
            display: "flex",
            alignItems: "center",
            width: "100%",
            transition: "background-color 150ms ease, border-color 150ms ease",
            border: `1px solid ${
                checked
                    ? theme.colors[theme.primaryColor][
                          theme.colorScheme === "dark" ? 9 : 6
                      ]
                    : theme.colorScheme === "dark"
                    ? theme.colors.dark[8]
                    : theme.colors.gray[3]
            }`,
            borderRadius: theme.radius.sm,
            padding: theme.spacing.sm,
            backgroundColor: checked
                ? theme.colorScheme === "dark"
                    ? theme.fn.rgba(theme.colors[theme.primaryColor][8], 0.3)
                    : theme.colors[theme.primaryColor][0]
                : theme.colorScheme === "dark"
                ? theme.colors.dark[8]
                : theme.white,
        },

        image: {
            border: `1px solid ${
                checked
                    ? theme.colors[theme.primaryColor][
                          theme.colorScheme === "dark" ? 9 : 6
                      ]
                    : theme.colorScheme === "dark"
                    ? theme.colors.dark[8]
                    : theme.colors.gray[3]
            }`,
            borderRadius: "50%",
            width: "42px",
            height: "42px",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            overflow: "hidden",
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
        await axios.post(
            `${API_URL}${GuildRoute.postAddSound}${guild.id}/${file.id}`,
            {},
            { headers: { authorization: `${cookies.access_token}` } }
        );
    };

    const removeFromGuild = async () => {
        await axios.delete(
            `${API_URL}${GuildRoute.postAddSound}${guild.id}/${file.id}`,
            { headers: { authorization: `${cookies.access_token}` } }
        );
    };

    return (
        <UnstyledButton
            className={classes.button}
            onClick={() => {
                if (!isUpdating) {
                    handleToggle(!hasFile);
                }
            }}
        >
            <Group position="apart" style={{ flexGrow: 1 }} noWrap>
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
                        style={{
                            textOverflow: "ellipsis",
                            overflow: "hidden",
                            whiteSpace: "nowrap",
                        }}
                        lineClamp={1}
                    >
                        {guild.name}
                    </Text>
                </Group>
                <Switch
                    disabled={isUpdating}
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
