pub fn check_required_env_variables() {
    dotenv::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID env variable missing!");
    dotenv::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET env variable missing!");
    dotenv::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN env variable missing!");
    dotenv::var("DATABASE_URL").expect("DATABASE_URL env variable missing!");
}
