import {
    ActionIcon,
    Box,
    createStyles,
    Group,
    Modal,
    Paper,
    Popover,
    Progress,
    ScrollArea,
    Text,
    Title,
} from "@mantine/core";
import { useState } from "react";
import { FaInfo } from "react-icons/fa";
import { primaryShade } from "../../utils/utils";
import { InputFile, SeparatedFiles } from "../../views/Upload";

const useStyles = createStyles((theme) => {
    const shade = primaryShade(theme);
    return {
        successContainerStyle: {
            paddingTop: "2px",
            flexBasis: "100%",
            height: "300px",
            borderColor: theme.colors.green[5],
            borderWidth: 2,
            display: "flex",
            flexDirection: "column",
        },
        failedContainerStyle: {
            paddingTop: "2px",
            flexBasis: "100%",
            height: "300px",
            borderColor: theme.colors.red[6],
            borderWidth: 2,
            display: "flex",
            flexDirection: "column",
        },
        scrollAreaContainer: {
            display: "flex",
            flexGrow: 1,
            overflow: "hidden",
        },
        scrollAreaStyle: {
            width: "100%",
        },
        modalItemStyle: {
            alignItems: "center",
            paddingTop: "5px",
            paddingBottom: "5px",
            marginBottom: "1px",

            "&:hover": {
                cursor: "default",
                transition: "150ms ease",
                borderRadius: "3px",
                backgroundColor:
                    theme.colorScheme === "dark"
                        ? theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          )
                        : theme.fn.rgba(
                              theme.colors[theme.primaryColor][shade],
                              0.3
                          ),
            },
        },
        popoverPaperStyle: {
            background: "transparent",
        },
    };
});

type ModalItemProps = {
    idx: number;
    file: InputFile;
};

export function ModalItem({ file, idx }: ModalItemProps) {
    const { classes } = useStyles();
    return (
        <Box className={classes.modalItemStyle}>
            {`${idx}. `}
            {file.displayName ?? file.file.fileHandle.name}
        </Box>
    );
}

type UploadModalProps = {
    progressValue: number;
    isOpen: boolean;
    closeCallback: () => void;
    inputFiles: InputFile[];
    isUploading: boolean;
    hasQuickEnableFailed: boolean;
    separatedFiles: SeparatedFiles;
    countdown: number;
    successfulUpload: boolean;
};

export default function UploadModal({
    progressValue,
    isOpen,
    closeCallback,
    inputFiles,
    isUploading,
    hasQuickEnableFailed,
    separatedFiles,
    countdown,
    successfulUpload,
}: UploadModalProps) {
    const { classes } = useStyles();
    const [popoverOpened, setPopoverOpened] = useState(false);

    return (
        <Modal
            centered
            size="lg"
            withCloseButton={!isUploading && !successfulUpload}
            closeOnClickOutside={false}
            closeOnEscape={false}
            opened={isOpen}
            onClose={() => {
                setPopoverOpened(false);
                closeCallback();
            }}
            title={
                isUploading
                    ? `Uploading ${inputFiles.length} ${
                          inputFiles.length === 1 ? "file" : "files"
                      }`
                    : successfulUpload
                    ? "Upload successful"
                    : "Summary"
            }
        >
            {isUploading && (
                <>
                    <Text pb="xs" align="center">
                        Upload progress: {progressValue}%
                    </Text>
                    <Progress animate value={progressValue} />
                </>
            )}
            {!isUploading && hasQuickEnableFailed && (
                <Text pb="xs" align="center" color="red">
                    Failed to add files to selected servers. You'll have to do
                    this manually.
                </Text>
            )}
            {!isUploading && successfulUpload && (
                <Text pb="xs" align="center" color="green">
                    Files have been successfully uploaded. You'll be redirected
                    in:
                    <Text component="span" color="violet">
                        {` ${countdown} `}
                    </Text>
                    seconds.
                </Text>
            )}
            {!isUploading && !successfulUpload && (
                <Group direction="row" noWrap>
                    {separatedFiles.successfullFiles.length > 0 && (
                        <Paper
                            my="xs"
                            px="xs"
                            withBorder
                            className={classes.successContainerStyle}
                        >
                            <Title order={3} pb="xs">
                                Uploaded
                            </Title>
                            <Box className={classes.scrollAreaContainer}>
                                <ScrollArea className={classes.scrollAreaStyle}>
                                    {separatedFiles.successfullFiles.map(
                                        (f, idx) => {
                                            return (
                                                <ModalItem
                                                    key={f.file.id}
                                                    idx={idx + 1}
                                                    file={f}
                                                />
                                            );
                                        }
                                    )}
                                </ScrollArea>
                            </Box>
                        </Paper>
                    )}
                    {separatedFiles.failedFiles.length > 0 && (
                        <Paper
                            my="xs"
                            px="xs"
                            withBorder
                            className={classes.failedContainerStyle}
                        >
                            <Group position="apart" pb="xs">
                                <Title order={3}>Failed to upload</Title>
                                <Popover
                                    withArrow
                                    opened={popoverOpened}
                                    width={250}
                                    target={
                                        <ActionIcon
                                            onClick={() =>
                                                setPopoverOpened((o) => !o)
                                            }
                                        >
                                            <FaInfo />
                                        </ActionIcon>
                                    }
                                >
                                    <Paper
                                        className={classes.popoverPaperStyle}
                                    >
                                        <Text size="sm">
                                            Listed files failed to upload. Try
                                            again or try another file type.
                                        </Text>
                                        <Text size="sm">
                                            Failed files have been left in
                                            selected files.
                                        </Text>
                                    </Paper>
                                </Popover>
                            </Group>
                            <Box className={classes.scrollAreaContainer}>
                                <ScrollArea className={classes.scrollAreaStyle}>
                                    {separatedFiles.failedFiles.map(
                                        (f, idx) => {
                                            return (
                                                <ModalItem
                                                    key={f.file.id}
                                                    idx={idx + 1}
                                                    file={f}
                                                />
                                            );
                                        }
                                    )}
                                </ScrollArea>
                            </Box>
                        </Paper>
                    )}
                </Group>
            )}
        </Modal>
    );
}
