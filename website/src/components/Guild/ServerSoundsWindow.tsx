import {
    Box,
    Button,
    Group,
    LoadingOverlay,
    Modal,
    Paper,
    ScrollArea,
    Text,
    Title,
} from "@mantine/core";
import { Dispatch, SetStateAction, useState } from "react";
import { useCookies } from "react-cookie";
import { COOKIE_NAMES } from "../../auth/AuthProvider";
import {
    ApiRequest,
    GuildFile,
    LOADINGOVERLAY_ZINDEX,
    MODAL_ZINDEX,
} from "../../utils/utils";
import DeleteModalBody from "../DeleteModalBody";
import { PlayControl } from "../PlayControl";
import SearchBar from "../SearchBar";

type ServerSoundsWindowProps = {
    guildId: string;
    guildFiles: GuildFile[];
    setGuildFiles: Dispatch<SetStateAction<GuildFile[]>>;
    classes: Record<
        | "serverSoundsPaper"
        | "scollAreaStyle"
        | "sideWindowsStyle"
        | "titleStyle",
        string
    >;
    adminMode: boolean;
    toggleAdminMode: () => void;
    isUpdating: boolean;
};

export default function ServerSoundsWindow({
    guildId,
    guildFiles,
    classes,
    adminMode,
    setGuildFiles,
    toggleAdminMode,
    isUpdating,
}: ServerSoundsWindowProps) {
    const [filterTerm, setFilterTerm] = useState("");
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [lastClickedFile, setLastClickedFile] = useState<
        GuildFile | undefined
    >(undefined);

    const [cookies] = useCookies(COOKIE_NAMES);

    const filterFiles = () => {
        if (filterTerm !== "") {
            return guildFiles.filter((file) => {
                if (file.sound_file.display_name) {
                    return (
                        file.sound_file.display_name
                            .toLowerCase()
                            .indexOf(filterTerm) !== -1
                    );
                } else {
                    return false;
                }
            });
        } else {
            return guildFiles;
        }
    };

    return (
        <>
            <Paper
                withBorder
                shadow="sm"
                p="sm"
                className={classes.serverSoundsPaper}
            >
                <LoadingOverlay
                    zIndex={LOADINGOVERLAY_ZINDEX}
                    visible={isUpdating}
                />
                <Group position="apart" direction="row">
                    <Box>
                        <Title
                            title="Server sounds"
                            order={3}
                            pb="xs"
                            className={classes.titleStyle}
                        >
                            Server sounds
                        </Title>
                    </Box>
                    <Button
                        onClick={() => {
                            toggleAdminMode();
                        }}
                    >
                        Toggle admin mode...
                    </Button>
                </Group>
                <Box py="sm">
                    <SearchBar
                        filterCallback={(searchValue) => {
                            setFilterTerm(searchValue);
                        }}
                    />
                </Box>
                <ScrollArea className={classes.scollAreaStyle}>
                    <Group>
                        {guildFiles.length > 0
                            ? !adminMode
                                ? filterFiles().map((file) => {
                                      return (
                                          <PlayControl
                                              key={file.file_id}
                                              file={file}
                                              guildId={guildId}
                                          />
                                      );
                                  })
                                : filterFiles().map((file) => {
                                      return (
                                          <Button
                                              onClick={() => {
                                                  setLastClickedFile(file);
                                                  setIsModalOpen(true);
                                              }}
                                          >
                                              {file.sound_file.display_name}
                                          </Button>
                                      );
                                  })
                            : isUpdating && (
                                  <Text size="xl" weight="bold">
                                      Server has no sounds.
                                  </Text>
                              )}
                    </Group>
                </ScrollArea>
            </Paper>
            {adminMode && lastClickedFile ? (
                <Modal
                    zIndex={MODAL_ZINDEX}
                    opened={isModalOpen}
                    withCloseButton={false}
                    closeOnClickOutside={false}
                    closeOnEscape={false}
                    centered
                    onClose={() => setIsModalOpen(false)}
                    title={lastClickedFile?.sound_file.display_name}
                    styles={{
                        title: {
                            maxWidth: "15ch",
                            textOverflow: "ellipsis",
                        },
                    }}
                >
                    <DeleteModalBody
                        file={lastClickedFile.sound_file}
                        closeCallback={() => setIsModalOpen(false)}
                        deleteCallback={() => {
                            return new Promise<void>((resolve, reject) => {
                                ApiRequest.removeFileFromGuild(
                                    guildId,
                                    lastClickedFile.file_id,
                                    cookies.access_token
                                )
                                    .then((_resp) => {
                                        resolve();
                                        setGuildFiles([
                                            ...guildFiles.filter((file) => {
                                                return (
                                                    file.file_id !==
                                                    lastClickedFile.file_id
                                                );
                                            }),
                                        ]);
                                    })
                                    .catch((e) => {
                                        reject(e);
                                    });
                            });
                        }}
                    />
                </Modal>
            ) : (
                <></>
            )}
        </>
    );
}
