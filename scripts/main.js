import { setUpControl, setFontSize, setColourScheme } from './colour-scheme.js';

import littlefoot from 'littlefoot';

// This script should have the 'defer' attribute set, so that the
// 'DOMContentLoaded' event will not yet have fired when it is run.
document.addEventListener("DOMContentLoaded", function(_event) {
  setUpControl("font-size", "input", setFontSize);
  setUpControl("colour-scheme", "change", setColourScheme);
});

littlefoot();
