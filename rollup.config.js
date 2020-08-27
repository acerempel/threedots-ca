import resolve from '@rollup/plugin-node-resolve';

export default {
  input: 'scripts/main.js',
  output: {
    file: 'bundle.js',
    format: 'iife'
  },
  plugins: [resolve()]
};
