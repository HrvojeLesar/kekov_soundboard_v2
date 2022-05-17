export const API_URL = "http://localhost:8080/v1";
export const DISCORD_CND_USER_AVATAR = (
    id?: string,
    icon?: string,
    discriminator?: string
) => {
    if (id && icon) {
        return `https://cdn.discordapp.com/avatars/${id}/${icon}`;
    }

    const discNum = Number(discriminator);
    if (!isNaN(discNum)) {
        return `https://cdn.discordapp.com/embed/avatars/${discNum % 5}.png`;
    }
    return `https://cdn.discordapp.com/embed/avatars/1.png`;
};

export enum DiscordRoutes {
    Me = "https://discord.com/api/v9/users/@me",
}

export enum AuthRoute {
    getInit = "/auth/init",
    getBotInvite = "/auth/botinvite",
    getCallback = "/auth/callback",
    postRevoke = "/auth/revoke",
}

export enum UserRoute {
    getFiles = "/user/files",
    getGuilds = "/user/guilds",
    getGuildsWithFile = "/user/guilds/",
    getEnabledFiles = "/user/",
    deleteFile = "/user/files/",
    deleteFiles = "/user/files",
}

export enum GuildRoute {
    getGuildSounds = "/guilds/",
    postAddSound = "/guilds/",
    deleteSound = "/guilds/",
}

export enum ControlsRoute {
    postPlay = "/controls/play",
    postStop = "/controls/stop",
    postSkip = "/controls/skip",
    postQueue = "/controls/queue",
}

export enum FilesRoute {
    postUpload = "/files/upload",
}
