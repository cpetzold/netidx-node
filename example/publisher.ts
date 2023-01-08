// Fake example for now

import { createPublisher, Auth, path } from "netidx";

function getCpuTemp() {
  return 42;
}

async function run() {
  const publisher = await createPublisher({
    auth: Auth.Anonymous,
  });

  const temp = publisher.publish("/hw/washu-chan/cpu-temp", getCpuTemp());

  while (true) {
    await new Promise((resolve) => {
      setTimeout(resolve, 500);
    });

    const batch = publisher.startBatch();
    temp.update(batch, getCpuTemp());

    await batch.commit();
  }
}

run();
