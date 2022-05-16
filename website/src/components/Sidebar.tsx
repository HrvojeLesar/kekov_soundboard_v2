import { Button, Center, Group, Navbar, Skeleton } from "@mantine/core";
import axios from "axios";
import { useContext, useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { API_URL, UserRoute } from "../api/ApiRoutes";
import { AuthContext } from "../auth/AuthProvider";

type Guild = {
    id: string,
    name: string,
    icon?: string,
    icon_hash?: string,
}

export default function Sidebar() {
    let { tokens } = useContext(AuthContext);
    const [guilds, setGuilds] = useState<Guild[]>([]);

    const spawnSkeletons = () => {
        let skeletons = [];
        for (let i = 0; i < 5; i++) {
            skeletons.push(<Skeleton key={i} height={50} circle mb="xl" />);
        }
        return skeletons;
    }

    const fetchGuilds = async () => {
        try {
            if (tokens) {
                let { data } = await axios.get<Guild[]>(`${API_URL}${UserRoute.getGuilds}`, {
                    headers: {
                        Authorization: `${tokens.access_token}`,
                    }
                });
                console.log(data);
                setGuilds(data);
            }
        } catch (e) {
            // TODO: HANDLE 
            console.log(e);
        }
    }

    const nameToInitials = (guildName: string): string => {
        let initials = "";
        guildName
            .split(' ')
            .forEach((word) => {
                if (word[0]) {
                    initials = initials.concat(word[0]);
                }
            });
        return initials;
    }

    const renderGuilds = () => {
        return guilds.map((guild) => {
            return (
                <Button component={Link} to={`/guilds/${guild.id}`} key={guild.id}>
                    {guild.icon_hash ?? nameToInitials(guild.name)}
                </Button>);
        });
    }

    useEffect(() => {
        fetchGuilds();
    }, [])

    return (
        <Navbar height="100vh" width={{ base: 80 }} p="md">
            <Center>
                <Group direction="column">
                    <Button component={Link} to={`/upload`} >
                        Upload
                    </Button>
                    <Button component={Link} to={`/user`} >
                        User files
                    </Button>
                </Group>
            </Center>
            <Navbar.Section grow mt={50}>
                <Group direction="column" align="center" spacing={2}>
                    {guilds.length > 0 ? renderGuilds() : spawnSkeletons()}
                </Group>
            </Navbar.Section>
        </Navbar>
    );
}
