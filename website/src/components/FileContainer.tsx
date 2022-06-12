import {
    RingProgress,
    TextInput,
    Text,
    ActionIcon,
    Paper,
    Group,
    Checkbox,
    createStyles,
} from "@mantine/core";
import { forwardRef, useEffect, useImperativeHandle, useState } from "react";
import { TbTrash } from "react-icons/tb";

const MAXLEN = 50;

const useStyles = createStyles((theme) => {
    return {
        root: {
            position: "relative",
        },

        input: {
            height: "auto",
            paddingTop: "22px",
        },

        label: {
            position: "absolute",
            pointerEvents: "none",
            fontSize: theme.fontSizes.xs,
            paddingLeft: theme.spacing.sm,
            paddingTop: theme.spacing.sm / 2,
            zIndex: 1,
        },

        invalid: {
            height: "auto",
            paddingTop: "22px",
            backgroundColor:
                theme.colorScheme === "dark"
                    ? theme.fn.rgba(theme.colors.red[8], 0.15)
                    : theme.colors.red[0],
            borderColor: theme.colors.red[5],
        },

        icon: {
            color: theme.colors.red[theme.colorScheme === "dark" ? 7 : 6],
        },

        paperStyle: {
            width: "250px",
            overflow: "hidden",
        },

        groupStyle: {
            flexGrow: 1,
        },

        textStyle: {
            textOverflow: "ellipsis",
            overflow: "hidden",
            whiteSpace: "nowrap",
            maxWidth: "19ch",
        },
    };
});

type FileContainerProps = {
    file: File;
    deleteCallback: (file: File, error: boolean) => void;
    disabled: boolean;
    inputErrorCallback: (inputError: boolean) => void;
};

export type FileContainerRef = {
    fileName: string;
};

export const FileUploadContainer = forwardRef<
    FileContainerRef,
    FileContainerProps
>((props, ref) => {
    const { file, deleteCallback, disabled, inputErrorCallback } = props;
    const [ringCharCount, setRingCharCount] = useState<{
        value: number;
        color: string;
    }>({ value: 0, color: "red" });
    const [isPublic, setIsPublic] = useState(false);
    const [isNameLenghtExceeded, setIsNameLenghtExceeded] = useState(false);
    const { classes } = useStyles();

    const removeExtension = (initialValue: string) => {
        let index = initialValue.length - 1;
        for (; index >= 0; index--) {
            if (initialValue[index] === ".") {
                break;
            }
        }

        if (index > 0) {
            return initialValue.substring(0, index);
        }
        return initialValue;
    };

    const [value, setValue] = useState(removeExtension(file.name));

    const calculateCharCount = () => {
        const percent = Math.ceil((value.length / MAXLEN) * 100);
        let set = { value: percent, color: "violet" };
        if (MAXLEN - value.length <= 20) {
            set.color = "orange";
        }
        if (value.length >= MAXLEN) {
            set = { value: 100, color: "red" };
        }
        return set;
    };

    const showNum = () => {
        const len = MAXLEN - value.length;
        if (len > 20) {
            return <></>;
        }
        let color = "orange";
        if (len <= 0) {
            color = "red";
        }
        return (
            <Text align="center" size="xs" color={color}>
                {len}
            </Text>
        );
    };

    useEffect(() => {
        setRingCharCount(calculateCharCount());
        if (value.length > MAXLEN && !isNameLenghtExceeded) {
            inputErrorCallback(true);
            setIsNameLenghtExceeded(true);
        } else if (value.length <= MAXLEN && isNameLenghtExceeded) {
            inputErrorCallback(false);
            setIsNameLenghtExceeded(false);
        }
    }, [value]);

    useImperativeHandle(ref, () => ({
        fileName: value,
    }));

    return (
        <Paper withBorder shadow="xs" p="sm" className={classes.paperStyle}>
            <Group position="apart" mb="xs" className={classes.groupStyle}>
                <Text
                    lineClamp={1}
                    weight="bold"
                    title={file.name}
                    className={classes.textStyle}
                >
                    {file.name}
                </Text>
                <ActionIcon
                    title="Remove file"
                    disabled={disabled}
                    color="red"
                    variant="outline"
                    onClick={() => deleteCallback(file, isNameLenghtExceeded)}
                >
                    <TbTrash size={24} />
                </ActionIcon>
            </Group>
            <TextInput
                disabled={disabled}
                classNames={
                    !isNameLenghtExceeded
                        ? classes
                        : { ...classes, input: classes.invalid }
                }
                value={value}
                onChange={(e) => {
                    setValue(e.target.value);
                }}
                placeholder={file.name}
                label={"File name"}
                styles={{ label: { fontWeight: "bold" } }}
                rightSection={
                    <>
                        <RingProgress
                            size={30}
                            thickness={3}
                            sections={[ringCharCount]}
                            label={showNum()}
                        />
                    </>
                }
            />
            {isNameLenghtExceeded && (
                <Text color="red" size="xs">
                    Maximum name lenght exceeded!
                </Text>
            )}
            <Checkbox
                // disabled={disabled}
                disabled
                mt="xs"
                checked={isPublic}
                label={"Public"}
                styles={{ label: { fontWeight: "bold" } }}
                onChange={(e) => setIsPublic(e.target.checked)}
            />
        </Paper>
    );
});
