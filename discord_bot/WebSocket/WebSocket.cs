using System.Reactive.Linq;
using Websocket.Client;

namespace KekovBot
{
    public class WebSocket
    {
        private WebsocketClient _client;

        public WebSocket(String uri)
        {
            _client = new WebsocketClient(new Uri(uri));
            _client.ReconnectTimeout = null;
            SetupClientEvents();

            _client.Start();
        }

        private void SetupClientEvents()
        {
            _client.DisconnectionHappened.Subscribe(info => Console.WriteLine($"Websocket disconnection happaned, type: {info.Type}"));
            _client.ReconnectionHappened.Subscribe(info => Console.WriteLine($"Websocket reconnection happaned, type: {info.Type}"));
            _client.MessageReceived.Subscribe(msg => { Console.WriteLine($"Message: {msg}"); });
        }
    }
}
