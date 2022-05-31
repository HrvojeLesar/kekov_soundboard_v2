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

export const cookieOptions = (
    data: LoginResponse | undefined = undefined
): CookieSetOptions => {
    return { path: "/", maxAge: data?.expires_in };
};

export enum ClientErrorEnum {
    InvalidGuildId = "InvalidGuildId",
    GuildNotFound = "GuildNotFound",
    ChannelNotFound = "ChannelNotFound",
    ChannelsEmpty = "ChannelsEmpty",
    LavalinkConnectionNotEstablished = "LavalinkConnectionNotEstablished",
    InvalidVoiceChannel = "InvalidVoiceChannel",
    FileLoadingFailed = "FileLoadingFailed",
    InvalidFileId = "InvalidFileId",
    NotPlaying = "NotPlaying",
    Unknown = "Unknown",
}

export enum PlayOpCodeEnum {
    Error = "Error",
    PlayResponse = "PlayResponse",
    PlayResponseQueued = "PlayResponseQueued",
}

export const convertClientErrorToString = (error: ClientErrorEnum) => {
    switch (error) {
        case ClientErrorEnum.InvalidGuildId:
            return "Provided guild id is invalid!";
        case ClientErrorEnum.GuildNotFound:
            return "Guild not found!";
        case ClientErrorEnum.ChannelNotFound:
            return "Channel not found!";
        case ClientErrorEnum.ChannelsEmpty:
            return "All channels are empty!";
        case ClientErrorEnum.LavalinkConnectionNotEstablished:
            return "Lavalink connection could not be established";
        case ClientErrorEnum.InvalidVoiceChannel:
            return "Provided invalid voice channel!";
        case ClientErrorEnum.FileLoadingFailed:
            return "Failed to load a file!";
        case ClientErrorEnum.InvalidFileId:
            return "Invalid file id!";
        case ClientErrorEnum.NotPlaying:
            return "Nothing is playing!";
        case ClientErrorEnum.Unknown:
            return "Unknown error!";
    }
};
