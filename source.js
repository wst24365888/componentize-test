import { f } from 'wasi:foo/bar@0.1.0';

export function sayHello (name) {
  return `${printHello()} ${name}!`
}