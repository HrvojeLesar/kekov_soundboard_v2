import {
    Box,
    Checkbox,
    Paper,
    ScrollArea,
    Text,
    Title,
    createStyles,
} from "@mantine/core";
import {
    forwardRef,
    useEffect,
    useImperativeHandle,
    useMemo,
    useState,
} from "react";
import { Guild } from "../../auth/AuthProvider";
import { uploadMaximumWindowHeight } from "../../views/Upload";
import UploadGuildCheckbox from "./UploadGuildCheckbox";

type UploadGuildWindowProps = {
    guilds: Guild[];
};

export type UploadGuildWindowRef = {
    selectedGuildIds: string[];
};

const useStyles = createStyles((_theme) => {
    return {
        paperStyle: {
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
            position: "relative",
            ...uploadMaximumWindowHeight,
        },
    };
});

type CheckedGuild = {
    key: string;
    checked: boolean;
};

export const UploadGuildWindow = forwardRef<
    UploadGuildWindowRef,
    UploadGuildWindowProps
>((props, ref) => {
    const { guilds } = props;
    const { classes } = useStyles();

    const mappedGuilds = useMemo(() => {
        return guilds.map((guild) => {
            return { key: guild.id, checked: false };
        });
    }, [guilds]);

    const [checkedGuilds, setCheckedGuilds] = useState<CheckedGuild[]>([]);

    useEffect(() => {
        setCheckedGuilds([...mappedGuilds]);
    }, [mappedGuilds]);

    const allChecked = checkedGuilds.every((value) => value.checked);
    const indeterminate =
        checkedGuilds.some((value) => value.checked) && !allChecked;

    useImperativeHandle(
        ref,
        () => {
            return {
                selectedGuildIds: checkedGuilds
                    .filter((val) => val.checked)
                    .map((val) => val.key),
            };
        },
        [checkedGuilds]
    );

    return (
        <Paper withBorder shadow="sm" p="sm" className={classes.paperStyle}>
            <Title order={3} pb="xs">
                Add to server
            </Title>
            {checkedGuilds.length > 0 ? (
                <>
                    <Checkbox
                        mb="sm"
                        checked={allChecked}
                        indeterminate={indeterminate}
                        label="Add to all servers"
                        transitionDuration={0}
                        onChange={() =>
                            setCheckedGuilds((current) => {
                                return current.map((value) => ({
                                    ...value,
                                    checked: !allChecked,
                                }));
                            })
                        }
                    />
                    <ScrollArea>
                        {guilds.map((guild, index) => {
                            return (
                                <Box m="sm" key={guild.id}>
                                    <UploadGuildCheckbox
                                        guild={guild}
                                        isChecked={checkedGuilds[index].checked}
                                        onChange={(checked: boolean) => {
                                            checkedGuilds[index].checked =
                                                checked;
                                            setCheckedGuilds([
                                                ...checkedGuilds,
                                            ]);
                                        }}
                                    />
                                </Box>
                            );
                        })}
                    </ScrollArea>
                </>
            ) : (
                <Text size="xl" weight="bold">
                    You don't share any server with bot.
                </Text>
            )}
        </Paper>
    );
});
