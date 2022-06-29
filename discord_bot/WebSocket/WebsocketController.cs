using System.Net.WebSockets;
using dotenv.net;
using Websocket.Client;

namespace KekovBot.WebSocket
{
    public class WebsocketController
    {
        protected WebsocketClient _client;
        public WebsocketClient Client { get { return _client; } }

        protected WebsocketController(String uri)
        {
            var factory = new Func<ClientWebSocket>(() =>
            {
                var env = DotEnv.Read();
                var client = new ClientWebSocket();
                client.Options.SetRequestHeader("X-Ws-Token", env["WS_TOKEN"]);
                return client;
            });
            _client = new WebsocketClient(new Uri(uri), factory);
            _client.ReconnectTimeout = null;
            _client.ErrorReconnectTimeout = TimeSpan.FromSeconds(5);
        }

        public void StartClient()
        {
            _client.Start();
        }
    }
}
