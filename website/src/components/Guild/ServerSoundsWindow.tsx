import { Box, Group, Paper, ScrollArea, TextInput, Title } from "@mantine/core";
import { useEffect, useState } from "react";
import { GuildFile } from "../../views/Guild";
import { PlayControl } from "../PlayControl";
import SearchBar from "../SearchBar";

type ServerSoundsWindowProps = {
    guildId: string | undefined;
    guildFiles: GuildFile[];
    classes: Record<
        "serverSoundsPaper" | "scollAreaStyle" | "sideWindowsStyle",
        string
    >;
};

export default function ServerSoundsWindow({
    guildId,
    guildFiles,
    classes,
}: ServerSoundsWindowProps) {
    const [filterTerm, setFilterTerm] = useState("");

    const filterFiles = () => {
        if (filterTerm !== "") {
            return guildFiles.filter((file) => {
                if (file.display_name) {
                    return file.display_name.toLowerCase().indexOf(filterTerm) !== -1;
                } else {
                    return false;
                }
            });
        } else {
            return guildFiles;
        }
    };

    return (
        <Paper
            withBorder
            shadow="sm"
            p="sm"
            className={classes.serverSoundsPaper}
        >
            <Title title="Server sounds" order={3} pb="xs">
                Server sounds
            </Title>
            <Box py="sm">
                <SearchBar
                    filterCallback={(searchValue) => {
                        setFilterTerm(searchValue);
                    }}
                />
            </Box>
            <ScrollArea className={classes.scollAreaStyle}>
                <Group>
                    {guildId ? (
                        filterFiles().map((file) => {
                            return (
                                <PlayControl
                                    key={file.id}
                                    file={file}
                                    guildId={guildId}
                                />
                            );
                        })
                    ) : (
                        <div></div>
                    )}
                    {/*TODO: ^^^ Warning message or redirect*/}
                </Group>
            </ScrollArea>
        </Paper>
    );
}
