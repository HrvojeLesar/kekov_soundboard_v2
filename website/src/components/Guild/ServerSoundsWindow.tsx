import {
    Box,
    Button,
    Group,
    Modal,
    Paper,
    ScrollArea,
    Title,
} from "@mantine/core";
import { Dispatch, SetStateAction, useState } from "react";
import { useCookies } from "react-cookie";
import { COOKIE_NAMES } from "../../auth/AuthProvider";
import { ApiRequest, GuildFile } from "../../utils/utils";
import DeleteModalBody from "../DeleteModalBody";
import { PlayControl } from "../PlayControl";
import SearchBar from "../SearchBar";

type ServerSoundsWindowProps = {
    guildId: string;
    guildFiles: GuildFile[];
    setGuildFiles: Dispatch<SetStateAction<GuildFile[]>>;
    classes: Record<
        "serverSoundsPaper" | "scollAreaStyle" | "sideWindowsStyle",
        string
    >;
    adminMode: boolean;
};

export default function ServerSoundsWindow({
    guildId,
    guildFiles,
    classes,
    adminMode,
    setGuildFiles,
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
                if (file.display_name) {
                    return (
                        file.display_name.toLowerCase().indexOf(filterTerm) !==
                        -1
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
                <Title title="Server sounds" order={3} pb="xs">
                    Server sounds
                </Title>
                <Box py="sm">
                    <SearchBar
                        filterCallback={(searchValue) => {
                            setFilterTerm(searchValue);
                        }}
                    />
                </Box>
                <ScrollArea className={classes.scollAreaStyle}>
                    <Group>
                        {!adminMode
                            ? filterFiles().map((file) => {
                                  return (
                                      <PlayControl
                                          key={file.id}
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
                                          {file.display_name}
                                      </Button>
                                  );
                              })}
                    </Group>
                </ScrollArea>
            </Paper>
            {adminMode && lastClickedFile ? (
                <Modal
                    opened={isModalOpen}
                    withCloseButton={false}
                    closeOnClickOutside={false}
                    closeOnEscape={false}
                    centered
                    onClose={() => setIsModalOpen(false)}
                    title={lastClickedFile?.display_name}
                    styles={{
                        title: {
                            maxWidth: "15ch",
                            textOverflow: "ellipsis",
                        },
                    }}
                >
                    <DeleteModalBody
                        file={lastClickedFile}
                        closeCallback={() => setIsModalOpen(false)}
                        deleteCallback={() => {
                            return new Promise<void>((resolve, reject) => {
                                ApiRequest.removeFileFromGuild(
                                    guildId,
                                    lastClickedFile.id,
                                    cookies.access_token
                                )
                                    .then((_resp) => {
                                        resolve();
                                        setGuildFiles([
                                            ...guildFiles.filter((file) => {
                                                return (
                                                    file.id !==
                                                    lastClickedFile.id
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
