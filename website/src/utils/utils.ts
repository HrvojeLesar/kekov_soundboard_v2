import { MantineTheme } from "@mantine/core";
import axios, { AxiosRequestConfig, AxiosResponse } from "axios";
import qs from "qs";
import { Dispatch, SetStateAction } from "react";
import { CookieSetOptions } from "universal-cookie";
import {
    API_URL,
    AuthRoute,
    ControlsRoute,
    DiscordRoutes,
    FilesRoute,
    GuildRoute,
    UserRoute,
} from "../api/ApiRoutes";
import { DiscordUser, Guild } from "../auth/AuthProvider";
import { EnabledUserFile } from "../components/Guild/QuickEnableWindow";

export const LOADINGOVERLAY_ZINDEX = 100;
export const MODAL_ZINDEX = 200;

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
    QueueFull = "QueueFull",
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
        case ClientErrorEnum.QueueFull:
            return "Queue is full!";
        case ClientErrorEnum.Unknown:
            return "Unknown error!";
    }
};

const axiosInstance = axios.create({
    baseURL: API_URL,
});

const authorizationHeaders = (accessToken: string): AxiosRequestConfig => {
    return {
        headers: { authorization: accessToken },
    };
};

enum TokenType {
    AccessToken = "access_token",
    RefreshToken = "refresh_token",
}

type RevokeAccessToken = {
    token: string;
    token_type: TokenType;
};

export type RefreshToken = {
    refresh_token: string;
};

export type LoginResponse = {
    access_token: string;
    expires_in: number;
    guild?: Guild;
    refresh_token: string;
    scope: string;
    token_type: string;
};

export type PlayPayload = {
    guild_id: string;
    file_id: string;
    channel_id?: string;
};

export type PlayResponse = {
    client_error?: ClientErrorEnum;
    op: PlayOpCodeEnum;
};

export type GuildsWithFile = {
    guild: Guild;
    has_file: boolean;
};

export type SoundFile = {
    id: string;
    display_name?: string;
    owner?: string;
    is_public: boolean;
    time_added: string;
};

export type GuildFile = {
    file_id: string;
    guild_id: string;
    time_added: string;
    sound_file: SoundFile;
};

export type QueueReponse = {
    id: string;
    display_name: string;
};

export type BulkEnablePayload = {
    guilds: string[];
    files: string[];
};

