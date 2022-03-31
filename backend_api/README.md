# Env variables
Enviromental variables can be set in .env file

Required: 
* **DISCORD_CLIENT_ID**: Application client id obtained from Discord.
* **DISCORD_CLIENT_SECRET**: Applications secret **(DO NOT LEAK)**

Optional:
* **PORT**: Defaults to 8080
* **REDIRECT_URL**: Defaults to localhost

# Routes
Api available at route **/v1**.

## Auth

### Initialize oauth authorization
**GET** /auth/init
Redirects to Discords oauth login.

### OAuth callback
**GET** /auth/callback
Returns Discords [access token response.](https://discord.com/developers/docs/topics/oauth2#authorization-code-grant-access-token-response)

### OAuth revoke
[TODO]: <> (Make sure this is accurate with actual implementation)
**GET** /auth/revoke
Revokes token sent in form-data (token, optional token_type).

## File

### File upload
**POST** /files/upload
Uploaded files require to be audio files. Returns successfully uploaded files json.
