import {
    Box,
    createStyles,
    Group,
    Paper,
    ScrollArea,
    Title,
} from "@mantine/core";
import { showNotification } from "@mantine/notifications";
import axios, { CanceledError } from "axios";
import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { X } from "tabler-icons-react";
import { API_URL, GuildRoute, UserRoute } from "../../api/ApiRoutes";
import { COOKIE_NAMES } from "../../auth/AuthProvider";
import { UserFile } from "../../views/UserFiles";
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
                const { data } = await axios.get<EnabledUserFile[]>(
                    `${API_URL}${UserRoute.getEnabledFiles}${guildId}`,
                    {
                        headers: { authorization: `${cookies.access_token}` },
                        signal: abortController?.signal,
                    }
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
                icon: <X />,
            });
        }
    };

    const addToGuild = async (file: EnabledUserFile) => {
        await axios.post(
            `${API_URL}${GuildRoute.postAddSound}${guildId}/${file.sound_file.id}`,
            {},
            { headers: { authorization: `${cookies.access_token}` } }
        );
    };

    const removeFromGuild = async (file: EnabledUserFile) => {
        await axios.delete(
            `${API_URL}${GuildRoute.postAddSound}${guildId}/${file.sound_file.id}`,
            { headers: { authorization: `${cookies.access_token}` } }
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
            {isFetchingFiles ? (
                <div>Loading...</div>
            ) : (
                <ScrollArea>
                    {filterFiles().map((file) => {
                        return (
                            <Box m="sm" key={file.sound_file.id}>
                                <QuickEnableCheckbox
                                    file={file}
                                    onChange={handleToggle}
                                />
                            </Box>
                        );
                    })}
                </ScrollArea>
            )}
        </Paper>
    );
}
