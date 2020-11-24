import { setUpControl, setColourScheme } from './colour-scheme.js';

import littlefoot from 'littlefoot';

import set_greetings from './greeting.js';

// This script should have the 'defer' attribute set, so that the
// 'DOMContentLoaded' event will not yet have fired when it is run.
document.addEventListener("DOMContentLoaded", function(_event) {
  setUpControl("colour-scheme", "change", setColourScheme);
  set_greetings();
  littlefoot();
});

