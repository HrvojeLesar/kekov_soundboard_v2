import {
    Box,
    Button,
    Grid,
    Group,
    Modal,
    Pagination,
    Paper,
    Skeleton,
    Stack,
    Table,
    Text,
    Title,
} from "@mantine/core";
import { useHover, usePagination } from "@mantine/hooks";
import axios from "axios";
import { useContext, useEffect, useState } from "react";
import { API_URL, UserRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import AddModalBody from "../components/AddModalBody";
import DeleteModalBody from "../components/DeleteModalBody";
import EditModalBody from "../components/EditModalBody";
import { GuildToggle } from "../components/GuildToggle";
import UserFileContainer from "../components/UserFileContainer";

export type UserFile = {
    id: string;
    display_name: string;
};

export enum UserFilesModalType {
    Add,
    Edit,
    Delete,
}

export default function UserFiles() {
    const { tokens } = useContext(AuthContext);
    const [files, setFiles] = useState<UserFile[]>([]);
    const [isFetching, setIsFetching] = useState(true);
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [modalType, setModalType] = useState<UserFilesModalType>();
    const [currentFile, setCurrentFile] = useState<UserFile>();

    const fetchFiles = async () => {
        if (tokens?.access_token) {
            try {
                const { data } = await axios.get<UserFile[]>(
                    `${API_URL}${UserRoute.getFiles}`,
                    {
                        headers: { authorization: `${tokens.access_token}` },
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

    useEffect(() => {
        fetchFiles();
    }, []);

    const renderModal = () => {
        switch (modalType) {
            case UserFilesModalType.Add: {
                if (currentFile) {
                    return <AddModalBody file={currentFile} />;
                }
                // WARN: This should be unreachable
                console.log("Unreachable!");
                return <></>;
            }
            case UserFilesModalType.Edit: {
                return (
                    <EditModalBody
                        file={currentFile}
                        closeModalCallback={() => setIsModalOpen(false)}
                        editSuccessCallback={() => { }}
                    />
                );
            }
            case UserFilesModalType.Delete: {
                return (
                    <DeleteModalBody
                        file={currentFile}
                        closeModalCallback={() => setIsModalOpen(false)}
                        deletionSuccessCallback={() => {
                            setIsModalOpen(false);
                            setFiles(files.filter((f) => f !== currentFile));
                        }}
                    />
                );
            }
        }
    };

    const openModal = (type: UserFilesModalType, file: UserFile) => {
        setModalType(type);
        setCurrentFile(file);
        setIsModalOpen(true);
    };

    const modalTitle = () => {
        switch (modalType) {
            case UserFilesModalType.Add: {
                return "Add to server";
            }
            case UserFilesModalType.Edit: {
                return "Edit";
            }
            case UserFilesModalType.Delete: {
                return "Delete confirmation!";
            }
        }
    };

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
                            height: "calc(100vh - 255px)",
                            display: "flex",
                            flexDirection: "column",
                            overflow: "hidden",
                        }}
                    >
                        <Title order={3} pb="xs">
                            Title
                        </Title>
                    </Paper>
                </Grid.Col>
                <Grid.Col xs={9}>
                </Grid.Col>
            </Grid>
            <Modal
                overflow="inside"
                title={modalTitle()}
                opened={isModalOpen}
                onClose={() => setIsModalOpen(false)}
            >
                {renderModal()}
            </Modal>
            {isFetching && <Skeleton>placeholder</Skeleton>}
            {!isFetching && files.length > 0 &&
                files.map((file) => {
                    return (
                        <UserFileContainer
                            key={file.id}
                            file={file}
                            openModal={openModal}
                        />
                    );
                })
            }
        </>
    );
}
