# Env variables
Enviromental variables can be set in .env file

Required: 
- **DISCORD_CLIENT_ID**: Application client id obtained from Discord.
- **DISCORD_CLIENT_SECRET**: Applications secret **(DO NOT LEAK)**
- **DATABASE_URL**: PotsgreSQL database url

Optional:
- **PORT**: Defaults to 8080
- **REDIRECT_URL**: Defaults to localhost
- **MAX_FILE_SIZE**: Total maximum file size of all files in one upload request in bytes. Defaults
  to 10_000_000 bytes (10 MB)

# Routes
Api available at route **/v1**.

## Auth

### Initialize oauth authorization
**GET** `/auth/init`
- Redirects to Discords oauth login.

### OAuth callback
**GET** `/auth/callback`
- Returns Discords [access token response.](https://discord.com/developers/docs/topics/oauth2#authorization-code-grant-access-token-response)

### OAuth revoke
[TODO]: <> (Make sure this is accurate with actual implementation)
**GET** `/auth/revoke`
- Revokes token sent in form-data (token, optional token_type).

## File

### File upload
**POST** `/files/upload`
- Uploaded files require to be audio files. Returns successfully uploaded files json.

## User

### List user files
**GET** `/user/files`
- Returns a list of files uploaded by user.

### Delete multiple users files
**DELETE** `/user/files`
- Deletes user owned files specified in json payload.
Json must contain files field that is an array of file ids.
```json
{ "files": [1, 2, 438] }
```
- Returns json of all successfully deleted files.

### Delete a single file
**DELETE** `/user/files/{file_id}`
- Returns deleted file.

## Websocket

### Connecting to Websocket
**GET** `/`
Makes a connection to websocket server, localhost only.

## Controls

### Play
**POST** `/play`
Takes in a json payload with `file_id` and `guild_id`.

### Stop
**POST** `/stop`
Takes in a json payload with `guild_id`.
