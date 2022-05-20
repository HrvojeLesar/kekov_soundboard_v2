import { Box, Checkbox, Paper, ScrollArea, Title } from "@mantine/core";
import { useListState } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { Guild } from "../../auth/AuthProvider";
import UploadGuildCheckbox from "./UploadGuildCheckbox";

type UploadGuildWindowProps = {
    guilds: Guild[];
};

export default function UploadGuildWindow({ guilds }: UploadGuildWindowProps) {
    const mappedGuilds = guilds.map((guild) => {
        return { key: guild.id, checked: false };
    });

    const [values, handlers] = useListState<{ key: string; checked: boolean }>(
        mappedGuilds
    );

    const allChecked = values.every((value) => value.checked);
    const indeterminate = values.some((value) => value.checked) && !allChecked;

    useEffect(() => {
        handlers.setState(mappedGuilds);
    }, [guilds]);

    return (
        <Paper
            withBorder
            shadow="sm"
            p="sm"
            style={{
                height: "calc(100vh - 255px)",
                display: "flex",
                flexDirection: "column",
                overflow: "hidden",
            }}
        >
            <Title order={3} pb="xs">
                Add to server
            </Title>
            {values.length > 0 ? (
                <>
                    <Checkbox
                        mb="sm"
                        checked={allChecked}
                        indeterminate={indeterminate}
                        label="Add to all servers"
                        transitionDuration={0}
                        onChange={() =>
                            handlers.setState((current) =>
                                current.map((value) => ({
                                    ...value,
                                    checked: !allChecked,
                                }))
                            )
                        }
                    />
                    <ScrollArea>
                        {guilds.map((guild, index) => {
                            return (
                                <Box m="sm" key={guild.id}>
                                    <UploadGuildCheckbox
                                        guild={guild}
                                        isChecked={values[index].checked}
                                        onChange={(checked: boolean) => {
                                            handlers.setItemProp(
                                                index,
                                                "checked",
                                                checked
                                            );
                                        }}
                                    />
                                </Box>
                            );
                        })}
                    </ScrollArea>
                </>
            ) : (
                <></>
            )}
        </Paper>
    );
}
