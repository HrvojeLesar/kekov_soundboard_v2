import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { COOKIE_NAMES } from "../auth/AuthProvider";
import { ApiRequest, UserFile } from "../utils/utils";
import { FileToggle } from "./FileToggle";

type GuildAddFileModalBodyPropsType = {
    addFileCallback: (file: UserFile) => void;
    removeFileCallback: (file: UserFile) => void;
    guildId: string;
};

type EnabledFile = {
    sound_file: UserFile;
    enabled: boolean;
};

export default function GuildAddFileModalBody({
    guildId,
    addFileCallback,
    removeFileCallback,
}: GuildAddFileModalBodyPropsType) {
    const [cookies] = useCookies(COOKIE_NAMES);
    const [files, setFiles] = useState<EnabledFile[]>([]);

    const fetchFiles = async () => {
        if (cookies.access_token) {
            try {
                const { data } = await ApiRequest.fetchEnabledUserFiles(
                    guildId,
                    undefined,
                    cookies.access_token
                );
                console.log(data);
                setFiles(data);
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    const addToGuild = async (file: UserFile) => {
        await ApiRequest.addFileToGuild(guildId, file.id, cookies.access_token);
        addFileCallback(file);
        return;
    };

    const removeFromGuild = async (file: UserFile) => {
        await ApiRequest.removeFileFromGuild(
            guildId,
            file.id,
            cookies.access_token
        );
        removeFileCallback(file);
        return;
    };

    useEffect(() => {
        fetchFiles();
    }, []);

    // const Row = ({ index, style }: { index: number, style: any }) => {
    //     return (
    //         <Box style={style}>
    //             <PlayControl
    //                 file={guildFiles[index]}
    //                 playFunc={playFunc}
    //                 key={guildFiles[index].id}
    //             />
    //         </Box>
    //     );
    // };
    //                 <FixedSizeList
    //                     height={height - 35}
    //                     width="100%"
    //                     itemCount={guildFiles.length}
    //                     itemSize={80}
    //                 >
    //                     {Row}
    //                 </FixedSizeList>

    // const { height } = useViewportSize();

    return (
        <>
            {files.map((file) => {
                return (
                    <FileToggle
                        key={file.sound_file.id}
                        file={file.sound_file}
                        isActive={file.enabled}
                        addCallback={(file) => addToGuild(file)}
                        removeCallback={(file) => removeFromGuild(file)}
                    />
                );
            })}
        </>
    );
}
