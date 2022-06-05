using System.Reactive.Linq;
using Newtonsoft.Json;
using Serilog;
using Websocket.Client;

namespace KekovBot
{
    public class ControlsWebsocket : WebsocketController
    {
        public ControlsWebsocket(String uri) : base(uri)
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
            _client.MessageReceived.Subscribe(async msg => await HandleMessage(msg));
        }

        private async Task HandleMessage(ResponseMessage msg)
        {
            Log.Debug($"Message: {msg}");
            ControlMessage? control = JsonConvert.DeserializeObject<ControlMessage>(msg.Text);
            List<Sound>? queue = null;
            try
            {
                bool addedToQueue = false;
                switch (control?.OpCode)
                {
                    case OpCode.Play:
                        {
                            addedToQueue = await Controls.Play(control);
                            break;
                        }
                    case OpCode.Stop:
                        {
                            await Controls.Stop(control);
                            break;
                        }
                    case OpCode.Skip:
                        {
                            await Controls.Skip(control);
                            break;
                        }
                    case OpCode.GetQueue:
                        {
                            queue = await Controls.GetQueue(control);
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
                                Log.Error("IMPLEMENT MISSING OP CODES!");
                            System.Environment.Exit(1);
                            break;
                        }
                }

                var respOpCode = OpCodeConverter.ToResponse(control.OpCode, addedToQueue);
                if (respOpCode != null)
                {
                    var response = new ControlMessage((OpCode)respOpCode, queue, control);
                    var json_response = JsonConvert.SerializeObject(response);
                    _client.Send(json_response);
                }
            }
            catch (WebSocketException e)
            {
                Log.Error(e.ToString());
                if (control != null)
                {
                    var respOpCode = ClientErrorConverter.ToClientError(e);
                    var response = new ControlMessage(respOpCode, control);
                    var json_response = JsonConvert.SerializeObject(response);
                    _client.Send(json_response);
                }
            }
            catch (Exception e)
            {
                Log.Error(e.ToString());
            }
        }
    }
}
