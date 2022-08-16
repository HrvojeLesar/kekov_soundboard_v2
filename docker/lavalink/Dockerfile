FROM azul/zulu-openjdk:13

RUN apt-get update && apt-get install wget -y
RUN groupadd -g 322 lavalink
RUN useradd -r -u 322 -g lavalink lavalink
USER lavalink

WORKDIR /app
COPY docker/lavalink/application.yml .

RUN wget https://github.com/freyacodes/Lavalink/releases/download/3.4/Lavalink.jar
CMD ["java", "-Djdk.tls.client.protocols=TLSv1.1,TLSv1.2", "-Xmx4G", "-jar", "Lavalink.jar"]
