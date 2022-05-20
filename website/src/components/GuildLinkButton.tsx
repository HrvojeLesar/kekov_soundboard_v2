import { createStyles, Text } from "@mantine/core";
import { Guild } from "../auth/AuthProvider";
import { nameToInitials } from "../utils/utils";
import BaseSidebarButton from "./BaseSidebarButton";

const useStyles = createStyles(() => ({
    guildLinkImage: {
        maxHeight: "100%",
        maxWidth: "100%",
    },
}));

export default function GuildLinkButton({ guild }: { guild: Guild }) {
    const { classes } = useStyles();

    return (
        <BaseSidebarButton route={`/guilds/${guild.id}`} label={guild.name}>
            {guild.icon ? (
                <img
                    className={classes.guildLinkImage}
                    src={`https://cdn.discordapp.com/icons/${guild.id}/${guild.icon}`}
                />
            ) : (
                <Text weight="bold">{nameToInitials(guild.name)}</Text>
            )}
        </BaseSidebarButton>
    );
}
