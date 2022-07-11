# Env variables
Enviromental variables can be set in .env file

Required: 
- **DISCORD_CLIENT_ID**: Application client id (from Discord).
- **DISCORD_CLIENT_SECRET**: Applications secret **(DO NOT LEAK)** (from Discord).
- **DATABASE_URL**: PotsgreSQL database url.
- **SOUNDFILE_DIR**: Directory to which files are saved to (Database only cares for file ids, not their location).
- **WS_TOKEN**: Communication with bot application is done over websockets and this token is used to limit incoming websocket connections to only ones who hold this token.

Optional:
- **PORT**: Defaults to 8080.
- **MAX_FILE_SIZE**: Total maximum file size of all files in one upload request in bytes. Defaults
  to 10_000_000 bytes (10 MB).
- **TESTING_DATABASE_URL**: Database url for database to run tests on.

# Routes
Api available at route **`/v1`**.

> All ids sent in json format must be strings.
>
> Files is sometimes used to refer to sound files.

# Public routes

## Auth

### Initialize oauth authentication 
**GET** `/auth/init`
- Redirects to Discords oauth login.

### Invite bot to Discord Server
**GET** `/auth/botinvite`
- Redirects to Discords oauth login and bot invitation.

### OAuth callback
**GET** `/auth/callback`
- Returns Discords [access token response.](https://discord.com/developers/docs/topics/oauth2#authorization-code-grant-access-token-response)

### OAuth revoke
**POST** `/auth/revoke`
- Revokes token sent in form-data (token, optional token_type).

# Protected routes
For these routes user authentication is required.
Authentication is done through [auth routes](#auth).

## File

### File upload
**POST** `/files/upload`
- Uploaded files require to be audio files. 
- Returns a json array of uploaded files. Successfully uploaded files return with their valid data, failed files are marked with `uploaded: false` in the array.

**GET** `/files/public`
- Supports query params: `search_query, page, limit` (Upper limit is 200 files).
- Returns the first page of public files (first 200 files) if no query params are specified.

## Guild

### Add sound to guild
**POST** `/guilds/{guild_id}/{file_id}`
- Adds a sound to a chosen guild.
- Returns the sound file in json object.

### Delete sound from guild
**POST** `/guilds/{guild_id}/{file_id}`
- Deletes a sound from a chosen guild.
- Returns the sound file in json object.

### Get guild files
**GET** `/guilds/{guild_id}`
- Returns sounds available to chosen guild.

### Bulk enable sounds
**POST** `/guilds/bulkenable`
- Takes in a json object with a fields `guilds`, `files` which are both arrays.
Example:
```json
{
    "guilds": ["8456", "4789"],
    "files": ["1", "2", "438"]
}
```
- Tries to enable all provided sounds in all provided guilds.

## User

### List user files
**GET** `/user/files`
- Returns a list of files uploaded by user.

### Delete a single file
**DELETE** `/user/files/{file_id}`
- Returns deleted file.

### Delete multiple users files
**DELETE** `/user/files`
- Deletes user owned files specified in json payload.
- Json must contain a field `files` that is an array of file ids.

Example:
```json
{ "files": ["1", "2", "438"] }
```
- Returns json of all successfully deleted files.

### Get guilds
**GET** `/user/guilds`
- Returns a json of guilds shared by the user and bot.

### Toggle file visibility
**PATCH** `/user/togglevisibility/{file_id}`
- Toggles files visibility from public to private or vice versa.
- Returns the toggled sound file json object.

### Get user guilds
**GET** `/user/guilds`
- Returns Discord guilds user is a part of.

### Get guilds with files
**GET** `/user/guilds/{file_id}`
- Returns a json array of guilds that have the sound file enabled (doesn't return guilds user is not a part of).

### Get enabled user files
**GET** `/user/{guild_id}`
- Returns a json array of sound files that are enabled in guild.

## Controls
Routes for sending commands to Discord bot.

### Play
**POST** `/controls/play`
- Takes in a json payload with `file_id` and `guild_id` and optional `channel_id`.

### Stop
**POST** `/controls/stop`
- Takes in a json payload with `guild_id`.

### Skip
**POST** `/controls/skip`
- Takes in a json payload with `guild_id`.

### GetQueue
**POST** `/controls/queue`
- Takes in a json payload with `guild_id`.

# Websocket routes

## Protected websocket routes
Routes are protected with a token that should match on the backend and bots websocket client.

### Controls websocket
`/ws/controls`
- Used for bot application to communicate with backend.
- Receiving and responding to commands from controls routes.

### Sync websocket
`/ws/sync`
- Used for bot application to communicate with backend.
- Tries to sync bot being added/kicked/banned from various Discord servers.
- Updates guilds cache for users leaving guilds.

## Public websocket routes

### Channels websocket
`/ws/channels`
- Used to provide live preview of guilds voice channels.
- Requires authorization by sending an Identify payload in json format and responding to re-identify messages (failing to identify will terminate connection).

Example:
```json
{ "op": "Identify", "access_token": "stringified_access_token" }
```

- Can subscribe to a single guild at a time by sending a subscribe message. Server will not respond to if user is not identified or has tried to subscribe to a guild that they are not a part of.

Example:
```json
{ "op": "Subscribe", "guild_id": "00000000000" }
```

