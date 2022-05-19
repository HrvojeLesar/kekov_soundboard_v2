import {
    ChangeEvent,
    ComponentProps,
    ReactNode,
    useCallback,
    useContext,
    useEffect,
    useMemo,
    useRef,
    useState,
} from "react";
import {
    Box,
    Button,
    Center,
    Container,
    createStyles,
    Grid,
    Group,
    MantineTheme,
    Paper,
    Progress,
    RingProgress,
    ScrollArea,
    Stack,
    Text,
    Title,
    useMantineTheme,
} from "@mantine/core";
import { Dropzone, DropzoneStatus } from "@mantine/dropzone";
import {
    FileUploadContainer,
    FileContainerRef,
} from "../components/FileContainer";
import { AuthContext } from "../auth/AuthProvider";
import axios from "axios";
import { API_URL, FilesRoute } from "../api/ApiRoutes";
import { Files, FileUpload, Icon, X } from "tabler-icons-react";
import { v4 as uuidv4 } from "uuid";

const MAX_TOTAL_SIZE = 10_000_000;
const ACCEPTED_MIMES = [
    "audio/mid",
    "audio/midi",
    "audio/x-midi",
    "audio/aac",
    "audio/flac",
    "audio/m4a",
    "audio/x-m4a",
    "audio/mpeg",
    "audio/ogg",
    "audio/wav",
    "audio/webm",
];

type FileWithId = {
    id: string;
    file: File;
};

const getIconColor = (status: DropzoneStatus, theme: MantineTheme) => {
    return status.accepted
        ? theme.colors[theme.primaryColor][theme.colorScheme === "dark" ? 4 : 6]
        : status.rejected
        ? theme.colors.red[theme.colorScheme === "dark" ? 4 : 6]
        : theme.colorScheme === "dark"
        ? theme.colors.dark[0]
        : theme.colors.gray[7];
};

const UploadIcon = ({
    status,
    ...props
}: ComponentProps<Icon> & { status: DropzoneStatus }) => {
    if (status.rejected) {
        return <X {...props} />;
    }

    return <FileUpload {...props} />;
};

const useStyles = createStyles((theme) => {
    return {
        disabled: {
            backgroundColor:
                theme.colorScheme === "dark"
                    ? theme.colors.dark[6]
                    : theme.colors.gray[0],
            borderColor:
                theme.colorScheme === "dark"
                    ? theme.colors.dark[5]
                    : theme.colors.gray[2],
            cursor: "not-allowed",

            "& *": {
                color:
                    theme.colorScheme === "dark"
                        ? theme.colors.dark[3]
                        : theme.colors.gray[5],
            },
        },
    };
});

