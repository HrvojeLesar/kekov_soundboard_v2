using Websocket.Client;

namespace KekovBot
{
    public class WebsocketController
    {
        protected WebsocketClient _client;

        protected WebsocketController(String uri)
        {
            _client = new WebsocketClient(new Uri(uri));
            _client.ReconnectTimeout = null;
            _client.ErrorReconnectTimeout = TimeSpan.FromSeconds(5);
        }

        public void StartClient() {
            _client.Start();
        }
    }
}
