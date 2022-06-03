import { ActionIcon, Modal } from "@mantine/core";
import { useState } from "react";
import { Trash } from "tabler-icons-react";
import { GuildFile, MODAL_ZINDEX, UserFile } from "../../utils/utils";
import DeleteModalBody from "../DeleteModalBody";

type DeleteFileProps = {
    file: UserFile | GuildFile;
    deleteCallback: () => Promise<void>;
};

export default function DeleteFile({ file, deleteCallback }: DeleteFileProps) {
    const [isModalOpen, setIsModalOpen] = useState(false);

    return (
        <>
            <Modal
                zIndex={MODAL_ZINDEX}
                opened={isModalOpen}
                withCloseButton={false}
                closeOnClickOutside={false}
                closeOnEscape={false}
                centered
                onClose={() => setIsModalOpen(false)}
                title={file.display_name}
                styles={{
                    title: {
                        maxWidth: "15ch",
                        textOverflow: "ellipsis",
                    },
                }}
            >
                <DeleteModalBody
                    file={file}
                    closeCallback={() => setIsModalOpen(false)}
                    deleteCallback={deleteCallback}
                />
            </Modal>
            <ActionIcon onClick={() => setIsModalOpen(true)} color="red">
                <Trash />
            </ActionIcon>
        </>
    );
}