export const ApiRequest = {
    revokeToken: (
        token: RevokeAccessToken
    ): Promise<AxiosResponse<RevokeAccessToken>> => {
        return axiosInstance.post(AuthRoute.postRevoke, qs.stringify(token), {
            headers: {
                ContentType: "application/x-www-form-urlencoded",
            },
        });
    },
    refreshToken: (
        token: RefreshToken
    ): Promise<AxiosResponse<LoginResponse>> => {
        return axiosInstance.post(AuthRoute.postRefresh, {
            refresh_token: token,
        });
    },
    fetchDiscordUser: (
        accessToken: string
    ): Promise<AxiosResponse<DiscordUser>> => {
        return axiosInstance.get(DiscordRoutes.Me, {
            headers: {
                authorization: `Bearer ${accessToken}`,
            },
        });
    },
    fetchGuilds: (accessToken: string): Promise<AxiosResponse<Guild[]>> => {
        return axiosInstance.get(
            UserRoute.getGuilds,
            authorizationHeaders(accessToken)
        );
    },
    loginCallback: (
        code: string,
        state: string
    ): Promise<AxiosResponse<LoginResponse>> => {
        return axiosInstance.get(
            `${AuthRoute.getCallback}?code=${code}&state=${state}`
        );
    },
    controlsPlay: (
        playPayload: PlayPayload,
        accessToken: string
    ): Promise<AxiosResponse<PlayResponse>> => {
        return axiosInstance.post(
            ControlsRoute.postPlay,
            playPayload,
            authorizationHeaders(accessToken)
        );
    },
    controlsGetQueue: (
        guildId: string,
        accessToken: string
    ): Promise<AxiosResponse<QueueReponse[]>> => {
        return axiosInstance.post(
            ControlsRoute.postQueue,
            {
                guild_id: guildId,
            },
            authorizationHeaders(accessToken)
        );
    },
    controlsSkip: (
        guildId: string,
        accessToken: string
    ): Promise<AxiosResponse> => {
        return axiosInstance.post(
            ControlsRoute.postSkip,
            {
                guild_id: guildId,
            },
            authorizationHeaders(accessToken)
        );
    },
    controlsStop: (
        guildId: string,
        accessToken: string
    ): Promise<AxiosResponse> => {
        return axiosInstance.post(
            ControlsRoute.postStop,
            {
                guild_id: guildId,
            },
            authorizationHeaders(accessToken)
        );
    },
    addFileToGuild: (
        guildId: string,
        fileId: string,
        accessToken: string
    ): Promise<AxiosResponse<GuildFile>> => {
        return axiosInstance.post(
            `${GuildRoute.postAddSound}${guildId}/${fileId}`,
            {},
            authorizationHeaders(accessToken)
        );
    },
    removeFileFromGuild: (
        guildId: string,
        fileId: string,
        accessToken: string
    ): Promise<AxiosResponse<GuildFile>> => {
        return axiosInstance.delete(
            `${GuildRoute.deleteSound}${guildId}/${fileId}`,
            authorizationHeaders(accessToken)
        );
    },
    fetchEnabledUserFiles: (
        guildId: string,
        abortController: AbortController | undefined,
        accessToken: string
    ): Promise<AxiosResponse<EnabledUserFile[]>> => {
        return axiosInstance.get(`${UserRoute.getEnabledFiles}${guildId}`, {
            ...authorizationHeaders(accessToken),
            signal: abortController?.signal,
        });
    },
    fetchGuildsWithFile: (
        fileId: string,
        abortController: AbortController | undefined,
        accessToken: string
    ): Promise<AxiosResponse<GuildsWithFile[]>> => {
        return axiosInstance.get(`${UserRoute.getGuildsWithFile}${fileId}`, {
            ...authorizationHeaders(accessToken),
            signal: abortController?.signal,
        });
    },
    fetchGuildFiles: (
        guildId: string,
        abortController: AbortController | undefined,
        accessToken: string
    ): Promise<AxiosResponse<GuildFile[]>> => {
        return axiosInstance.get(`${GuildRoute.getGuildSounds}${guildId}`, {
            ...authorizationHeaders(accessToken),
            signal: abortController?.signal,
        });
    },
    bulkEnable: (
        bulk: BulkEnablePayload,
        accessToken: string
    ): Promise<AxiosResponse<GuildFile[]>> => {
        return axiosInstance.post(
            GuildRoute.postBulkenable,
            bulk,
            authorizationHeaders(accessToken)
        );
    },
    upload: (
        formData: FormData,
        accessToken: string,
        setProgressValue: Dispatch<SetStateAction<number>>
    ): Promise<AxiosResponse<SoundFile[]>> => {
        return axiosInstance.post(FilesRoute.postUpload, formData, {
            headers: {
                "Content-Type": "multipart/form-data",
            },
            ...authorizationHeaders(accessToken),
            onUploadProgress: (progress) => {
                const uploadPercent = Math.round(
                    (progress.loaded / progress.total) * 100
                );
                setProgressValue(uploadPercent);
            },
        });
    },
    deleteUserFile: (
        fileId: string,
        accessToken: string
    ): Promise<AxiosResponse<SoundFile>> => {
        return axiosInstance.delete(
            `${UserRoute.deleteFile}${fileId}`,
            authorizationHeaders(accessToken)
        );
    },
    getUserFiles: (
        accessToken: string
    ): Promise<AxiosResponse<SoundFile[]>> => {
        return axiosInstance.get(
            UserRoute.getFiles,
            authorizationHeaders(accessToken)
        );
    },
    toggleFileVisibility: (
        fileId: string,
        accessToken: string,
        abortController: AbortController | undefined
    ): Promise<AxiosResponse<SoundFile>> => {
        return axiosInstance.patch(
            `${UserRoute.toggleFileVisability}${fileId}`,
            {},
            {
                ...authorizationHeaders(accessToken),
                signal: abortController?.signal,
            }
        );
    },
    getPublicFiles: (
        accessToken: string
    ): Promise<AxiosResponse<SoundFile[]>> => {
        return axiosInstance.get(
            FilesRoute.getPublic,
            authorizationHeaders(accessToken)
        );
    },
};

export const primaryShade = (theme: MantineTheme): number => {
    return typeof theme.primaryShade === "number"
        ? theme.primaryShade
        : theme.colorScheme === "dark"
        ? theme.primaryShade.dark
        : theme.primaryShade.light;
};
