import {
    UnstyledButton,
    Image,
    Text,
    Checkbox,
    createStyles,
    Group,
} from "@mantine/core";
import { Guild } from "../../auth/AuthProvider";
import { nameToInitials } from "../../utils/utils";

type UploadGuildCheckboxProps = {
    guild: Guild;
    isChecked: boolean;
    onChange: (checked: boolean) => void;
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

export default function UploadGuildCheckbox({
    guild,
    isChecked,
    onChange,
}: UploadGuildCheckboxProps) {
    const { classes } = useStyles({ checked: isChecked });

    return (
        <UnstyledButton
            className={classes.button}
            onClick={() => {
                onChange(!isChecked);
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
                            maxWidth: "19ch",
                            textOverflow: "ellipsis",
                            overflow: "hidden",
                        }}
                        lineClamp={1}
                    >
                        {guild.name}
                    </Text>
                </Group>
                <Checkbox
                    checked={isChecked}
                    onChange={() => {}}
                    tabIndex={-1}
                    styles={{ input: { cursor: "pointer" } }}
                />
            </Group>
        </UnstyledButton>
    );
}
