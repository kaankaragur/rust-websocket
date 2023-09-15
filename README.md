# Most basic in the world, rust web socket.

Possible with Tungstenite.

Let's get into a small introduction!

## What is this for?

Websockets are famous for communication between different clients.
The reason why I have done this is for my Roblox-alt-manager repository.
It is an interesting situation that you need a server to run a WebSocket.

Also, Luau doesn't support communication between clients with WebSockets, so this server is a component for websocket setup for your roblox client.
Logically, you can create your WebSocket client to communicate between your server and roblox. (Exploits are the best choice. They have support for listening web sockets).
By the way, I do my websockets setting from roblox.

## How is it work?

It's fairly straightforward; it involves utilizing your network to establish a WebSocket server.
You can configure ports on the network you use for the server and your home internet. (It's not recommended to use your home internet)

## Additional Information

This program is being detected as "an unrecognized app", so you will get a warning screen from Microsoft itself.


Virustotal: https://www.virustotal.com/gui/file/52c5f5592b2abd7e5b59366c6deb47554e0997678e3307d1ee38094a2b0bf937/detection


Or just build it yourself by

> cargo run

