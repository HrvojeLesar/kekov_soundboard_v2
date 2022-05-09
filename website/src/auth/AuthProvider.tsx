import axios from "axios";
import { createContext, useEffect, useMemo, useState } from "react";
import { ReactNode } from "react";
import { useCookies } from "react-cookie";

type User = {
    id: string,
    username: string,
    discriminator: string,
    avatar?: string,
    bot?: boolean,
    system?: boolean,
    mfa_enabled?: boolean,
    banner?: string,
    accent_color?: number,
    locale?: string,
    flags?: number,
}

type Tokens = {
    access_token: string,
    refresh_token: string,
    expires: number,
}

type AuthContextType = {
    user?: User;
    tokens?: Tokens;
    login: (tokens: Tokens) => void;
    logout: () => void;
    refresh: () => void;
}

export let AuthContext = createContext<AuthContextType>(null!);

function AuthProvider({ children }: { children: ReactNode }) {
    let [user, setUser] = useState<User | undefined>();
    let [tokens, setTokens] = useState<Tokens | undefined>();
    let [cookies] = useCookies(['access_token', 'refresh_token', 'expires']);

    const fetchUserInfo = async (tokens: Tokens) => {
        try {
            const { data } = await axios.get<User>('https://discord.com/api/v9/users/@me', {
                headers: {
                    Authorization: `Bearer ${tokens.access_token}`,
                }
            });
            console.log(data);
            setUser(data);
        } catch (e) {
            console.log(e);
        }
    }

    const login = async (tokens: Tokens) => {
        console.log(tokens);
        await fetchUserInfo(tokens);
    }

    const logout = async () => {
    }

    const refresh = async () => {
    }

    let value: AuthContextType = { user: user, tokens: tokens, login: login, logout: logout, refresh: refresh };

    if (!tokens && cookies.access_token && cookies.refresh_token && cookies.expires) {
        let tokens = {
            access_token: cookies.access_token,
            refresh_token: cookies.refresh_token,
            expires: cookies.expires,
        };
        setTokens(tokens);
        login(tokens);
    }

    return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}

export default AuthProvider;
