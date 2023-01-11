import { createPublisher } from "..";

async function run() {
  const publisher = (await createPublisher())!;

  const temp = publisher.publish("/hello/world", "hello");

  console.log({ publisher, temp });

  while (true) {
    await new Promise((resolve) => {
      setTimeout(resolve, 500);
    });

    // const batch = publisher.startBatch();
    // temp.update(batch, getCpuTemp());

    // await batch.commit();
  }
}

run();
