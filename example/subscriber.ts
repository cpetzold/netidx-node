import { createSubscriber } from "..";

async function run() {
  const subscriber = createSubscriber((batch) => console.log("!", batch));

  subscriber.subscribe("/hello/world");

  while (true) {
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}

run();
