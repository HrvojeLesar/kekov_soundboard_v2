import {
    Button,
    Text,
    Center,
    Group,
    Paper,
    Badge,
    Tooltip,
    ActionIcon,
    Box,
    UnstyledButton,
    createStyles,
} from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { useEffect } from "react";
import { CirclePlus, Pencil, Trash } from "tabler-icons-react";
import { UserFile } from "../../views/UserFiles";

type UserFileContainerProps = {
    isSelected: boolean;
    file: UserFile;
    onClickCallback: () => void;
};

const useStyles = createStyles(
    (theme, { isSelected }: { isSelected: boolean }) => {
        return {
            button: {
                display: "flex",
                alignItems: "center",
                width: "100%",
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
                padding: theme.spacing.sm,
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
        <Paper
            withBorder
            shadow="xs"
            style={{ padding: "0", width: "250px", overflow: "hidden" }}
            className={classes.button}
        >
            <UnstyledButton
                p="sm"
                style={{ width: "100%", height: "100%" }}
                onClick={() => {
                    onClickCallback();
                }}
            >
                <Group spacing="sm">
                    <Text
                        style={{
                            maxWidth: "150px",
                            textOverflow: "ellipsis",
                            whiteSpace: "nowrap",
                            overflow: "hidden",
                        }}
                        title={file.display_name}
                    >
                        {file.display_name}
                    </Text>
                </Group>
            </UnstyledButton>
        </Paper>
    );
}
