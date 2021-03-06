import { createContext, useCallback, useEffect, useState } from "react";
import { ReactNode } from "react";
import { useCookies } from "react-cookie";
import { useNavigate } from "react-router-dom";
import { ApiRequest, cookieOptions, LoginResponse } from "../utils/utils";

enum TokenType {
    AccessToken = "access_token",
    RefreshToken = "refresh_token",
}

export type DiscordUser = {
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
    time_added: string;
    permissions?: string;
};

type AuthContextType = {
    user: DiscordUser | undefined;
    guilds: Guild[];
    login: (data: LoginResponse) => Promise<void>;
    logout: () => Promise<void>;
    refresh: () => Promise<void>;
    fetchGuilds: () => Promise<void>;
    isFetching: boolean;
    isFetchingGuilds: boolean;
};

export const COOKIE_NAMES = ["access_token", "refresh_token", "expires"];

export const AuthContext = createContext<AuthContextType>(null!);

function AuthProvider({ children }: { children: ReactNode }) {
    const navigate = useNavigate();
    const [user, setUser] = useState<DiscordUser | undefined>();
    const [guilds, setGuilds] = useState<Guild[]>([]);
    const [cookies, setCookie, removeCookie] = useCookies(COOKIE_NAMES);
    const [isFetching, setIsFetching] = useState(true);
    const [isFetchingGuilds, setIsFetchingGuilds] = useState(true);

    const fetchUserInfo = async (access_token: string) => {
        await ApiRequest.fetchDiscordUser(access_token)
            .then(({ data }) => {
                setUser(data);
            })
            .catch((e) => {
                console.log(e);
            });
    };

    const revokeAccess = useCallback(() => {
        const options = cookieOptions();
        removeCookie("access_token", options);
        removeCookie("refresh_token", options);
        removeCookie("expires", options);
        setUser(undefined);
    }, [removeCookie]);

    const login = async (data: LoginResponse) => {
        await fetchUserInfo(data.access_token);
    };

    const logout = async () => {
        if (cookies.access_token) {
            try {
                revokeAccess();
                navigate("/");
                let token: RevokeAccessToken = {
                    token: cookies.access_token,
                    token_type: TokenType.AccessToken,
                };
                await ApiRequest.revokeToken(token);
            } catch (e) {
                // TODO: Handle
                console.log(e);
            }
        }
    };

    const refreshAccess = useCallback((newData: LoginResponse) => {
        const options = cookieOptions(newData);
        setCookie("access_token", newData.access_token, options);
        setCookie("refresh_token", newData.refresh_token, options);
        setCookie("expires", Date.now() + newData.expires_in * 1000, options);
        return newData.access_token;
    }, [setCookie]);

    const refreshToken = useCallback(async () => {
        return ApiRequest.refreshToken(cookies.refresh_token)
            .then(({ data }) => {
                return refreshAccess(data);
            })
            .catch((e) => {
                console.log(e);
                revokeAccess();
                navigate("/login");
                return undefined;
            });
    }, [cookies.refresh_token, navigate, refreshAccess, revokeAccess]);

    const fetchGuilds = useCallback(async () => {
        try {
            if (cookies.access_token) {
                let { data } = await ApiRequest.fetchGuilds(
                    cookies.access_token
                );
                data.sort((a, b) => {
                    return Date.parse(a.time_added) - Date.parse(b.time_added);
                });
                setIsFetchingGuilds(false);
                setGuilds(data);
            }
        } catch (e) {
            // TODO: HANDLE
            console.log(e);
        }
    }, [cookies.access_token]);

    const handleRefresh = async () => {
        if (cookies.access_token && cookies.refresh_token) {
            await refreshToken();
        }
    };

    useEffect(() => {
        const handleLoad = async () => {
            if (
                cookies.access_token &&
                cookies.refresh_token &&
                cookies.expires
            ) {
                const now = Date.now() + 24 * 3600 * 1000;
                if (now > cookies.expires) {
                    var newAccessToken = await refreshToken();
                }
                await fetchUserInfo(newAccessToken ?? cookies.access_token);
            }
            setIsFetching(false);
        };
        handleLoad();
    }, [cookies.access_token, cookies.refresh_token, cookies.expires, refreshToken]);

    const value: AuthContextType = {
        user: user,
        login: login,
        logout: logout,
        refresh: handleRefresh,
        fetchGuilds: fetchGuilds,
        guilds: guilds,
        isFetching: isFetching,
        isFetchingGuilds: isFetchingGuilds,
    };

    return (
        <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
    );
}

export default AuthProvider;
