import resolve from '@rollup/plugin-node-resolve';
import { string } from "rollup-plugin-string";
import replace from "@rollup/plugin-replace";
import { terser } from "rollup-plugin-terser";

export default {
  input: 'scripts/main.js',
  output: {
    file: 'bontent/bundle.js',
    format: 'iife'
  },
  plugins: [
    resolve(),
    string({ include: "bontent/base.css" }),
    replace({ 'process.env.NODE_ENV': JSON.stringify('development') }),
    terser()
  ]
};
