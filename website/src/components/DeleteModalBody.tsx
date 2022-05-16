import { Button, Group, Stack, Text } from "@mantine/core";
import axios from "axios";
import { useContext, useState } from "react";
import { API_URL, UserRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import { UserFile } from "../views/UserFiles";

type DeleteModalBodyProps = {
    file: UserFile | undefined;
    closeModalCallback: () => void;
    deletionSuccessCallback: () => void;
};

export default function DeleteModalBody({
    file,
    closeModalCallback,
    deletionSuccessCallback,
}: DeleteModalBodyProps) {
    const { tokens } = useContext(AuthContext);
    const [isDeletionInProgress, setIsDeletionInProgress] = useState(false);

    const deleteFile = () => {
        axios
            .delete<UserFile>(`${API_URL}${UserRoute.deleteFile}${file?.id}`, {
                headers: { authorization: `${tokens?.access_token}` },
            })
            .then((resp) => {
                console.log(resp);
                deletionSuccessCallback();
            })
            .catch((err) => {
                // TODO: Notify
                console.log(err);
            });
    };

    return (
        <Stack m="sm">
            <Text>{`Are you sure you want to delete ${file?.display_name}?`}</Text>
            <Group position="right">
                <Button
                    disabled={isDeletionInProgress}
                    onClick={() => closeModalCallback()}
                >
                    Cancel
                </Button>
                <Button
                    disabled={isDeletionInProgress}
                    onClick={() => { setIsDeletionInProgress(true); deleteFile(); }}
                    color="red"
                >
                    Confirm
                </Button>
            </Group>
        </Stack>
    );
}
