import { EventEmitter } from "stream";
import {
  createSubscriber,
  Duration,
  F32,
  F64,
  I32,
  I64,
  U32,
  U64,
  V32,
  V64,
  Z32,
  Z64,
  Subscriber as SubscriberInner,
  EventType,
} from "..";

type Value =
  | null
  | boolean
  | string
  | number
  | U32
  | V32
  | Z32
  | I32
  | U64
  | V64
  | Z64
  | I64
  | F32
  | F64
  | Duration
  | Date
  | Buffer
  | Value[];

type SubscribeEvent = {
  path: string;
  type: EventType;
  value?: Value;
};

class Subscriber extends EventEmitter {
  idToPath: Map<bigint, string>;
  pathToId: Map<string, bigint>;
  inner: SubscriberInner;

  constructor() {
    super();

    this.idToPath = new Map();
    this.pathToId = new Map();

    this.inner = createSubscriber((batch) => {
      batch.forEach(({ id, type, value }) => {
        const path = this.idToPath.get(id)!;
        const event: SubscribeEvent = { type, value: value as Value, path };
        if (path) {
          this.emit(path, event);
        }
        this.emit("any", event);
      });
    });

    this.on("newListener", (path) => {
      const id = this.inner.subscribe(path);
      this.idToPath.set(id, path);
      this.pathToId.set(path, id);
    });

    this.on("removeListener", (path) => {
      const id = this.pathToId.get(path);
      if (this.listenerCount(path) === 0 && id) {
        this.inner.unsubscribe(id);
      }
    });
  }

  any(callback: (event: SubscribeEvent) => void) {
    this.on("any", callback);
    return () => this.off("any", callback);
  }

  subscribe(path: string, callback: (event: SubscribeEvent) => void) {
    this.on(path, callback);
    return () => this.off(path, callback);
  }
}

async function run() {
  const subscriber = new Subscriber();

  subscriber.subscribe("/", ({ path, type, value }) => {
    if (type === EventType.Unsubscribed) {
      console.log(`Unsubscribed|${path}`);
    } else {
      console.log(`${path}|${typeof value}|${value}`);
    }
  });

  subscriber.any(console.log);

  while (true) {
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}

run();
