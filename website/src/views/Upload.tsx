import {
    ComponentProps,
    CSSProperties,
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
    createStyles,
    Grid,
    Group,
    MantineTheme,
    Paper,
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
import { TbFileUpload, TbX } from "react-icons/tb";
import { v4 as uuidv4 } from "uuid";
import { showNotification } from "@mantine/notifications";
import {
    UploadGuildWindow,
    UploadGuildWindowRef,
} from "../components/Upload/UploadGuildWindow";
import { useCookies } from "react-cookie";
import { useDocumentTitle } from "@mantine/hooks";
import { ApiRequest, UploadedFile } from "../utils/utils";
import { IconType } from "react-icons";
import UploadModal from "../components/Upload/UploadModal";
import { useNavigate } from "react-router-dom";

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
    fileHandle: File;
};

export type InputFile = {
    file: FileWithId;
    inputHasError: boolean;
    displayName?: string;
};

export type SeparatedFiles = {
    successfullFiles: InputFile[];
    failedFiles: InputFile[];
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
}: ComponentProps<IconType> & { status: DropzoneStatus }) => {
    if (status.rejected) {
        return <TbX {...props} />;
    }

    return <TbFileUpload {...props} />;
};

const useStyles = createStyles((theme) => {
    return {
        uploadPaperStyle: {
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
    height: "calc(100vh - 293px)",
};

// TODO: Make progressbar cover whole screen
export default function Upload() {
    const { guilds } = useContext(AuthContext);
    const [cookies] = useCookies(COOKIE_NAMES);
    const { classes } = useStyles();
    const openRef = useRef<() => void>(() => {});
    const containerRefs = useRef<FileContainerRef[]>([]);
    const selectedGuildsRef = useRef<UploadGuildWindowRef>(null!);
    const [inputFiles, setInputFiles] = useState<InputFile[]>([]);
    const [totalSize, setTotalSize] = useState<number>(0);
    const [progressValue, setProgressValue] = useState(0);
    const [isLimitExceeded, setIsLimitExceeded] = useState(false);
    const [isUploading, setIsUploading] = useState(false);

    const [isModalOpen, setIsModalOpen] = useState(false);
    const [hasQuickEnableFailed, setHasQuickEnableFailed] = useState(false);
    const [separatedFiles, setSeparatedFiles] = useState<SeparatedFiles>({
        successfullFiles: [],
        failedFiles: [],
    });
    const [countdown, setCountdown] = useState(3);
    const [successfulUpload, setSuccessfullUpload] = useState(false);
    const [isCountdownRunning, setIsCountdownRunning] = useState(false);

    const navigate = useNavigate();

    const theme = useMantineTheme();
    useDocumentTitle("KSv2 - Upload");

    const addFiles = (selectedFiles: File[]) => {
        let newFiles = selectedFiles.map((file) => {
            return {
                file: { fileHandle: file, id: uuidv4() },
                inputHasError: false,
            };
        });
        setInputFiles([...inputFiles, ...newFiles]);
    };

    const removeFile = (fileHandle: File) => {
        let others = inputFiles.filter((inputFile) => {
            if (inputFile.file.fileHandle === fileHandle) {
                return false;
            }
            return true;
        });
        setInputFiles(others);
    };

    const calcSize = useCallback(() => {
        let size = 0;
        inputFiles.forEach((inputFile) => {
            size += inputFile.file.fileHandle.size;
        });
        return size;
    }, [inputFiles]);

    useEffect(() => {
        if (countdown === 0) {
            return navigate("/user");
        }
        if (isCountdownRunning) {
            var timeout = setTimeout(() => {
                setCountdown((old) => old - 1);
            }, 1000);
        }
        return () => {
            clearTimeout(timeout);
        };
    }, [countdown, isCountdownRunning, navigate]);

    const upload = () => {
        if (!cookies.access_token) {
            return;
        }
        const formData = new FormData();
        inputFiles.forEach((inputFile, index) => {
            let { fileName, isPublic } = containerRefs.current[index];
            fileName =
                fileName.trim() === ""
                    ? inputFile.file.fileHandle.name
                    : fileName;

            inputFile.displayName = fileName;

            formData.append(
                isPublic ? fileName.concat("_p") : fileName.concat("_n"),
                inputFile.file.fileHandle
            );
        });

        setIsUploading(true);
        setIsModalOpen(true);
        setSuccessfullUpload(false);
        setIsCountdownRunning(false);
        setCountdown(3);

        ApiRequest.upload(formData, cookies.access_token, setProgressValue)
            .then(async ({ data }) => {
                const selectedGuildIds =
                    selectedGuildsRef.current.selectedGuildIds;

                if (selectedGuildIds.length > 0 && data.length > 0) {
                    await quickEnable(selectedGuildIds, data)
                        .then(() => {
                            setHasQuickEnableFailed(false);
                        })
                        .catch((err) => {
                            console.log(err);
                            setHasQuickEnableFailed(true);
                        });
                }

                const successfullFiles = inputFiles.filter(
                    (_f, idx) => data[idx].uploaded
                );
                const failedFiles = inputFiles.filter(
                    (_f, idx) => !data[idx].uploaded
                );

                setSeparatedFiles({
                    successfullFiles: successfullFiles,
                    failedFiles: failedFiles,
                });

                setInputFiles(failedFiles);

                if (successfullFiles.length > 0 && failedFiles.length === 0) {
                    setSuccessfullUpload(true);
                    setIsCountdownRunning(true);
                }
            })
            .catch((e) => {
                console.log(e);
                showNotification({
                    title: "Error",
                    message: "File upload failed!",
                    autoClose: false,
                    color: "red",
                    icon: <TbX size={24} />,
                });
            })
            .finally(() => {
                setIsUploading(false);
            });
    };

    const quickEnable = (guilds: string[], files: UploadedFile[]) => {
        if (!cookies.access_token) {
            return Promise.reject("Access token not set");
        }
        const bulk = {
            guilds: guilds,
            files: files.filter((f) => f.uploaded).map((f) => f.sound_file.id),
        };
        return ApiRequest.bulkEnable(bulk, cookies.access_token);
    };

    const handleInputErrors = (inputHasError: boolean, index: number) => {
        if (index >= 0 && index < inputFiles.length) {
            inputFiles[index].inputHasError = inputHasError;
            setInputFiles([...inputFiles]);
        }
    };

    const limitPercentage = useMemo(() => {
        return Math.round((totalSize / MAX_TOTAL_SIZE) * 100);
    }, [totalSize]);

    const hasInputErrors = () => {
        for (const inputFile of inputFiles) {
            if (inputFile.inputHasError) {
                return true;
            }
        }
        return false;
    };

    useEffect(() => {
        setTotalSize(calcSize());
    }, [inputFiles, calcSize]);

    useEffect(() => {
        if (totalSize > MAX_TOTAL_SIZE) {
            setIsLimitExceeded(true);
        } else {
            setIsLimitExceeded(false);
        }
    }, [totalSize]);

    return (
        <>
            <Group direction="column">
                <Paper
                    withBorder
                    shadow="sm"
                    p="sm"
                    className={classes.uploadPaperStyle}
                >
                    <Title order={3} pb="xs">
                        Upload
                    </Title>
                    <Group>
                        <Group direction="column" position="center">
                            <RingProgress
                                sections={[
                                    {
                                        value: limitPercentage,
                                        color: isLimitExceeded
                                            ? "red"
                                            : "violet",
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
                            <Button
                                disabled={
                                    isUploading ||
                                    isLimitExceeded ||
                                    inputFiles.length === 0 ||
                                    hasInputErrors()
                                }
                                onClick={() => upload()}
                            >
                                Upload
                            </Button>
                        </Group>
                        <Box>
                            <Group direction="column">
                                <Group>
                                    <Button onClick={() => openRef.current()}>
                                        Select files
                                    </Button>
                                </Group>
                            </Group>
                        </Box>
                        <Box className={classes.dropzoneBoxStyle}>
                            <Dropzone
                                onDrop={addFiles}
                                onReject={(file) =>
                                    console.log("rejected: ", file)
                                }
                                openRef={openRef}
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
                        {inputFiles.length > 0 ? (
                            <ScrollArea className={classes.scrollAreaStyle}>
                                <Group>
                                    {inputFiles.map((inputFile, index) => {
                                        return (
                                            <FileUploadContainer
                                                key={inputFile.file.id}
                                                ref={(ref) => {
                                                    if (ref) {
                                                        containerRefs.current[
                                                            index
                                                        ] = ref;
                                                    }
                                                }}
                                                file={inputFile.file.fileHandle}
                                                deleteCallback={(
                                                    file: File
                                                ) => {
                                                    removeFile(file);
                                                }}
                                                inputErrorCallback={(
                                                    hasErr: boolean
                                                ) => {
                                                    handleInputErrors(
                                                        hasErr,
                                                        index
                                                    );
                                                }}
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
            <UploadModal
                inputFiles={inputFiles}
                progressValue={progressValue}
                isOpen={isModalOpen}
                isUploading={isUploading}
                hasQuickEnableFailed={hasQuickEnableFailed}
                separatedFiles={separatedFiles}
                closeCallback={() => {
                    setIsModalOpen(false);
                }}
                countdown={countdown}
                successfulUpload={successfulUpload}
            />
        </>
    );
}
