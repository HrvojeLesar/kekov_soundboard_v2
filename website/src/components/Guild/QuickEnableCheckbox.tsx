import {
    Checkbox,
    Group,
    UnstyledButton,
    Text,
    createStyles,
    Box,
} from "@mantine/core";
import { EnabledUserFile } from "./QuickEnableWindow";

const useStyles = createStyles((theme, { checked }: { checked: boolean }) => {
    return {
        button: {
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
        groupStyle: {
            flexGrow: 1,
        },
        textStyle: {
            maxWidth: "19ch",
            textOverflow: "ellipsis",
            overflow: "hidden",
        },
    };
});

type QuickEnableCheckboxProps = {
    onChange: (state: boolean, file: EnabledUserFile) => void;
    file: EnabledUserFile;
};

export default function QuickEnableCheckbox({
    onChange,
    file,
}: QuickEnableCheckboxProps) {
    const { classes } = useStyles({ checked: file.enabled });

    return (
        <UnstyledButton
            className={classes.button}
            onClick={() => {
                onChange(!file.enabled, file);
            }}
        >
            <Group position="apart" className={classes.groupStyle} noWrap>
                <Group>
                    <Text
                        title={file.sound_file.display_name}
                        className={classes.textStyle}
                        lineClamp={1}
                    >
                        {file.sound_file.display_name}
                    </Text>
                </Group>
                <Checkbox
                    checked={file.enabled}
                    onChange={() => {}}
                    tabIndex={-1}
                    styles={{ input: { cursor: "pointer" } }}
                />
            </Group>
        </UnstyledButton>
    );
}
