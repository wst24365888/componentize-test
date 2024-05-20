import { printHello } from 'wasi:foo/bar@0.1.0';

export const greeting = {
  sayHello: (name) => {
    return `${printHello()} ${name}!`
  }
}