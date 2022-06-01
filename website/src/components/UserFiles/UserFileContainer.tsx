import {
    Text,
    Group,
    Paper,
    UnstyledButton,
    createStyles,
} from "@mantine/core";
import { UserFile } from "../../utils/utils";

type UserFileContainerProps = {
    isSelected: boolean;
    file: UserFile;
    onClickCallback: (file: UserFile) => void;
};

const useStyles = createStyles(
    (theme, { isSelected }: { isSelected: boolean }) => {
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
                        ? theme.colors[theme.primaryColor][
                              theme.colorScheme === "dark" ? 9 : 6
                          ]
                        : theme.colorScheme === "dark"
                        ? theme.colors.dark[8]
                        : theme.colors.gray[3]
                }`,
                borderRadius: theme.radius.sm,
                padding: 0,
                backgroundColor: isSelected
                    ? theme.colorScheme === "dark"
                        ? theme.fn.rgba(
                              theme.colors[theme.primaryColor][8],
                              0.3
                          )
                        : theme.colors[theme.primaryColor][0]
                    : theme.colorScheme === "dark"
                    ? theme.colors.dark[8]
                    : theme.white,

                "&:hover": {
                    transition: "150ms ease",
                    backgroundColor: theme.colors.gray[0],
                },
            },

            image: {
                border: `1px solid ${
                    isSelected
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

export default function UserFileContainer({
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
