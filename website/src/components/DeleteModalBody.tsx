import { Button, Group, Stack, Text } from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import { useState } from "react";
import { X } from "tabler-icons-react";
import { GuildFile } from "../views/Guild";
import { UserFile } from "../views/UserFiles";

type DeleteModalBodyProps = {
    file: UserFile | GuildFile;
    closeCallback: () => void;
    deleteCallback: () => Promise<void>;
};

export default function DeleteModalBody({
    file,
    closeCallback,
    deleteCallback,
}: DeleteModalBodyProps) {
    const [isDeletionInProgress, setIsDeletionInProgress] = useState(false);

    const handleDelete = () => {
        setIsDeletionInProgress(true);
        deleteCallback()
            .then(() => {
                showNotification({
                    title: "File deleted",
                    message: "File has been successfully deleted!",
                    autoClose: 3000,
                    color: "green",
                });
                closeCallback();
            })
            .catch((e) => {
                console.log(e);
                showNotification({
                    title: "Deletion failed",
                    message: "Failed to delete selected file",
                    autoClose: 5000,
                    color: "red",
                    icon: <X />,
                });
            })
            .finally(() => {
                setIsDeletionInProgress(false);
            });
    };

    return (
        <Stack m="sm">
            <Text>{`Are you sure you want to delete ${file.display_name}?`}</Text>
            <Group position="right">
                <Button
                    disabled={isDeletionInProgress}
                    onClick={() => {
                        closeCallback();
                    }}
                >
                    Cancel
                </Button>
                <Button
                    loading={isDeletionInProgress}
                    onClick={() => handleDelete()}
                    color="red"
                >
                    Confirm
                </Button>
            </Group>
        </Stack>
    );
}
