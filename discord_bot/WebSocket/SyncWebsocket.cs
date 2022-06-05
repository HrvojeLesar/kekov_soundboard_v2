using System.Reactive.Linq;
using Serilog;

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
                Log.Warning($"Websocket disconnection happaned, type: {info.Type}");
            });
            _client.ReconnectionHappened.Subscribe(info =>
            {
                Log.Warning($"Websocket reconnection happaned, type: {info.Type}");
            });
        }
    }
}
