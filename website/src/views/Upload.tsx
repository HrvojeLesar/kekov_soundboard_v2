import { ChangeEvent, ComponentProps, ReactNode, useContext, useRef, useState } from "react";
import {
    Box,
    Button,
    Center,
    Container,
    Grid,
    Group,
    MantineTheme,
    Progress,
    RingProgress,
    ScrollArea,
    Stack,
    Text,
    useMantineTheme,
} from "@mantine/core";
import {
    Dropzone,
    DropzoneStatus,
    FullScreenDropzone,
    IMAGE_MIME_TYPE,
} from "@mantine/dropzone";
import {
    FileUploadContainer,
    FileContainerRef,
} from "../components/FileContainer";
import { AuthContext } from "../auth/AuthProvider";
import axios from "axios";
import { API_URL, FilesRoute } from "../api/ApiRoutes";
import { Files, FileUpload, Icon, X } from "tabler-icons-react";

const MAX_TOTAL_SIZE = 10_000_000;

const getIconColor = (status: DropzoneStatus, theme: MantineTheme) => {
    return status.accepted
        ? theme.colors[theme.primaryColor][theme.colorScheme === "dark" ? 4 : 6]
        : status.rejected
            ? theme.colors.red[theme.colorScheme === "dark" ? 4 : 6]
            : theme.colorScheme === "dark"
                ? theme.colors.dark[0]
                : theme.colors.gray[7];
};

const UploadIcon = ({ status, ...props }: ComponentProps<Icon> & { status: DropzoneStatus }) => {
    if (status.rejected) {
        return <X {...props} />;
    }

    return <FileUpload {...props} />;
}

export default function Upload() {
    const { tokens } = useContext(AuthContext);
    const openRef = useRef<() => void>(() => { });
    const containerRefs = useRef<FileContainerRef[]>([]);
    const [files, setFiles] = useState<File[]>([]);
    const [totalSize, setTotalSize] = useState<number>(0);
    const [progressValue, setProgressValue] = useState(0);
    const [isUploading, setIsUploading] = useState(false);
    const theme = useMantineTheme();

    // TODO: cull duplicates
    const addFiles = (selectedFiles: File[]) => {
        console.log(files);
        console.log(selectedFiles);
        let size = totalSize;
        selectedFiles.forEach((file) => (size += file.size));
        setTotalSize(size);
        setFiles([...files, ...selectedFiles]);
    };

    const removeFile = (file: File) => {
        let size = totalSize;
        console.log(size);
        let others = files.filter((f) => {
            if (f == file) {
                size -= file.size;
                return false;
            }
            return true;
        });
        console.log(size);
        setTotalSize(size);
        setFiles(others);
    };

    const upload = async () => {
        if (tokens?.access_token) {
            try {
                const formData = new FormData();
                files.forEach((file, index) => {
                    let { fileName } = containerRefs.current[index];
                    fileName = fileName.trim() == "" ? file.name : fileName;
                    formData.append(fileName, file);
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
                        console.log("Gotovo");
                    });
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    // <Input type="file" multiple accept={IMAGE_MIME_TYPE.join()} onClick={() => openRef.current()} />
    return (
        <>
            <Group mx="auto">
                {/* TODO: Animate ring ?? */}
                <RingProgress
                    sections={[
                        {
                            value: Math.round(
                                (totalSize / MAX_TOTAL_SIZE) * 100
                            ),
                            color: "blue",
                        },
                    ]}
                    label={<Text align="center">Limit</Text>}
                />
                <Box style={{ width: "300px" }}>
                    <Group direction="column">
                        <Group position="apart" style={{ width: "100%" }}>
                            <Button onClick={() => openRef.current()}>
                                Select files
                            </Button>
                            <Button onClick={() => upload()}>Upload</Button>
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
                        onDrop={addFiles}
                        openRef={openRef}
                        accept={IMAGE_MIME_TYPE}
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

            {/*WARN: fix height*/}
            {/* <Box p="xs" m="xs" style={{ height: '80vh', overflow: 'auto' }}> */}
            <Box p="xs" m="xs">
                <Grid>
                    {files.map((file: File, index) => {
                        return (
                            // TODO: width
                            <Grid.Col xs={12} sm={6} md={4} key={index}>
                                <FileUploadContainer
                                    ref={(ref) => {
                                        if (ref) {
                                            containerRefs.current[index] = ref;
                                        }
                                    }}
                                    disabled={isUploading}
                                    file={file}
                                    deleteCallback={(file: File) =>
                                        removeFile(file)
                                    }
                                />
                            </Grid.Col>
                        );
                    })}
                </Grid>
            </Box>
        </>
    );
}
