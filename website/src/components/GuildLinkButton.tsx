import { createStyles, Text } from "@mantine/core";
import { Guild } from "../LoginCallback";
import BaseSidebarButton from "./BaseSidebarButton";

const useStyles = createStyles(() => ({
    guildLinkImage: {
        maxHeight: "100%",
        maxWidth: "100%",
    },
}));

export default function GuildLinkButton({ guild }: { guild: Guild }) {
    const { classes } = useStyles();

    const nameToInitials = (guildName: string): string => {
        let initials = "";
        guildName.split(" ").forEach((word) => {
            if (word[0]) {
                initials = initials.concat(word[0]);
            }
        });
        return initials;
    };

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
