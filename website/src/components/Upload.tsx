import { ChangeEvent, ReactNode, useContext, useRef, useState } from "react";
import {
    Button,
    Group,
    Input,
    RingProgress,
    Stack,
    Text,
    TextInput,
} from "@mantine/core";
import { Dropzone, FullScreenDropzone } from "@mantine/dropzone";
import { FileUploadContainer, FileContainerRef } from "./FileContainer";

const MAX_TOTAL_SIZE = 10_000_000;

export default function Upload() {
    const openRef = useRef<() => void>(() => { });
    const containerRefs = useRef<FileContainerRef[]>([]);
    const [files, setFiles] = useState<File[]>([]);
    const [totalSize, setTotalSize] = useState<number>(0);

    // TODO: cull duplicates
    const addFiles = (selectedFiles: File[]) => {
        let size = totalSize;
        console.log(size);
        selectedFiles.forEach((file) => size += file.size);
        console.log(size);
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

    const startUpload = () => {
        containerRefs.current.forEach((fileContainer) =>
            fileContainer.startUpload()
        );
    };

    // <Input type="file" multiple accept={IMAGE_MIME_TYPE.join()} onClick={() => openRef.current()} />
    return (
        <>
            {/* Hidden dropzone for cheating opening a file explorer*/}
            <Dropzone
                style={{ display: "none" }}
                onDrop={addFiles}
                openRef={openRef}
            >
                {(_status) => {
                    return <></>;
                }}
            </Dropzone>
            <FullScreenDropzone
                accept={["*"]}
                onDrop={(files) => addFiles(files)}
            >
                {(_status) => <p>Message</p>}
            </FullScreenDropzone>
            <Stack>
                {files.map((file: File, index) => {
                    return (
                        <FileUploadContainer
                            ref={(ref) => {
                                if (ref) {
                                    containerRefs.current[index] = ref;
                                }
                            }}
                            file={file}
                            key={index}
                            deleteCallback={(file: File) => removeFile(file)}
                        />
                    );
                })}
            </Stack>
            <Button onClick={() => openRef.current()}>Select files</Button>
            <Button onClick={() => startUpload()}>Upload</Button>
            {/* TODO: Animate ring ?? */}
            <RingProgress
                sections={[
                    {
                        value: Math.round((totalSize / MAX_TOTAL_SIZE) * 100),
                        color: "blue",
                    },
                ]}
                label={<Text align="center">Limit</Text>}
            />
        </>
    );
}
