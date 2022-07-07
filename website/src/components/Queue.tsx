import { Divider, Modal, Text } from "@mantine/core";
import { MODAL_ZINDEX, QueueReponse } from "../utils/utils";

type QueueProps = {
    isModalOpen: boolean;
    setIsModalOpen: React.Dispatch<React.SetStateAction<boolean>>;
    queueData: QueueReponse[];
};

export default function Queue({
    isModalOpen,
    setIsModalOpen,
    queueData,
}: QueueProps) {
    return (
        <Modal
            zIndex={MODAL_ZINDEX}
            opened={isModalOpen}
            closeOnEscape={false}
            overflow="inside"
            centered
            onClose={() => setIsModalOpen(false)}
            title="Queue"
            styles={{
                title: {
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    whiteSpace: "nowrap",
                },
            }}
        >
            {queueData.map((q, index) => {
                return index === 0 ? (
                    <>
                        <Text weight="bold" key={index}>
                            Currently playing:
                            <Text
                                weight={500}
                                component="span"
                                color="violet"
                            >{` ${q.display_name}`}</Text>
                        </Text>
                        {queueData.length > 1 ? <Divider my="xs" /> : <></>}
                    </>
                ) : (
                    <Text key={index}>{`${index}. ${q.display_name}`}</Text>
                );
            })}
        </Modal>
    );
}
