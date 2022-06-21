import {
    Text,
    Group,
    Paper,
    UnstyledButton,
    createStyles,
} from "@mantine/core";
import { primaryShade, SoundFile } from "../../utils/utils";

type UserFileContainerProps = {
    isSelected: boolean;
    file: SoundFile | SoundFile;
    onClickCallback: (file: SoundFile | SoundFile ) => void;
};

const useStyles = createStyles(
    (theme, { isSelected }: { isSelected: boolean }) => {
        const shade = primaryShade(theme);
        return {
            button: {
                width: "250px",
                overflow: "hidden",
                display: "flex",
                alignItems: "center",
                transition:
                    "background-color 150ms ease, border-color 150ms ease",
                border: `1px solid ${
                    isSelected
                        ? theme.colors[theme.primaryColor][shade]
                        : theme.colorScheme === "dark"
                        ? theme.colors.dark[shade]
                        : theme.colors.gray[shade]
                }`,
                borderRadius: theme.radius.sm,
                padding: 0,
                backgroundColor: isSelected
                    ? theme.colorScheme === "dark"
                        ? theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          )
                        : theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          )
                    : theme.colorScheme === "dark"
                    ? theme.colors.dark[8]
                    : theme.white,

                "&:hover": {
                    transition: "150ms ease",
                    backgroundColor:
                        theme.colorScheme === "dark"
                            ? theme.fn.rgba(
                                  theme.colors[theme.primaryColor][shade],
                                  0.3
                              )
                            : theme.fn.rgba(
                                  theme.colors[theme.primaryColor][shade],
                                  0.3
                              ),
                },
            },
            unstyledButtonStyle: { width: "100%", height: "100%" },
            textStyle: {
                maxWidth: "150px",
                textOverflow: "ellipsis",
                whiteSpace: "nowrap",
                overflow: "hidden",
            },
        };
    }
);

export default function SelectableFileContainer({
    isSelected,
    file,
    onClickCallback,
}: UserFileContainerProps) {
    const { classes } = useStyles({ isSelected: isSelected });

    return (
        <Paper withBorder shadow="xs" className={classes.button}>
            <UnstyledButton
                p="sm"
                className={classes.unstyledButtonStyle}
                onClick={() => {
                    onClickCallback(file);
                }}
            >
                <Group spacing="sm">
                    <Text
                        className={classes.textStyle}
                        title={file.display_name}
                    >
                        {file.display_name}
                    </Text>
                </Group>
            </UnstyledButton>
        </Paper>
    );
}
