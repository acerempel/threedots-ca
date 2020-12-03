import resolve from '@rollup/plugin-node-resolve';
import { string } from "rollup-plugin-string";

export default {
  input: 'scripts/main.js',
  output: {
    file: 'bontent/bundle.js',
    format: 'iife'
  },
  plugins: [
    resolve(),
    string({ include: "bontent/base.css" })]
};
