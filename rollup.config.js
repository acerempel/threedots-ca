import resolve from '@rollup/plugin-node-resolve';
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
    replace({ 'process.env.NODE_ENV': JSON.stringify('development') }),
    terser({ ecma: 2015 })
  ]
};
