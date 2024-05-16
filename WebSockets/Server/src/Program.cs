
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using Server;

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddControllers();
builder.Services.AddSingleton<Notifier>();

var app = builder.Build();
app.UseWebSockets();
app.MapControllers();

app.Run();

