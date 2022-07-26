import { ActionIcon, createStyles, Modal } from "@mantine/core";
import { useState } from "react";
import { TbTrash } from "react-icons/tb";
import { MODAL_ZINDEX, SoundFile } from "../../utils/utils";
import DeleteModalBody from "../DeleteModalBody";

type DeleteFileProps = {
    file: SoundFile;
    deleteCallback: () => Promise<void>;
};

const useStyle = createStyles((theme) => {
    return {
        actionIconColor: {
            color: theme.colors.red[5],
        },
    };
});

export default function DeleteFile({ file, deleteCallback }: DeleteFileProps) {
    const [isModalOpen, setIsModalOpen] = useState(false);
    const { classes } = useStyle();

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
                        whiteSpace: "nowrap",
                        textOverflow: "ellipsis",
                        overflow: "hidden",
                    },
                }}
            >
                <DeleteModalBody
                    file={file}
                    closeCallback={() => setIsModalOpen(false)}
                    deleteCallback={deleteCallback}
                />
            </Modal>
            <ActionIcon
                onClick={() => setIsModalOpen(true)}
                className={classes.actionIconColor}
            >
                <TbTrash size={24} />
            </ActionIcon>
        </>
    );
}
