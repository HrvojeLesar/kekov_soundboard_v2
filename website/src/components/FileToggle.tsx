import { Avatar, Group, Text, Switch, Box } from "@mantine/core";
import { useContext, useState } from "react";
import { UserFile } from "../views/UserFiles";

type FileToggleProps = {
    file: UserFile;
    isActive: boolean;
    addCallback: (file: UserFile) => Promise<void>;
    removeCallback: (file: UserFile) => Promise<void>;
};

export function FileToggle({
    file,
    isActive,
    addCallback,
    removeCallback,
}: FileToggleProps) {
    const [checked, setChecked] = useState(isActive);

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
