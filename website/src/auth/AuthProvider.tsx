import axios from "axios";
import { createContext, useEffect, useState } from "react";
import { ReactNode } from "react";
import { useCookies } from "react-cookie";
import qs from "qs";
import { API_URL, AuthRoute, DiscordRoutes, UserRoute } from "../api/ApiRoutes";
import { useNavigate } from "react-router-dom";
import { cookieOptions } from "../utils/utils";

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
    user: User | undefined;
    guilds: Guild[];
    login: (data: LoginResponse) => Promise<void>;
    logout: () => Promise<void>;
    refresh: () => Promise<void>;
    fetchGuilds: () => Promise<void>;
    isFetching: boolean;
};

export type LoginResponse = {
    access_token: string;
    expires_in: number;
    guild?: Guild;
    refresh_token: string;
    scope: string;
    token_type: string;
};

export const COOKIE_NAMES = ["access_token", "refresh_token", "expires"];

export const AuthContext = createContext<AuthContextType>(null!);

function AuthProvider({ children }: { children: ReactNode }) {
    const navigate = useNavigate();
    const [user, setUser] = useState<User | undefined>();
    const [guilds, setGuilds] = useState<Guild[]>([]);
    const [cookies, setCookie, removeCookie] = useCookies(COOKIE_NAMES);
    const [isFetching, setIsFetching] = useState(true);

    const fetchUserInfo = async (access_token: string) => {
        await axios
            .get<User>(DiscordRoutes.Me, {
                headers: {
                    Authorization: `Bearer ${access_token}`,
                },
            })
            .then(({ data }) => {
                setUser(data);
            })
            .catch((e) => {
                console.log(e);
            });
    };

    const revokeAccess = () => {
        removeCookie("access_token");
        removeCookie("refresh_token");
        removeCookie("expires");
        setUser(undefined);
    };

    const login = async (data: LoginResponse) => {
        await fetchUserInfo(data.access_token);
    };

    const logout = async () => {
        if (cookies.access_token) {
            try {
                let token: RevokeAccessToken = {
                    token: cookies.access_token,
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
                navigate("/");
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    const refreshAccess = (newData: LoginResponse) => {
        const options = cookieOptions(newData);
        setCookie("access_token", newData.access_token, options);
        setCookie("refresh_token", newData.refresh_token, options);
        setCookie("expires", Date.now() + newData.expires_in * 1000, options);
        return newData.access_token;
    };

    const refreshToken = async () => {
        return await axios
            .post<LoginResponse>(`${API_URL}${AuthRoute.postRefresh}`, {
                refresh_token: `${cookies.refresh_token}`,
            })
            .then(({ data }) => {
                return refreshAccess(data);
            })
            .catch((e) => {
                console.log(e);
                revokeAccess();
                navigate("/login");
                return undefined;
            });
    };

    const fetchGuilds = async () => {
        try {
            if (cookies.access_token) {
                let { data } = await axios.get<Guild[]>(
                    `${API_URL}${UserRoute.getGuilds}`,
                    {
                        headers: {
                            Authorization: `${cookies.access_token}`,
                        },
                    }
                );
                setGuilds(data);
            }
        } catch (e) {
            // TODO: HANDLE
            console.log(e);
        }
    };

    const handleRefresh = async () => {
        if (cookies.access_token && cookies.refresh_token) {
            await refreshToken();
        }
    };

    const handleLoad = async () => {
        if (cookies.access_token && cookies.refresh_token && cookies.expires) {
            const now = Date.now() + 24 * 3600 * 1000;
            if (now > cookies.expires) {
                var newAccessToken = await refreshToken();
            }
            await fetchUserInfo(newAccessToken ?? cookies.access_token);
        }
        setIsFetching(false);
    };

    useEffect(() => {
        handleLoad();
    }, []);

    const value: AuthContextType = {
        user: user,
        login: login,
        logout: logout,
        refresh: handleRefresh,
        fetchGuilds: fetchGuilds,
        guilds: guilds,
        isFetching: isFetching,
    };

    return (
        <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
    );
}

export default AuthProvider;
