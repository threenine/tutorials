using Microsoft.AspNetCore.Mvc;

namespace Server;

public class WebSocketController(Notifier notifier) : ControllerBase
{
    [Route("/")]
    public async Task Get()
    {
        if (HttpContext.WebSockets.IsWebSocketRequest)
        {
            using var webSocket = await HttpContext.WebSockets.AcceptWebSocketAsync();
            await notifier.Handle(webSocket);
        }
        else
        {
            HttpContext.Response.StatusCode = StatusCodes.Status400BadRequest;
        }
    }
}