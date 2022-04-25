using System.Reactive.Linq;
using Newtonsoft.Json;
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
            _client.ErrorReconnectTimeout = TimeSpan.FromSeconds(5);
            SetupClientEvents();

            _client.Start();
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
            _client.MessageReceived.Subscribe(async msg => await HandleMessage(msg));
        }

        private async Task HandleMessage(ResponseMessage msg)
        {
            Console.WriteLine($"Message: {msg}");
            ControlMessage? control = JsonConvert.DeserializeObject<ControlMessage>(msg.Text);
            try
            {
                switch (control?.OpCode)
                {
                    case OpCode.Play:
                        {
                            await Controls.Play(control);
                            break;
                        }
                    case OpCode.Stop:
                        {
                            break;
                        }
                    case OpCode.Connection:
                        {
                            break;
                        }
                    // Do not reply, ignore
                    case OpCode.StopResponse:
                    case OpCode.PlayResponse:
                        { break; }
                    default:
                        {
                            for (var i = 0; i < 10; i++)
                                Console.WriteLine("IMPLEMENT MISSING OP CODES!");
                            System.Environment.Exit(1);
                            break;
                        }
                }

                var respOpCode = OpCodeConverter.ToResponse(control.OpCode);
                if (respOpCode != null)
                {
                    var response = new ControlMessage((OpCode)respOpCode, control);
                    var json_response = JsonConvert.SerializeObject(response);
                    _client.Send(json_response);
                }
            }
            catch (WebSocketException e)
            {
                if (control != null) {
                    var respOpCode = ClientErrorConverter.ToClientError(e);
                    var response = new ControlMessage(respOpCode, control);
                    var json_response = JsonConvert.SerializeObject(response);
                    _client.Send(json_response);
                }
            }
            catch (Exception e)
            {
                Console.WriteLine(e);
            }
        }
    }
}
