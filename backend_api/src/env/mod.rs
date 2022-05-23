pub fn check_required_env_variables() {
    dotenv::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID env variable missing!");
    dotenv::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET env variable missing!");
    dotenv::var("DATABASE_URL").expect("DATABASE_URL env variable missing!");
    #[cfg(target_os = "windows")]
    dotenv::var("SOUNDFILE_DIR").expect("SOUNDFILE_DIR env variable missing!");
    #[cfg(target_os = "linux")]
    dotenv::var("SOUNDFILE_DIR").expect("SOUNDFILE_DIR env variable missing!");
}
