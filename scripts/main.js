import { setUpControl, setColourScheme, setLineHeight } from './colour-scheme.js';
import set_greetings from './greeting.js';
import './note.js';

// This script should have the 'defer' attribute set, so that the
// 'DOMContentLoaded' event will not yet have fired when it is run.
document.addEventListener("DOMContentLoaded", function(_event) {
  setUpControl("colour-scheme", "change", setColourScheme);
  setUpControl("line-height", "change", setLineHeight);
  set_greetings();
});

