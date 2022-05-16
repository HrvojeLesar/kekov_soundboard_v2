import { Avatar, Group, Text, Switch, Box } from "@mantine/core";
import axios from "axios";
import { useContext, useState } from "react";
import { API_URL, GuildRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";
import { UserFile } from "../views/UserFiles";

type GuildToggleProps = {
    guildId: string;
    guildName: string;
    file: UserFile;
    hasFile: boolean;
};

export function GuildToggle({
    guildId,
    guildName,
    hasFile,
    file,
}: GuildToggleProps) {
    const [checked, setChecked] = useState(hasFile);
    const { tokens } = useContext(AuthContext);

    const toggleFile = async (state: boolean) => {
        try {
            if (state) {
                await addToGuild();
            } else {
                await removeFromGuild();
            }
            setChecked(state);
        } catch (e) {
            // TODO: Handle
            console.log(e);
        }
    };

    const addToGuild = async () => {
        return await axios.post(
            `${API_URL}${GuildRoute.postAddSound}${guildId}/${file.id}`,
            {},
            { headers: { authorization: `${tokens?.access_token}` } }
        );
    };

    const removeFromGuild = async () => {
        return await axios.delete(
            `${API_URL}${GuildRoute.postAddSound}${guildId}/${file.id}`,
            { headers: { authorization: `${tokens?.access_token}` } }
        );
    };

    return (
        <>
            <Group position="apart">
                <div>
                    <Group>
                        <Avatar src={null} />
                        <Text>{guildName}</Text>
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
