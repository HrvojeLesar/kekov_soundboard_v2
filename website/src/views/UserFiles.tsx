import {
    Box,
    createStyles,
    Grid,
    Group,
    LoadingOverlay,
    Paper,
    ScrollArea,
    Title,
} from "@mantine/core";
import { useDocumentTitle } from "@mantine/hooks";
import { CSSProperties, useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { COOKIE_NAMES } from "../auth/AuthProvider";
import SearchBar from "../components/SearchBar";
import DeleteFile from "../components/UserFiles/DeleteFile";
import ServerSelect from "../components/UserFiles/ServerSelect";
import UserFileContainer from "../components/UserFiles/UserFileContainer";
import { ApiRequest, LOADINGOVERLAY_ZINDEX, UserFile } from "../utils/utils";

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

export default function UserFiles() {
    const [cookies] = useCookies(COOKIE_NAMES);
    const [files, setFiles] = useState<UserFile[]>([]);
    const [isFetching, setIsFetching] = useState(true);
    const [selectedFile, setSelectedFile] = useState<UserFile | undefined>(
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

    const deleteFile = (file: UserFile): Promise<void> => {
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

    useEffect(() => {
        fetchFiles();
    }, []);

    // WARN: Make performant
    // TODO: Make performant
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
                                filterCallback={(searchValue) => {
                                    setSelectedFile(undefined);
                                    setFilterTerm(searchValue);
                                }}
                            />
                        </Box>
                        <LoadingOverlay zIndex={LOADINGOVERLAY_ZINDEX} visible={isFetching} />
                        <ScrollArea className={classes.scollAreaStyle}>
                            <Group>
                            {!isFetching &&
                                    filterFiles().map((file) => {
                                        return (
                                            <UserFileContainer
                                                key={file.id}
                                                file={file}
                                                isSelected={
                                                    selectedFile?.id === file.id
                                                }
                                                onClickCallback={(f) => {
                                                    setSelectedFile(f);
                                                }}
                                            />
                                        );
                                    })}
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
                            <Title
                                order={3}
                                pb="xs"
                                title={getEditTitle()}
                                className={classes.userFilesTitleStyle}
                            >
                                {getEditTitle()}
                            </Title>
                            {/*TODO: Add delete, toggle public, private*/}
                            {selectedFile !== undefined ? (
                                <DeleteFile
                                    deleteCallback={() =>
                                        deleteFile(selectedFile)
                                    }
                                    file={selectedFile}
                                />
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
