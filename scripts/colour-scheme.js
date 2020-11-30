'use strict';

var prefix = "colour-scheme-";
var colourSchemeClassNames = ['auto', 'light', 'dark'].map(function(c) {return prefix.concat(c)});

function setColourScheme(colourScheme) {
  var element = document.querySelector("body");
  var colourSchemeClassName = prefix.concat(colourScheme);
  element.classList.remove(...colourSchemeClassNames);
  element.classList.add(colourSchemeClassName);
}

function setFontSize(size) {
  document.documentElement.style.setProperty("--base-font-size", size + "rem");
}

function setLineHeight(size) {
  document.documentElement.style.setProperty("--base-line-height", size + "rem");
}

function setUpControl(elementId, eventName, applyValue) {
  var control = document.getElementById(elementId);
  if (!control) {
    console.log("Error! Element with id " + elementId + "not found!");
  }
  var stored = window.localStorage.getItem(elementId);
  if (stored) {
    applyValue(stored);
    control.value = stored;
  }
  var callback = function(event) {
    var val = event.target.value;
    applyValue(val);
    window.localStorage.setItem(elementId, val);
  };
  control.addEventListener(eventName, callback, {passive: true});
}

export { setUpControl, setFontSize, setColourScheme, setLineHeight };
