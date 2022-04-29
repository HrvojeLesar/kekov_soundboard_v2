using System.Reactive.Linq;

namespace KekovBot
{
    public class SyncWebsocket : WebsocketController
    {
        public SyncWebsocket(String uri) : base(uri)
        {
            SetupClientEvents();
            StartClient();
        }

        // TODO: Needs resubscribing when after crash
        private void SetupClientEvents()
        {
            _client.DisconnectionHappened.Subscribe(info =>
            {
                Console.WriteLine($"Websocket disconnection happaned, type: {info.Type}");
            });
            _client.ReconnectionHappened.Subscribe(info =>
            {
                Console.WriteLine($"Websocket reconnection happaned, type: {info.Type}");
            });
        }
    }
}
