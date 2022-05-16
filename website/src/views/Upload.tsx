import { ChangeEvent, ReactNode, useContext, useRef, useState } from "react";
import {
    Box,
    Button,
    Center,
    Container,
    Grid,
    Group,
    Progress,
    RingProgress,
    ScrollArea,
    Stack,
    Text,
} from "@mantine/core";
import { Dropzone, FullScreenDropzone } from "@mantine/dropzone";
import { FileUploadContainer, FileContainerRef } from "../components/FileContainer";
import { AuthContext } from "../auth/AuthProvider";
import axios from "axios";
import { API_URL, FilesRoute } from "../api/ApiRoutes";

const MAX_TOTAL_SIZE = 10_000_000;

export default function Upload() {
    const { tokens } = useContext(AuthContext);
    const openRef = useRef<() => void>(() => { });
    const containerRefs = useRef<FileContainerRef[]>([]);
    const [files, setFiles] = useState<File[]>([]);
    const [totalSize, setTotalSize] = useState<number>(0);
    const [progressValue, setProgressValue] = useState(0);
    const [isUploading, setIsUploading] = useState(false);

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
            <Group>
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
                <Stack>
                    <Group>
                        <Button onClick={() => openRef.current()}>
                            Select files
                        </Button>
                        <Button onClick={() => upload()}>Upload</Button>
                    </Group>
                    <Progress animate value={progressValue} />
                </Stack>
            </Group>

            {/*WARN: fix height*/}
            {/* <Box p="xs" m="xs" style={{ height: '80vh', overflow: 'auto' }}> */}
            <Box p="xs" m="xs">
                <Dropzone
                    style={{ display: "none" }}
                    onDrop={addFiles}
                    openRef={openRef}
                >
                    {(_status) => {
                        console.log(_status);
                        return <>Brofist</>;
                    }}
                </Dropzone>
                <Grid>
                    {files.map((file: File, index) => {
                        return (
                            // TODO: width
                            <Grid.Col xs={12} sm={6} md={4}>
                                <FileUploadContainer
                                    ref={(ref) => {
                                        if (ref) {
                                            containerRefs.current[index] = ref;
                                        }
                                    }}
                                    disabled={isUploading}
                                    file={file}
                                    key={index}
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
