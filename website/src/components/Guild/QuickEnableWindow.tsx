import {
    Box,
    createStyles,
    LoadingOverlay,
    Paper,
    ScrollArea,
    Text,
    Title,
} from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import { CanceledError } from "axios";
import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { TbX } from "react-icons/tb";
import { COOKIE_NAMES } from "../../auth/AuthProvider";
import { ApiRequest, LOADINGOVERLAY_ZINDEX, UserFile } from "../../utils/utils";
import SearchBar from "../SearchBar";
import QuickEnableCheckbox from "./QuickEnableCheckbox";

const useStyle = createStyles((_theme) => {
    return {
        quickEnablePaper: {
            width: "100%",
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
            flexGrow: 1,
            position: "relative",
        },
    };
});

export type EnabledUserFile = {
    sound_file: UserFile;
    enabled: boolean;
};

type QuickEnableWindowProps = {
    guildId: string;
    enableCallback: (file: EnabledUserFile) => void;
};

let abortController: AbortController | undefined = undefined;

// TODO: add loading display for slow connections
export default function QuickEnableWindow({
    guildId,
    enableCallback,
}: QuickEnableWindowProps) {
    const [cookies] = useCookies(COOKIE_NAMES);
    const [userFiles, setUserFiles] = useState<EnabledUserFile[]>([]);
    const [isFetchingFiles, setIsFetchingFiles] = useState(true);
    const [filterTerm, setFilterTerm] = useState("");

    const { classes } = useStyle();

    const fetchUserFiles = async () => {
        if (cookies.access_token) {
            try {
                abortController = new AbortController();
                const { data } = await ApiRequest.fetchEnabledUserFiles(
                    guildId,
                    abortController,
                    cookies.access_token
                );
                setUserFiles(data);
                setIsFetchingFiles(false);
            } catch (e) {
                // TODO: Handle
                console.log(e);
                if (e instanceof CanceledError) {
                    return;
                } else {
                    setIsFetchingFiles(false);
                }
            }
        }
    };

    const handleToggle = async (state: boolean, file: EnabledUserFile) => {
        const foundFile = userFiles.find((f) => {
            return f.sound_file.id === file.sound_file.id;
        });
        if (foundFile) {
            try {
                if (state) {
                    await addToGuild(foundFile);
                } else {
                    await removeFromGuild(foundFile);
                }
                foundFile.enabled = state;
                setUserFiles([...userFiles]);
                enableCallback(foundFile);
            } catch (e) {
                console.log(e);
            }
        } else {
            showNotification({
                title: "Error",
                message:
                    "An error occured while trying to toggle. Try refreshing!",
                autoClose: false,
                color: "red",
                icon: <TbX size={24} />,
            });
        }
    };

    const addToGuild = async (file: EnabledUserFile) => {
        await ApiRequest.addFileToGuild(
            guildId,
            file.sound_file.id,
            cookies.access_token
        );
    };

    const removeFromGuild = async (file: EnabledUserFile) => {
        await ApiRequest.removeFileFromGuild(
            guildId,
            file.sound_file.id,
            cookies.access_token
        );
    };

    const filterFiles = () => {
        if (filterTerm !== "") {
            return userFiles.filter((file) => {
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
            return userFiles;
        }
    };

    useEffect(() => {
        abortController?.abort();
        setIsFetchingFiles(true);
        fetchUserFiles();
    }, [guildId]);

    return (
        <Paper
            withBorder
            shadow="sm"
            p="sm"
            className={classes.quickEnablePaper}
        >
            <LoadingOverlay
                zIndex={LOADINGOVERLAY_ZINDEX}
                visible={isFetchingFiles}
            />
            <Title title="Quick enable files" order={3} pb="xs">
                Quick enable files
            </Title>
            <Box py="sm">
                <SearchBar
                    filterCallback={(searchValue) => {
                        setFilterTerm(searchValue);
                    }}
                />
            </Box>
            <ScrollArea>
                {userFiles.length > 0
                    ? filterFiles().map((file) => {
                          return (
                              <Box my="sm" key={file.sound_file.id}>
                                  <QuickEnableCheckbox
                                      file={file}
                                      onChange={handleToggle}
                                  />
                              </Box>
                          );
                      })
                    : !isFetchingFiles && (
                          <Text size="xl" weight="bold">
                              You have no files to quick enable.
                          </Text>
                      )}
            </ScrollArea>
        </Paper>
    );
}
