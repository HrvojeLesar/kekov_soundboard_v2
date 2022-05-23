import { Button, Grid, Group, Paper, ScrollArea, Title } from "@mantine/core";
import axios from "axios";
import { CSSProperties, useContext, useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { API_URL, UserRoute } from "../api/ApiRoutes";
import { AuthContext, COOKIE_NAMES } from "../auth/AuthProvider";
import DeleteFile from "../components/UserFiles/DeleteFile";
import ServerSelect from "../components/UserFiles/ServerSelect";
import UserFileContainer from "../components/UserFiles/UserFileContainer";

export type UserFile = {
    id: string;
    display_name: string;
};

export enum UserFilesModalType {
    Add,
    Edit,
    Delete,
}

export const userFilesMaximumWindowHeight: CSSProperties = {
    height: "calc(100vh - 34px)",
};

export default function UserFiles() {
    const [cookies] = useCookies(COOKIE_NAMES);
    const [files, setFiles] = useState<UserFile[]>([]);
    const [isFetching, setIsFetching] = useState(true);
    const [selectedIndex, setSelectedIndex] = useState<number | undefined>(
        undefined
    );

    const fetchFiles = async () => {
        if (cookies.access_token) {
            try {
                const { data } = await axios.get<UserFile[]>(
                    `${API_URL}${UserRoute.getFiles}`,
                    {
                        headers: { authorization: `${cookies.access_token}` },
                    }
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
        return selectedIndex !== undefined
            ? `Edit: ${files[selectedIndex].display_name}`
            : "Edit";
    };

    const deleteFile = (file: UserFile): Promise<void> => {
        return new Promise((resolve, reject) => {
            if (cookies.access_token) {
                axios
                    .delete<UserFile>(
                        `${API_URL}${UserRoute.deleteFile}${file.id}`,
                        {
                            headers: {
                                authorization: `${cookies.access_token}`,
                            },
                        }
                    )
                    .then(({ data }) => {
                        resolve();
                        setSelectedIndex(undefined);
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
                        style={{
                            display: "flex",
                            flexDirection: "column",
                            overflow: "hidden",
                            ...userFilesMaximumWindowHeight,
                        }}
                    >
                        <Title order={3} pb="xs">
                            Your files
                        </Title>
                        <ScrollArea style={{ height: "100%" }}>
                            <Group>
                                {!isFetching &&
                                    files.length > 0 &&
                                    files.map((file, index) => {
                                        return (
                                            <UserFileContainer
                                                key={file.id}
                                                file={file}
                                                isSelected={
                                                    selectedIndex === index
                                                }
                                                onClickCallback={() => {
                                                    setSelectedIndex(index);
                                                }}
                                            />
                                        );
                                    })}
                            </Group>
                        </ScrollArea>
                    </Paper>
                </Grid.Col>
                <Grid.Col xs={3}>
                    <Group
                        direction="column"
                        style={{
                            width: "100%",
                            ...userFilesMaximumWindowHeight,
                        }}
                    >
                        <Paper
                            withBorder
                            shadow="sm"
                            p="sm"
                            style={{
                                height: "20%",
                                width: "100%",
                                display: "flex",
                                flexDirection: "column",
                                overflow: "hidden",
                            }}
                        >
                            <Title
                                order={3}
                                pb="xs"
                                title={getEditTitle()}
                                style={{
                                    textOverflow: "ellipsis",
                                    overflow: "hidden",
                                    whiteSpace: "nowrap",
                                }}
                            >
                                {getEditTitle()}
                            </Title>
                            {/*TODO: Add delete, toggle public, private*/}
                            {selectedIndex !== undefined ? (
                                <DeleteFile
                                    deleteCallback={() =>
                                        deleteFile(files[selectedIndex])
                                    }
                                    file={files[selectedIndex]}
                                />
                            ) : (
                                "No file selected"
                            )}
                        </Paper>
                        <Paper
                            withBorder
                            shadow="sm"
                            p="sm"
                            style={{
                                flexGrow: 1,
                                width: "100%",
                                display: "flex",
                                flexDirection: "column",
                                overflow: "hidden",
                            }}
                        >
                            <Title order={3} pb="xs">
                                Servers
                            </Title>
                            <ScrollArea>
                                {selectedIndex !== undefined && (
                                    <ServerSelect file={files[selectedIndex]} />
                                )}
                            </ScrollArea>
                        </Paper>
                    </Group>
                </Grid.Col>
            </Grid>
        </>
    );
}
