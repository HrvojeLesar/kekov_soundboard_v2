import { Avatar, Group, Text, Switch } from "@mantine/core";
import { useEffect, useState } from "react";
import { SoundFile } from "../utils/utils";

type FileToggleProps = {
    file: SoundFile;
    isActive: boolean;
    addCallback: (file: SoundFile) => Promise<void>;
    removeCallback: (file: SoundFile) => Promise<void>;
};

export function FileToggle({
    file,
    isActive,
    addCallback,
    removeCallback,
}: FileToggleProps) {
    const [checked, setChecked] = useState(false);

    const toggleFile = async (state: boolean) => {
        try {
            if (state) {
                await addCallback(file);
            } else {
                await removeCallback(file);
            }
            setChecked(state);
        } catch (e) {
            // TODO: Handle
            console.log(e);
        }
    };

    useEffect(() => {
        setChecked(isActive);
    }, []);

    return (
        <>
            <Group position="apart">
                <div>
                    <Group>
                        <Avatar src={null} />
                        <Text>{file.display_name}</Text>
                    </Group>
                </div>
                <Switch
                    checked={checked}
                    size="lg"
                    onLabel="ON"
                    offLabel="OFF"
                    onChange={(e) => {
                        toggleFile(e.target.checked);
                    }}
                />
            </Group>
        </>
    );
}
