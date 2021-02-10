import { setUpControl, setColourScheme, setLineHeight } from './colour-scheme.js';
import set_greetings from './greeting.js';

// This script should have the 'defer' attribute set, so that the
// 'DOMContentLoaded' event will not yet have fired when it is run.
document.addEventListener("DOMContentLoaded", function(_event) {
  let setValue = (control, val) => { control.value = val };
  let loadFancyFonts = (val) => {
    if (val === 'fancy') {
      if (!window.fancyFontsLoaded) window.loadFancyFonts();
      document.body.classList.remove('fonts-default');
    } else {
      document.body.classList.add('fonts-default');
    }
  };
  setUpControl("colour-scheme", setColourScheme, setValue);
  setUpControl("line-height", setLineHeight, setValue);
  setUpControl("fonts", loadFancyFonts, (control, value) => { value === 'fancy' && (control.checked = true) });
  set_greetings();
  document.addEventListener("click", function(event) {
    let closestDropdown = event.target.closest(".dropdown");
    let dropdown;
    if (closestDropdown) return;
    if (dropdown = document.querySelector('.dropdown[open]')) {
      dropdown.open = false;
    }
    if (event.target.closest('.dropdown-nav-control')) return;
    if (dropdown = document.querySelector('.dropdown-nav.open')) {
      dropdown.classList.remove('open');
    }
  });
  document.querySelector('.dropdown-nav-control').addEventListener('click', (event) => {
    let dropdown = event.target.closest('.dropdown-nav');
    dropdown.classList.toggle("open");
  })
});
