import {
    ActionIcon,
    Box,
    Group,
    LoadingOverlay,
    Modal,
    Paper,
    ScrollArea,
    Text,
    Title,
    Tooltip,
    UnstyledButton,
} from "@mantine/core";
import { Dispatch, SetStateAction, useCallback, useState } from "react";
import { useCookies } from "react-cookie";
import { FaHatWizard } from "react-icons/fa";
import { MdVolumeUp } from "react-icons/md";
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
        | "titleStyle"
        | "button"
        | "unstyledButtonStyle"
        | "textStyle"
        | "iconStyle",
        string
    >;
    adminMode: boolean;
    toggleAdminMode: () => void;
    isUpdating: boolean;
    selectedChannelId: string | undefined;
};

export default function ServerSoundsWindow({
    guildId,
    guildFiles,
    classes,
    adminMode,
    setGuildFiles,
    toggleAdminMode,
    isUpdating,
    selectedChannelId,
}: ServerSoundsWindowProps) {
    const [filterTerm, setFilterTerm] = useState("");
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [lastClickedFile, setLastClickedFile] = useState<
        GuildFile | undefined
    >(undefined);

    const [cookies] = useCookies(COOKIE_NAMES);

    const handleSearch = useCallback(
        (searchValue: string) => {
            setFilterTerm(searchValue);
        },
        [setFilterTerm]
    );

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
                <Group position="apart" direction="row" noWrap>
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
                    <Tooltip
                        label="Toggle admin mode"
                        position={adminMode ? "left" : undefined}
                        withArrow
                    >
                        <ActionIcon
                            pb="xs"
                            onClick={() => {
                                toggleAdminMode();
                            }}
                        >
                            <FaHatWizard size={18} />
                        </ActionIcon>
                    </Tooltip>
                </Group>
                <Box py="sm">
                    <SearchBar onSearch={handleSearch} />
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
                                              selectedChannelId={
                                                  selectedChannelId
                                              }
                                          />
                                      );
                                  })
                                : filterFiles().map((file) => {
                                      return (
                                          <>
                                              <Paper
                                                  withBorder
                                                  shadow="xs"
                                                  className={classes.button}
                                              >
                                                  <UnstyledButton
                                                      p="sm"
                                                      className={
                                                          classes.unstyledButtonStyle
                                                      }
                                                      onClick={() => {
                                                          setLastClickedFile(
                                                              file
                                                          );
                                                          setIsModalOpen(true);
                                                      }}
                                                  >
                                                      <Group noWrap>
                                                          <MdVolumeUp
                                                              size={24}
                                                              className={
                                                                  classes.iconStyle
                                                              }
                                                          />
                                                          <Text
                                                              className={
                                                                  classes.textStyle
                                                              }
                                                              title={
                                                                  file
                                                                      .sound_file
                                                                      .display_name
                                                              }
                                                          >
                                                              {
                                                                  file
                                                                      .sound_file
                                                                      .display_name
                                                              }
                                                          </Text>
                                                      </Group>
                                                  </UnstyledButton>
                                              </Paper>
                                          </>
                                      );
                                  })
                            : guildFiles.length === 0 && (
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
                    closeOnEscape={false}
                    centered
                    onClose={() => setIsModalOpen(false)}
                    title={lastClickedFile?.sound_file.display_name}
                    styles={{
                        title: {
                            overflow: "hidden",
                            textOverflow: "ellipsis",
                            whiteSpace: "nowrap",
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
