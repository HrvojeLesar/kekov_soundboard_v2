import { Box, Checkbox, Paper, ScrollArea, Title } from "@mantine/core";
import { useListState } from "@mantine/hooks";
import { forwardRef, useEffect, useImperativeHandle } from "react";
import { Guild } from "../../auth/AuthProvider";
import { uploadMaximumWindowHeight } from "../../views/Upload";
import UploadGuildCheckbox from "./UploadGuildCheckbox";

type UploadGuildWindowProps = {
    guilds: Guild[];
};

export type UploadGuildWindowRef = {
    selectedGuildIds: string[];
};

export const UploadGuildWindow = forwardRef<
    UploadGuildWindowRef,
    UploadGuildWindowProps
>((props, ref) => {
    const { guilds } = props;

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

    useImperativeHandle(ref, () => {
        return {
            selectedGuildIds: values.filter((val) => val.checked).map((val) => val.key),
        };
    }, [values]);

    return (
        <Paper
            withBorder
            shadow="sm"
            p="sm"
            style={{
                display: "flex",
                flexDirection: "column",
                overflow: "hidden",
                ...uploadMaximumWindowHeight,
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
});
