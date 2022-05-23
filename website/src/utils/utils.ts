import { CookieSetOptions } from "universal-cookie";
import { LoginResponse } from "../auth/AuthProvider";

export const nameToInitials = (guildName: string): string => {
    let initials = "";
    guildName.split(" ").forEach((word) => {
        if (word[0]) {
            initials = initials.concat(word[0]);
        }
    });
    return initials;
};

export const cookieOptions = (data: LoginResponse): CookieSetOptions => {
    return { path: "/", maxAge: data.expires_in };
};
