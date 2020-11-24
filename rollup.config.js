import resolve from '@rollup/plugin-node-resolve';

export default {
  input: 'scripts/main.js',
  output: {
    file: 'bontent/bundle.js',
    format: 'iife'
  },
  plugins: [resolve()]
};