export default function Upload() {
    const { tokens } = useContext(AuthContext);
    const { classes } = useStyles();
    const openRef = useRef<() => void>(() => {});
    const containerRefs = useRef<FileContainerRef[]>([]);
    const [files, setFiles] = useState<FileWithId[]>([]);
    const [totalSize, setTotalSize] = useState<number>(0);
    const [progressValue, setProgressValue] = useState(0);
    const [isLimitExceeded, setIsLimitExceeded] = useState(false);
    const [isUploadDisabled, setIsUploadDisabled] = useState(true);
    const [isUploading, setIsUploading] = useState(false);
    const [inputErrorsCount, setInputErrorsCount] = useState({ count: 0 });
    const theme = useMantineTheme();

    const addFiles = (selectedFiles: File[]) => {
        let size = totalSize;
        let newFiles = selectedFiles.map((file) => {
            size += file.size;
            return { id: uuidv4(), file: file };
        });
        setTotalSize(size);
        setFiles([...files, ...newFiles]);
    };

    const removeFile = (file: File, hasError: boolean) => {
        let size = totalSize;
        let others = files.filter((f) => {
            if (f.file === file) {
                size -= file.size;
                return false;
            }
            return true;
        });
        setTotalSize(size);
        setFiles(others);
        if (hasError) {
            handleInputErrors(false);
        }
    };

    const upload = async () => {
        if (tokens?.access_token) {
            try {
                const formData = new FormData();
                files.forEach((file, index) => {
                    let { fileName } = containerRefs.current[index];
                    fileName =
                        fileName.trim() == "" ? file.file.name : fileName;
                    formData.append(fileName, file.file);
                });
                setIsUploading(true);
                axios
                    .post(`${API_URL}${FilesRoute.postUpload}`, formData, {
                        headers: {
                            Authorization: `${tokens?.access_token}`,
                            "Content-Type": "multipart/form-data",
                        },
                        onUploadProgress: (progress) => {
                            const uploadPercent = Math.round(
                                (progress.loaded / progress.total) * 100
                            );
                            setProgressValue(uploadPercent);
                        },
                    })
                    .then(() => {
                        // TODO: notify and remove successfully uploaded files
                        console.log("Gotovo");
                        setIsUploading(false);
                    });
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    const handleInputErrors = (inputError: boolean) => {
        if (inputError) {
            inputErrorsCount.count += 1;
        } else {
            inputErrorsCount.count -= 1;
        }
        setInputErrorsCount({ ...inputErrorsCount });
    };

    const limitPercentage = useMemo(() => {
        return Math.round((totalSize / MAX_TOTAL_SIZE) * 100);
    }, [totalSize]);

    useEffect(() => {
        if (totalSize > MAX_TOTAL_SIZE) {
            setIsLimitExceeded(true);
        } else {
            setIsLimitExceeded(false);
        }
    }, [totalSize]);

    useEffect(() => {
        if (files.length > 0 && isUploadDisabled) {
            setIsUploadDisabled(false);
        } else if (files.length == 0 && !isUploadDisabled) {
            setIsUploadDisabled(true);
        }
    }, [files]);

    return (
        <Group direction="column">
            <Paper withBorder shadow="sm" p="sm" style={{ width: "100%" }}>
                <Title order={2} pb="xs">
                    Upload
                </Title>
                <Group>
                    {/* TODO: Animate ring ?? */}
                    <RingProgress
                        sections={[
                            {
                                value: limitPercentage,
                                color: isLimitExceeded ? "red" : "blue",
                            },
                        ]}
                        label={
                            <>
                                <Text align="center" size="sm">
                                    Total size:
                                </Text>
                                <Text align="center" size="sm">
                                    {limitPercentage}%
                                </Text>
                            </>
                        }
                    />
                    <Box style={{ width: "300px" }}>
                        <Group direction="column">
                            <Group position="apart" style={{ width: "100%" }}>
                                <Button
                                    onClick={() => openRef.current()}
                                    disabled={isUploading}
                                >
                                    Select files
                                </Button>
                                <Text>
                                    {isUploading ? `${progressValue}%` : null}
                                </Text>
                                <Button
                                    disabled={
                                        isUploading ||
                                        isUploadDisabled ||
                                        isLimitExceeded ||
                                        inputErrorsCount.count != 0
                                    }
                                    onClick={() => upload()}
                                >
                                    Upload
                                </Button>
                            </Group>
                            <Progress
                                style={{ width: "100%" }}
                                animate
                                value={progressValue}
                            />
                        </Group>
                    </Box>
                    <Box style={{ flexGrow: 1 }}>
                        <Dropzone
                            disabled={isUploading}
                            onDrop={addFiles}
                            onReject={(file) => console.log("rejected: ", file)}
                            openRef={openRef}
                            className={
                                isUploading ? classes.disabled : undefined
                            }
                            accept={ACCEPTED_MIMES}
                        >
                            {(status) => {
                                return (
                                    <Group
                                        direction="column"
                                        position="center"
                                        spacing="sm"
                                        style={{ pointerEvents: "none" }}
                                    >
                                        <UploadIcon
                                            status={status}
                                            size={32}
                                            style={{
                                                color: getIconColor(
                                                    status,
                                                    theme
                                                ),
                                            }}
                                        />
                                        <div>
                                            <Text
                                                align="center"
                                                weight="bold"
                                                size="lg"
                                            >
                                                Sound file upload
                                            </Text>
                                            <Text
                                                align="center"
                                                size="sm"
                                                color="dimmed"
                                            >
                                                Drag sound files here or click
                                                to select files
                                            </Text>
                                        </div>
                                    </Group>
                                );
                            }}
                        </Dropzone>
                    </Box>
                </Group>
            </Paper>

            <Paper
                withBorder
                shadow="sm"
                p="sm"
                style={{
                    width: "100%",
                    height: "calc(100vh - 250px)",
                    display: "flex",
                    flexDirection: "column",
                    overflow: "hidden",
                }}
            >
                <Title order={3} pb="xs">
                    Selected files
                </Title>
                {files.length > 0 ? (
                    <ScrollArea style={{ height: "100%" }}>
                        <Group>
                            {files.map((file, index) => {
                                return (
                                    <FileUploadContainer
                                        key={file.id}
                                        ref={(ref) => {
                                            if (ref) {
                                                containerRefs.current[index] =
                                                    ref;
                                            }
                                        }}
                                        disabled={isUploading}
                                        file={file.file}
                                        deleteCallback={(
                                            file: File,
                                            hasError: boolean
                                        ) => removeFile(file, hasError)}
                                        inputErrorCallback={handleInputErrors}
                                    />
                                );
                            })}
                        </Group>
                    </ScrollArea>
                ) : (
                    <Box
                        style={{
                            height: "100%",
                            justifyContent: "center",
                            display: "flex",
                            textAlign: "center",
                            alignItems: "center",
                            flexDirection: "column",
                        }}
                    >
                        <Text weight="bold" align="center">
                            No files selected
                        </Text>
                        <Text size="sm" color="dimmed" align="center">
                            Please add some files to upload
                        </Text>
                    </Box>
                )}
            </Paper>
        </Group>
    );
}
