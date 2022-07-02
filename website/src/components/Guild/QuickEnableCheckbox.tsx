import {
    Checkbox,
    Group,
    UnstyledButton,
    Text,
    createStyles,
    LoadingOverlay,
    Box,
} from "@mantine/core";
import { useState } from "react";
import { LOADINGOVERLAY_ZINDEX, primaryShade } from "../../utils/utils";
import { EnabledUserFile } from "./QuickEnableWindow";

const useStyles = createStyles((theme, { checked }: { checked: boolean }) => {
    const shade = primaryShade(theme);
    return {
        button: {
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
                ? theme.fn.rgba(theme.colors[theme.primaryColor][shade], 0.3)
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
        boxStyle: {
            position: "relative",
        },
    };
});

type QuickEnableCheckboxProps = {
    onChange: (state: boolean, file: EnabledUserFile) => Promise<void>;
    file: EnabledUserFile;
};

export default function QuickEnableCheckbox({
    onChange,
    file,
}: QuickEnableCheckboxProps) {
    const { classes } = useStyles({ checked: file.enabled });
    const [isLoading, setIsLoading] = useState(false);

    return (
        <Box className={classes.boxStyle}>
            <LoadingOverlay
                zIndex={LOADINGOVERLAY_ZINDEX}
                visible={isLoading}
            />
            <UnstyledButton
                className={classes.button}
                onClick={() => {
                    setIsLoading(true);
                    onChange(!file.enabled, file).finally(() => {
                        setIsLoading(false);
                    });
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
        </Box>
    );
}
