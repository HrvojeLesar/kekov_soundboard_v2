import {
    Box,
    Checkbox,
    createStyles,
    Grid,
    Group,
    LoadingOverlay,
    Paper,
    ScrollArea,
    Text,
    Title,
} from "@mantine/core";
import { useDocumentTitle } from "@mantine/hooks";
import { showNotification } from "@mantine/notifications";
import { CSSProperties, useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { TbX } from "react-icons/tb";
import { COOKIE_NAMES } from "../auth/AuthProvider";
import SearchBar from "../components/SearchBar";
import DeleteFile from "../components/UserFiles/DeleteFile";
import ServerSelect from "../components/UserFiles/ServerSelect";
import SelectableFileContainer from "../components/UserFiles/UserFileContainer";
import { ApiRequest, LOADINGOVERLAY_ZINDEX, SoundFile } from "../utils/utils";

export enum UserFilesModalType {
    Add,
    Edit,
    Delete,
}

export const userFilesMaximumWindowHeight: CSSProperties = {
    height: "calc(100vh - 34px)",
};

const useStyle = createStyles((theme) => {
    return {
        paperStyle: {
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
            position: "relative",
            ...userFilesMaximumWindowHeight,
        },
        scollAreaStyle: {
            height: "100%",
        },
        userFilesGroupStyle: {
            width: "100%",
            display: "flex",
            flexDirection: "column",
            gap: theme.spacing.sm,
            ...userFilesMaximumWindowHeight,
        },
        userFilesPaperStyle: {
            height: "20%",
            width: "100%",
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
        },
        userFilesTitleStyle: {
            textOverflow: "ellipsis",
            overflow: "hidden",
            whiteSpace: "nowrap",
        },
    };
});

let abortController: AbortController | undefined = undefined;

export default function UserFiles() {
    const [cookies] = useCookies(COOKIE_NAMES);
    const [files, setFiles] = useState<SoundFile[]>([]);
    const [isFetching, setIsFetching] = useState(true);
    const [selectedFile, setSelectedFile] = useState<SoundFile | undefined>(
        undefined
    );
    const [filterTerm, setFilterTerm] = useState("");
    const { classes } = useStyle();
    useDocumentTitle("KSv2 - Your files");

    const fetchFiles = async () => {
        if (cookies.access_token) {
            try {
                const { data } = await ApiRequest.getUserFiles(
                    cookies.access_token
                );
                data.sort((a, b) => {
                    return Date.parse(a.time_added) - Date.parse(b.time_added);
                });
                setIsFetching(false);
                setFiles(data);
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    const getEditTitle = () => {
        return selectedFile !== undefined
            ? `Edit: ${selectedFile.display_name}`
            : "Edit";
    };

    const deleteFile = (file: SoundFile): Promise<void> => {
        return new Promise((resolve, reject) => {
            if (cookies.access_token) {
                ApiRequest.deleteUserFile(file.id, cookies.access_token)
                    .then(({ data }) => {
                        resolve();
                        setSelectedFile(undefined);
                        setFiles([
                            ...files.filter((file) => {
                                return file.id !== data.id;
                            }),
                        ]);
                    })
                    .catch((e) => {
                        reject(e);
                    });
            } else {
                reject("No access token present!");
            }
        });
    };

    const filterFiles = () => {
        if (filterTerm !== "") {
            return files.filter((file) => {
                if (file.display_name) {
                    return (
                        file.display_name.toLowerCase().indexOf(filterTerm) !==
                        -1
                    );
                } else {
                    return false;
                }
            });
        } else {
            return files;
        }
    };

    const toggleFileVisibility = (file: SoundFile) => {
        if (cookies.access_token) {
            abortController = new AbortController();
            ApiRequest.toggleFileVisibility(
                file.id,
                cookies.access_token,
                abortController
            )
                .then(({ data }) => {
                    setFiles([
                        ...files.map((f) => {
                            if (f.id === data.id) {
                                f.is_public = data.is_public;
                            }
                            return f;
                        }),
                    ]);
                })
                .catch((e) => {
                    console.log(e);
                    showNotification({
                        title: "Error",
                        message: "Failed to toggle files visibility!",
                        autoClose: false,
                        color: "red",
                        icon: <TbX size={24} />,
                    });
                });
        }
    };

    useEffect(() => {
        fetchFiles();
    }, []);

    useEffect(() => {
        abortController?.abort();
    }, [selectedFile]);

    return (
        <>
            <Grid>
                <Grid.Col xs={9}>
                    <Paper
                        withBorder
                        shadow="sm"
                        p="sm"
                        className={classes.paperStyle}
                    >
                        <Title order={3} pb="xs">
                            Your files
                        </Title>
                        <Box py="sm">
                            <SearchBar
                                onSearch={(searchValue) => {
                                    setSelectedFile(undefined);
                                    setFilterTerm(searchValue);
                                }}
                            />
                        </Box>
                        <LoadingOverlay
                            zIndex={LOADINGOVERLAY_ZINDEX}
                            visible={isFetching}
                        />
                        <ScrollArea className={classes.scollAreaStyle}>
                            <Group>
                                {files.length > 0
                                    ? !isFetching &&
                                      filterFiles().map((file) => {
                                          return (
                                              <SelectableFileContainer
                                                  key={file.id}
                                                  file={file}
                                                  isSelected={
                                                      selectedFile?.id ===
                                                      file.id
                                                  }
                                                  onClickCallback={(f) => {
                                                      setSelectedFile(f);
                                                  }}
                                              />
                                          );
                                      })
                                    : !isFetching && (
                                          <Text size="xl" weight="bold">
                                              You have no uploaded files.
                                          </Text>
                                      )}
                            </Group>
                        </ScrollArea>
                    </Paper>
                </Grid.Col>
                <Grid.Col xs={3}>
                    <Box className={classes.userFilesGroupStyle}>
                        <Paper
                            withBorder
                            shadow="sm"
                            p="sm"
                            className={classes.userFilesPaperStyle}
                        >
                            <Group noWrap position="apart" pb="xs">
                                <Title
                                    order={3}
                                    title={getEditTitle()}
                                    className={classes.userFilesTitleStyle}
                                >
                                    {getEditTitle()}
                                </Title>
                                {selectedFile && (
                                    <DeleteFile
                                        deleteCallback={() =>
                                            deleteFile(selectedFile)
                                        }
                                        file={selectedFile}
                                    />
                                )}
                            </Group>
                            {/*TODO: Add delete, toggle public, private*/}
                            {selectedFile !== undefined ? (
                                <Group>
                                    <Text>Set file visibility:</Text>
                                    <Checkbox
                                        label="Public"
                                        checked={selectedFile.is_public}
                                        onChange={() => {
                                            toggleFileVisibility(selectedFile);
                                        }}
                                    />
                                </Group>
                            ) : (
                                "No file selected"
                            )}
                        </Paper>
                        <ServerSelect file={selectedFile} />
                    </Box>
                </Grid.Col>
            </Grid>
        </>
    );
}
