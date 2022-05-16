import {
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
} from "@mantine/core";
import { PlayerPlay, Plus } from "tabler-icons-react";
import { GuildFile } from "../views/Guild";

export function PlayControl({
    file,
    playFunc,
}: {
    file: GuildFile;
    playFunc: any;
}) {
    // <Button key={file.id} onClick={() => playFunc(file.id)}>
    //     {file.display_name ?? "Golden Legendary"}
    // </Button>
    const playButtonStyle = (theme: MantineTheme): CSSObject => ({
        // border: "1px solid",
        textAlign: "center",
        width: "50px",
        height: "50px",
        borderRadius: "50%",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        backgroundColor: theme.colors.blue[6],

        "&:hover": {
            backgroundColor: theme.colors.blue[7],
        },
    });

    return (
        <Paper
            radius="md"
            withBorder
            shadow="sm"
            p="sm"
            style={{ width: "100%" }}
        >
            <Group position="apart">
                <Title>{file.display_name}</Title>
                <UnstyledButton
                    sx={playButtonStyle}
                    onClick={() => {
                        playFunc(file.id);
                    }}
                >
                    <PlayerPlay />
                </UnstyledButton>
            </Group>
        </Paper>
    );
}
