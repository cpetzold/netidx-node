import { createSubscriber } from "..";

async function run() {
  const subscriber = (await createSubscriber())!;

  const temp = subscriber.subscribe("/hello/world");

  temp.onUpdate((value) => console.log("!!", value));
}

run();
