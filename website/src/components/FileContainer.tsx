import {
    RingProgress,
    TextInput,
    Text,
    Progress,
    ActionIcon,
    Paper,
    Grid,
    Center,
    Box,
    Button,
    Stack,
} from "@mantine/core";
import { assignRef } from "@mantine/hooks";
import axios from "axios";
import {
    forwardRef,
    ReactNode,
    useContext,
    useEffect,
    useImperativeHandle,
    useState,
} from "react";
import { Trash } from "tabler-icons-react";
import { API_URL, FilesRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";

const MAXLEN = 255;

type FileContainerProps = {
    file: File;
    deleteCallback: (file: File) => void;
    disabled: boolean;
};

export type FileContainerRef = {
    fileName: string,
};

export const FileUploadContainer = forwardRef<
    FileContainerRef,
    FileContainerProps
>((props, ref) => {
    const { file, deleteCallback, disabled } = props;

    const removeExtension = (initialValue: string) => {
        let index = initialValue.length - 1;
        for (; index >= 0; index--) {
            if (initialValue[index] == ".") {
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
        let set = { value: percent, color: "blue" };
        if (MAXLEN - value.length <= 20) {
            set.color = "orange";
        }
        if (value.length >= MAXLEN) {
            set = { value: 100, color: "red" };
        }
        return set;
    };

    const [ringCharCount, setRingCharCount] = useState<{
        value: number;
        color: string;
    }>({ value: 100, color: "red" });

    const updateCharCount = () => {
        setRingCharCount(calculateCharCount());
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
        updateCharCount();
    }, [value]);

    useImperativeHandle(ref, () => ({
        fileName: value,
    }));

    return (
        <Paper withBorder shadow="xs" p="sm">
            <Grid columns={12}>
                <Grid.Col span={11}>
                    <Stack>
                        <TextInput
                            disabled={disabled}
                            value={value}
                            onChange={(e) => {
                                setValue(e.target.value);
                            }}
                            placeholder={file.name}
                            label={"Labela"}
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
                    </Stack>
                </Grid.Col>
                <Grid.Col span={1}>
                    {/*TODO: CENTER*/}
                    <Center>
                        <ActionIcon
                            disabled={disabled}
                            color="red"
                            variant="outline"
                            onClick={() => deleteCallback(file)}
                        >
                            <Trash />
                        </ActionIcon>
                    </Center>
                </Grid.Col>
            </Grid>
        </Paper>
    );
});
