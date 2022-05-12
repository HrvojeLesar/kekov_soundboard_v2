import { Button } from "@mantine/core";
import { GuildFile } from "./Guild";

export function PlayControl({ file, playFunc }: { file: GuildFile, playFunc: any }) {
    return (
        <Button key={file.id} onClick={() => playFunc(file.id)}>
            {file.display_name ?? "Golden Legendary"}
        </Button>
    );
}
