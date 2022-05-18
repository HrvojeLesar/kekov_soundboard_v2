import {
    Text,
    Avatar,
    Box,
    Button,
    Center,
    createStyles,
    CSSObject,
    Group,
    MantineTheme,
    Paper,
    Title,
    UnstyledButton,
    Tooltip,
} from "@mantine/core";
import { PlayerPlay, Plus } from "tabler-icons-react";
import { GuildFile } from "../views/Guild";

const playButtonStyle = (theme: MantineTheme): CSSObject => ({
    width: "50px",
    height: "50px",
    borderRadius: "50%",
    display: "flex",
    textAlign: "center",
    justifyContent: "center",
    alignItems: "center",
    backgroundColor: theme.colors.blue[6],
    color: theme.colors.gray[0],

    "&:hover": {
        backgroundColor: theme.colors.blue[7],
    },

    ":active": {
        transform: "translateY(1px)",
    },
});

const useStyles = createStyles((theme) => ({
    playButtonStyle: {
        ...playButtonStyle(theme),
    },

    container: {
        width: "200px",

        "&:hover": {
            backgroundColor: theme.colors.gray[0],
            transition: ".2s",
        },
    },
}));

export function PlayControl({
    file,
    playFunc,
}: {
    file: GuildFile;
    playFunc: any;
}) {
    const { classes } = useStyles();

    return (
        <Paper
            radius="md"
            withBorder
            shadow="sm"
            p="sm"
            className={classes.container}
            style={{ overflow: "hidden" }}
        >
            <Tooltip
                wrapLines
                withArrow
                position="top"
                label={file.display_name}
                styles={{
                    body: { maxWidth: "300px" }
                }}
                style={{ display: "block" }}
            >
                <Text
                    lineClamp={1}
                    weight="bold"
                    align="center"
                    mb="sm"
                    mx="xl"
                >
                    {file.display_name}
                </Text>
            </Tooltip>
            <UnstyledButton
                mx="auto"
                className={classes.playButtonStyle}
                onClick={() => {
                    playFunc(file.id);
                }}
            >
                <PlayerPlay />
            </UnstyledButton>
        </Paper>
    );
}
