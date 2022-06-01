import {
    ComponentProps,
    CSSProperties,
    useContext,
    useEffect,
    useMemo,
    useRef,
    useState,
} from "react";
import {
    Box,
    Button,
    createStyles,
    Grid,
    Group,
    MantineTheme,
    Paper,
    Progress,
    RingProgress,
    ScrollArea,
    Text,
    Title,
    useMantineTheme,
} from "@mantine/core";
import { Dropzone, DropzoneStatus } from "@mantine/dropzone";
import {
    FileUploadContainer,
    FileContainerRef,
} from "../components/FileContainer";
import { AuthContext, COOKIE_NAMES } from "../auth/AuthProvider";
import { FileUpload, Icon, X } from "tabler-icons-react";
import { v4 as uuidv4 } from "uuid";
import { showNotification } from "@mantine/notifications";
import {
    UploadGuildWindow,
    UploadGuildWindowRef,
} from "../components/Upload/UploadGuildWindow";
import { useCookies } from "react-cookie";
import { useDocumentTitle } from "@mantine/hooks";
import { ApiRequest, UserFile } from "../utils/utils";

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
        uploadPaperStyle: {
            width: "100%",
        },
        uploadControlsBoxStyle: {
            width: "300px",
        },
        uploadControlsGroupStyle: {
            width: "100%",
        },
        dropzoneBoxStyle: {
            flexGrow: 1,
        },
        dropzoneGroupStyle: {
            pointerEvents: "none",
        },
        selectedFilesPaperStyle: {
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
            ...uploadMaximumWindowHeight,
        },
        scrollAreaStyle: {
            height: "100%",
        },
        selectedFilesBoxStyle: {
            height: "100%",
            justifyContent: "center",
            display: "flex",
            textAlign: "center",
            alignItems: "center",
            flexDirection: "column",
        },
    };
});

export const uploadMaximumWindowHeight: CSSProperties = {
    height: "calc(100vh - 255px)",
};

