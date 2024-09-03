import express from "express";

if (!process.env.PORT) {
  throw new Error("PORT has not been provided");
}

const app = express();

const port = process.env.PORT;

app.get("/", (req, res) => {
  res.send(`worker ${+port.toString().slice(1)}`)
});

app.listen(port, () => {
  console.log("Server listening on port: ", port);
});