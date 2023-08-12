const express = require("express");
const app = express();
const http = require("http").Server(app);
const io = require("socket.io")(http);
const net = require("net");

// Use express.json() middleware to parse request bodies
app.use(express.json());

const clients = [];

app.get("/", (req, res) => {
  res.sendFile(__dirname + "/index.html");
});
// Endpoint to handle POST requests
app.post("/qr", (req, res) => {
  const qrData = req.body;

  console.log("Received QR Data:", qrData);
  io.to("web_room").emit("barcode", qrData);

  if (qrData.qr === "http://en.m.wikipedia.org") {
    io.emit("open", 1);
    console.log("Sending open to rele 1");
  }
  // Send a response back to the client
  res.status(200).json({ message: "QR data received successfully" });
});

// Create a TCP server
const tcpServer = net.createServer((client) => {
  console.log("A client connected");

  // Listen for data from the client
  client.on("data", (data) => {
    // Emit the image data to all connected clients
    //console.log(data);
    io.to("web_room").emit("frame", data);
  });

  // Handle client disconnect
  client.on("end", () => {
    console.log("A client disconnected");
  });
});

// Start the TCP server
tcpServer.listen(3001, () => {
  console.log("TCP server is listening on port 3001");
});

io.on("connection", (socket) => {
  var clientIp = socket.request.connection.remoteAddress;

  console.log(clientIp);
  socket.on("storeClientInfo", (data) => {
    const client = {
      name: data.name,
      id: socket.id,
    };
    clients.push(client);
    console.log("conected new " + data.name);
    if (client.name === "web") {
      socket.join("web_room");
    }
    console.log(clients);
  });

  socket.on("disconnect", (data) => {
    for (let i = 0; i < clients.length; ++i) {
      const c = clients[i];
      if (c.id === socket.id) {
        clients.splice(i, 1);
        break;
      }
    }
    console.log(clients);
  });

  socket.on("open", (data) => {
    socket.broadcast.emit("open", data);
    console.log("open: " + data);
  });
});

module.exports = http;