export default function Upload() {
    const { guilds } = useContext(AuthContext);
    const [cookies] = useCookies(COOKIE_NAMES);
    const { classes } = useStyles();
    const openRef = useRef<() => void>(() => {});
    const containerRefs = useRef<FileContainerRef[]>([]);
    const selectedGuildsRef = useRef<UploadGuildWindowRef>(null!);
    const [files, setFiles] = useState<FileWithId[]>([]);
    const [totalSize, setTotalSize] = useState<number>(0);
    const [progressValue, setProgressValue] = useState(0);
    const [isLimitExceeded, setIsLimitExceeded] = useState(false);
    const [isUploadDisabled, setIsUploadDisabled] = useState(true);
    const [isUploading, setIsUploading] = useState(false);
    const [inputErrorsCount, setInputErrorsCount] = useState({ count: 0 });
    const theme = useMantineTheme();
    useDocumentTitle("KSv2 - Upload");

    const addFiles = (selectedFiles: File[]) => {
        let newFiles = selectedFiles.map((file) => {
            return { id: uuidv4(), file: file };
        });
        setFiles([...files, ...newFiles]);
    };

    const removeFile = (file: File, hasError: boolean) => {
        let others = files.filter((f) => {
            if (f.file === file) {
                return false;
            }
            return true;
        });
        setFiles(others);
        if (hasError) {
            handleInputErrors(false);
        }
    };

    const calcSize = () => {
        let size = 0;
        files.forEach((f) => {
            size += f.file.size;
        });
        return size;
    };

    const compareUploaded: (uploadedFiles: UserFile[]) => FileWithId[] = (
        uploadedFiles: UserFile[]
    ) => {
        const maxLen = files.length - uploadedFiles.length;
        let len = 0;
        if (uploadedFiles.length !== files.length) {
            return files.filter((file, index) => {
                if (len > maxLen) {
                    return false;
                }
                let { fileName } = containerRefs.current[index];
                fileName = fileName.trim() === "" ? file.file.name : fileName;
                if (
                    uploadedFiles.find((userFile) => {
                        return userFile.display_name === fileName;
                    }) === undefined
                ) {
                    len++;
                    return true;
                }
                return false;
            });
        }
        return [];
    };

    const upload = () => {
        if (!cookies.access_token) {
            return;
        }
        const formData = new FormData();
        files.forEach((file, index) => {
            let { fileName } = containerRefs.current[index];
            fileName = fileName.trim() === "" ? file.file.name : fileName;
            formData.append(fileName, file.file);
        });
        setIsUploading(true);
        ApiRequest.upload(formData, cookies.access_token, setProgressValue)
            .then(async ({ data }) => {
                const selectedGuildIds =
                    selectedGuildsRef.current.selectedGuildIds;

                const failedFiles = compareUploaded(data);
                const hasFailedFiles = failedFiles.length > 0;
                let hasQuickEnableFailed = false;

                if (selectedGuildIds.length > 0 && data.length > 0) {
                    await quickEnable(selectedGuildIds, data).catch((err) => {
                        console.log(err);
                        hasQuickEnableFailed = true;
                    });
                }

                showNotification({
                    title: hasFailedFiles
                        ? hasQuickEnableFailed
                            ? "Error"
                            : "Some files failed to upload"
                        : hasQuickEnableFailed
                        ? "Upload successful, server addition failed"
                        : "All files uploaded",
                    message: hasFailedFiles
                        ? hasQuickEnableFailed
                            ? "Files left in selected files have failed to upload and successfully uploaded files failed to be added to selected servers!"
                            : "Files left in selected files have failed to upload!"
                        : hasQuickEnableFailed
                        ? "Failed to add files to servers but files have successfully uploaded and are available in user files!"
                        : "All files have been successfully uploaded!",
                    autoClose:
                        hasFailedFiles || hasQuickEnableFailed ? false : 3000,
                    color:
                        hasFailedFiles || hasQuickEnableFailed
                            ? "red"
                            : "green",
                    icon: hasFailedFiles || hasQuickEnableFailed ? <X /> : null,
                });

                setFiles(failedFiles);
                setIsUploading(false);
            })
            .catch((e) => {
                console.log(e);
                showNotification({
                    title: "Error",
                    message: "File upload failed!",
                    autoClose: false,
                    color: "red",
                    icon: <X />,
                });
            });
    };

    const quickEnable = (guilds: string[], files: UserFile[]) => {
        if (!cookies.access_token) {
            return Promise.reject("Access token not set");
        }
        const bulk = {
            guilds: guilds,
            files: files.map((f) => f.id),
        };
        return ApiRequest.bulkEnable(bulk, cookies.access_token);
    };

    useEffect(() => {
        setTotalSize(calcSize());
    }, [files]);

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
        } else if (files.length === 0 && !isUploadDisabled) {
            setIsUploadDisabled(true);
        }
    }, [files]);

    return (
        <>
            <Group direction="column">
                <Paper
                    withBorder
                    shadow="sm"
                    p="sm"
                    className={classes.uploadPaperStyle}
                >
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
                        <Box className={classes.uploadControlsBoxStyle}>
                            <Group direction="column">
                                <Group
                                    position="apart"
                                    className={classes.uploadControlsGroupStyle}
                                >
                                    <Button
                                        onClick={() => openRef.current()}
                                        disabled={isUploading}
                                    >
                                        Select files
                                    </Button>
                                    <Text>
                                        {isUploading
                                            ? `${progressValue}%`
                                            : null}
                                    </Text>
                                    <Button
                                        disabled={
                                            isUploading ||
                                            isUploadDisabled ||
                                            isLimitExceeded ||
                                            inputErrorsCount.count !== 0
                                        }
                                        onClick={() => upload()}
                                    >
                                        Upload
                                    </Button>
                                </Group>
                                <Progress
                                    className={classes.uploadControlsGroupStyle}
                                    animate
                                    value={progressValue}
                                />
                            </Group>
                        </Box>
                        <Box className={classes.dropzoneBoxStyle}>
                            <Dropzone
                                disabled={isUploading}
                                onDrop={addFiles}
                                onReject={(file) =>
                                    console.log("rejected: ", file)
                                }
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
                                            className={
                                                classes.dropzoneGroupStyle
                                            }
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
                                                    Drag sound files here or
                                                    click to select files
                                                </Text>
                                            </div>
                                        </Group>
                                    );
                                }}
                            </Dropzone>
                        </Box>
                    </Group>
                </Paper>
            </Group>
            <Grid mt="sm">
                <Grid.Col xs={9}>
                    <Paper
                        withBorder
                        shadow="sm"
                        p="sm"
                        className={classes.selectedFilesPaperStyle}
                    >
                        <Title order={3} pb="xs">
                            Selected files
                        </Title>
                        {files.length > 0 ? (
                            <ScrollArea className={classes.scrollAreaStyle}>
                                <Group>
                                    {files.map((file, index) => {
                                        return (
                                            <FileUploadContainer
                                                key={file.id}
                                                ref={(ref) => {
                                                    if (ref) {
                                                        containerRefs.current[
                                                            index
                                                        ] = ref;
                                                    }
                                                }}
                                                disabled={isUploading}
                                                file={file.file}
                                                deleteCallback={(
                                                    file: File,
                                                    hasError: boolean
                                                ) => removeFile(file, hasError)}
                                                inputErrorCallback={
                                                    handleInputErrors
                                                }
                                            />
                                        );
                                    })}
                                </Group>
                            </ScrollArea>
                        ) : (
                            <Box className={classes.selectedFilesBoxStyle}>
                                <Text weight="bold" align="center">
                                    No files selected
                                </Text>
                                <Text size="sm" color="dimmed" align="center">
                                    Please add some files to upload
                                </Text>
                            </Box>
                        )}
                    </Paper>
                </Grid.Col>
                <Grid.Col xs={3}>
                    <UploadGuildWindow
                        ref={selectedGuildsRef}
                        guilds={guilds}
                    />
                </Grid.Col>
            </Grid>
        </>
    );
}
