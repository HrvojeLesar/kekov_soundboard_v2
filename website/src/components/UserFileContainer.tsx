import {
    Button,
    Text,
    Center,
    Group,
    Paper,
    Badge,
    Tooltip,
    ActionIcon,
    Box,
} from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { useEffect } from "react";
import { CirclePlus, Pencil, Trash } from "tabler-icons-react";
import { UserFile, UserFilesModalType } from "../views/UserFiles";

type UserFileContainerProps = {
    index?: number;
    file: UserFile;
    openModal: (modalType: UserFilesModalType, file: UserFile) => void;
};

export default function UserFileContainer({
    index,
    file,
    openModal,
}: UserFileContainerProps) {
    return (
        <tr>
            <td>
                <Group spacing="sm">
                    <Tooltip
                        wrapLines
                        position="top"
                        placement="start"
                        label={file.display_name}
                    >
                        <Badge>
                            <Text
                                style={{
                                    maxWidth: "150px",
                                    textOverflow: "ellipsis",
                                    whiteSpace: "nowrap",
                                    overflow: "hidden",
                                }}
                            >
                                {file.display_name}
                            </Text>
                        </Badge>
                    </Tooltip>
                </Group>
            </td>
            <td>
                <Group>
                    <Text align="left" weight={700} size="xl" color="red">
                        Placeholder text
                    </Text>
                </Group>
            </td>
            <td>
                <Group spacing={10} position="right">
                    <ActionIcon
                        variant="filled"
                        color="blue"
                        onClick={() => openModal(UserFilesModalType.Add, file)}
                    >
                        <CirclePlus />
                    </ActionIcon>
                    <ActionIcon
                        variant="filled"
                        color="blue"
                        onClick={() => openModal(UserFilesModalType.Edit, file)}
                    >
                        <Pencil />
                    </ActionIcon>
                    <ActionIcon
                        variant="filled"
                        color="red"
                        onClick={() => openModal(UserFilesModalType.Delete, file)}
                    >
                        <Trash />
                    </ActionIcon>
                </Group>
            </td>
        </tr>
    );
}
