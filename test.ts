class Random {
  random = 0;
  constructor() {
    this.random = Math.floor(Math.random() * 10000000);
  }
}
export const __classNames = (...args: any[]) => args.filter(Boolean).join(' ');

const { random } = new Random();

console.log(random)

export interface HelloWorld {
  name: string
  nested: {
    value: string
  }
}

export interface MySharedInterface {
  shared: boolean;
  items: number[];
}

/** Adds two numbers together */
export function add(x: number, y: number) {
  return x + y;
}

export const classNames = (...args: any[]) => args.filter(Boolean).join(' ');

