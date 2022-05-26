import {
    Text,
    createStyles,
    CSSObject,
    MantineTheme,
    Paper,
    UnstyledButton,
} from "@mantine/core";
import { PlayerPlay } from "tabler-icons-react";
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
        overflow: "hidden",

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
        >
            <Text
                title={file.display_name}
                lineClamp={1}
                weight="bold"
                align="center"
                mb="sm"
                mx="xl"
            >
                {file.display_name}
            </Text>
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
