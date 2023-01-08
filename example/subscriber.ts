// Fake example for now

import { createSubscriber, Auth } from "netidx";

class Duration {}

type Result = { ok: true } | { ok: false; error: Error };

type Value =
  | null
  | boolean
  | number
  | string
  | Duration
  | Date
  | Buffer
  | Result
  | Value[];

interface Val {
  last(): Value;
  onUpdate(cb: (value: Value) => void): void;
}

interface Subscriber {
  subscribe(path: string): Val;
}

async function run() {
  const subscriber: Subscriber = await createSubscriber({
    auth: Auth.Anonymous,
  });

  const temp = subscriber.subscribe("/hw/washu-chan/cpu-temp");

  console.log(`washu-chan cpu temp is: ${temp.last()}`);

  temp.onUpdate((value) => {
    console.log(`washu-chan cpu temp is: ${value}`);
  });
}

run();
