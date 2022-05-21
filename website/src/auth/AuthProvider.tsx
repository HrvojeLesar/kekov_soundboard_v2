import axios from "axios";
import { createContext, useState } from "react";
import { ReactNode } from "react";
import { useCookies } from "react-cookie";
import qs from "qs";
import { API_URL, AuthRoute, DiscordRoutes, UserRoute } from "../api/ApiRoutes";

enum TokenType {
    AccessToken = "access_token",
    RefreshToken = "refresh_token",
}

type User = {
    id: string;
    username: string;
    discriminator: string;
    avatar?: string;
    bot?: boolean;
    system?: boolean;
    mfa_enabled?: boolean;
    banner?: string;
    accent_color?: number;
    locale?: string;
    flags?: number;
};

type Tokens = {
    access_token: string;
    refresh_token: string;
    expires: number;
};

type RevokeAccessToken = {
    token: string;
    token_type: TokenType;
};

export type Guild = {
    id: string;
    name: string;
    icon?: string;
    icon_hash?: string;
};

type AuthContextType = {
    user?: User;
    tokens?: Tokens;
    guilds: Guild[];
    login: (tokens: Tokens) => void;
    logout: () => void;
    refresh: () => void;
    fetchGuilds: () => void;
};

export const AuthContext = createContext<AuthContextType>(null!);

function AuthProvider({ children }: { children: ReactNode }) {
    const [user, setUser] = useState<User | undefined>();
    const [tokens, setTokens] = useState<Tokens | undefined>();
    const [guilds, setGuilds] = useState<Guild[]>([]);
    const [cookies, _setCookie, removeCookie] = useCookies([
        "access_token",
        "refresh_token",
        "expires",
    ]);

    const fetchUserInfo = async (tokens: Tokens) => {
        try {
            const { data } = await axios.get<User>(DiscordRoutes.Me, {
                headers: {
                    Authorization: `Bearer ${tokens.access_token}`,
                },
            });
            console.log(data);
            setUser(data);
        } catch (e) {
            // TODO: Handle
            console.log(e);
        }
    };

    const revokeAccess = () => {
        removeCookie("access_token");
        removeCookie("refresh_token");
        removeCookie("expires");
        setUser(undefined);
        setTokens(undefined);
    };

    const login = async (tokens: Tokens) => {
        await fetchUserInfo(tokens);
    };

    const logout = async () => {
        if (tokens?.access_token) {
            try {
                let token: RevokeAccessToken = {
                    token: tokens.access_token,
                    token_type: TokenType.AccessToken,
                };
                await axios.post<RevokeAccessToken>(
                    `${API_URL}${AuthRoute.postRevoke}`,
                    qs.stringify(token),
                    {
                        headers: {
                            ContentType: "application/x-www-form-urlencoded",
                        },
                    }
                );
                revokeAccess();
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    // TODO: add
    const refresh = async () => { };

    const fetchGuilds = async () => {
        try {
            if (tokens) {
                let { data } = await axios.get<Guild[]>(
                    `${API_URL}${UserRoute.getGuilds}`,
                    {
                        headers: {
                            Authorization: `${tokens.access_token}`,
                        },
                    }
                );
                console.log("got guilds", data);
                setGuilds(data);
            }
        } catch (e) {
            // TODO: HANDLE
            console.log(e);
        }
    };

    let value: AuthContextType = {
        user: user,
        tokens: tokens,
        login: login,
        logout: logout,
        refresh: refresh,
        fetchGuilds: fetchGuilds,
        guilds: guilds,
    };

    if (
        !tokens &&
        cookies.access_token &&
        cookies.refresh_token &&
        cookies.expires
    ) {
        let tokens = {
            access_token: cookies.access_token,
            refresh_token: cookies.refresh_token,
            expires: cookies.expires,
        };
        setTokens(tokens);
        login(tokens);
    }

    return (
        <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
    );
}

export default AuthProvider;
