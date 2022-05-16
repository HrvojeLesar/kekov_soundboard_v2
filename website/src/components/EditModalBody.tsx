import { Button, Group, Stack, Text, TextInput } from "@mantine/core";
import axios from "axios";
import { useContext, useState } from "react";
import { API_URL, UserRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import { UserFile } from "../views/UserFiles";

type EditModalBodyProps = {
    file: UserFile | undefined;
    closeModalCallback: () => void;
    editSuccessCallback: () => void;
};

export default function EditModalBody({
    file,
    closeModalCallback,
    editSuccessCallback,
}: EditModalBodyProps) {
    const [displayName, setDisplayName] = useState(file?.display_name);
    const [isSaveInProgress, setIsSaveInProgress] = useState(false);

    const saveEdit = () => {};

    return (
        <Stack m="sm">
            <Group>
                <TextInput
                    value={displayName}
                    placeholder={file?.display_name}
                    label="Labela"
                    onChange={(e) => setDisplayName(e.target.value)}
                />
            </Group>
            <Group position="right">
                <Button
                    disabled={isSaveInProgress}
                    onClick={() => closeModalCallback()}
                >
                    Cancel
                </Button>
                <Button
                    disabled={isSaveInProgress}
                    onClick={() => {
                        setIsSaveInProgress(true);
                    }}
                    color="red"
                >
                    Confirm
                </Button>
            </Group>
        </Stack>
    );
}
