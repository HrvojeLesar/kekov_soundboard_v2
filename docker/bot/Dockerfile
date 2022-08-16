FROM mcr.microsoft.com/dotnet/sdk:6.0 as builder
WORKDIR /app
COPY discord_bot .
RUN dotnet build -c Release -o out

FROM mcr.microsoft.com/dotnet/aspnet:6.0
WORKDIR /app
COPY --from=builder /app/out .
COPY docker/bot/copy_dotenv_and_start.sh .
RUN chmod +x copy_dotenv_and_start.sh
CMD ["./copy_dotenv_and_start.sh"]
