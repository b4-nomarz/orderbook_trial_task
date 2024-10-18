<script lang="ts">
  import { onMount, onDestroy, beforeUpdate, afterUpdate } from "svelte";

  // "BTCUSDC, ETHUSDC, etc."
  type Pair = string;
  type Value = string;

  type Message = {
    p: Pair;
    v: Value;
  };

  type PairQuery = {
    p: Pair;
  };

  // Component state setters and getter
  let orderBookPair: Pair | undefined = undefined;
  let orderBookAverage: Value = "Not connected to server :/";

  const updateOrderBookPair = (newPair: string) => {
    orderBookPair = newPair;
  };

  const updateOrderBookAverage = (newAverage: string) => {
    orderBookAverage = newAverage;
  };

  // Websocket functions
  const theSocket = new WebSocket(
    `ws://${location.host}/api/average_order_book_price`,
  );

  // initial message is hard coded for now
  const initialMessage: PairQuery = {
    p: "BTCUSDC",
  };

  const sendMessage = (socket: WebSocket) => {
    socket.send(JSON.stringify(initialMessage));
  };

  // puts callback in closures to be called in svelte lifecycle effects
  const socketOnOpen = (socket: WebSocket) => {
    theSocket.onopen = () => {
      sendMessage(theSocket);
    };
  };

  const socketOnMessage = (socket: WebSocket) => {
    return (socket.onmessage = (evt) => {
      if (socket.readyState === WebSocket.OPEN) {
        const data = JSON.parse(evt.data);
        updateOrderBookAverage(data.v);
        updateOrderBookPair(data.p);

        // keeps connection alive
        sendMessage(socket);
      }
    });
  };

  // for debugging as socket can be disconnected from the clientside
  // and will always call close callback rather than error callback
  //theSocket.onclose = (evt) => {
  //  console.log(evt);
  //};

  // Compenent lifecycle and side effects
  onMount(() => {
    socketOnOpen(theSocket);
  });

  afterUpdate(() => {
    socketOnMessage(theSocket);
  });

  onDestroy(() => {
    theSocket.close();
  });
</script>

<div>
  <h1>Current Average Order Book Value</h1>
  <h2>{orderBookPair}: {orderBookAverage}</h2>
</div>
