(function () {
  'use strict';

  var prefix = "colour-scheme-";
  var colourSchemeClassNames = ['auto', 'light', 'dark'].map(function(c) {return prefix.concat(c)});

  function setColourScheme(colourScheme) {
    var element = document.querySelector("body");
    var colourSchemeClassName = prefix.concat(colourScheme);
    element.classList.remove(...colourSchemeClassNames);
    element.classList.add(colourSchemeClassName);
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

  function getStyle(element, property) {
      var _a;
      const view = ((_a = element.ownerDocument) === null || _a === void 0 ? void 0 : _a.defaultView) || window;
      const style = view.getComputedStyle(element);
      return (style.getPropertyValue(property) || style[property]);
  }

  const PIXELS_PER_INCH = 96;
  const MILLIMETRES_PER_INCH = 25.4;
  const POINTS_PER_INCH = 72;
  const PICAS_PER_INCH = 6;
  function fontSize(element) {
      return element
          ? getStyle(element, 'fontSize') || fontSize(element.parentElement)
          : getStyle(window.document.documentElement, 'fontSize');
  }
  function parse(providedLength) {
      var _a;
      const length = providedLength || '0';
      const value = parseFloat(length);
      const match = length.match(/[\d-.]+(\w+)$/);
      const unit = (_a = match === null || match === void 0 ? void 0 : match[1]) !== null && _a !== void 0 ? _a : '';
      return [value, unit.toLowerCase()];
  }
  function pixels(length, element) {
      var _a, _b;
      const view = (_b = (_a = element === null || element === void 0 ? void 0 : element.ownerDocument) === null || _a === void 0 ? void 0 : _a.defaultView) !== null && _b !== void 0 ? _b : window;
      const root = view.document.documentElement || view.document.body;
      const [value, unit] = parse(length);
      switch (unit) {
          case 'rem':
              return value * pixels(fontSize(window.document.documentElement));
          case 'em':
              return value * pixels(fontSize(element), element === null || element === void 0 ? void 0 : element.parentElement);
          case 'in':
              return value * PIXELS_PER_INCH;
          case 'q':
              return (value * PIXELS_PER_INCH) / MILLIMETRES_PER_INCH / 4;
          case 'mm':
              return (value * PIXELS_PER_INCH) / MILLIMETRES_PER_INCH;
          case 'cm':
              return (value * PIXELS_PER_INCH * 10) / MILLIMETRES_PER_INCH;
          case 'pt':
              return (value * PIXELS_PER_INCH) / POINTS_PER_INCH;
          case 'pc':
              return (value * PIXELS_PER_INCH) / PICAS_PER_INCH;
          case 'vh':
              return (value * view.innerHeight || root.clientWidth) / 100;
          case 'vw':
              return (value * view.innerWidth || root.clientHeight) / 100;
          case 'vmin':
              return ((value *
                  Math.min(view.innerWidth || root.clientWidth, view.innerHeight || root.clientHeight)) /
                  100);
          case 'vmax':
              return ((value *
                  Math.max(view.innerWidth || root.clientWidth, view.innerHeight || root.clientHeight)) /
                  100);
          default:
              return value;
      }
  }

  function throttle(fn, delay = 0) {
      let lastCalled = 0;
      let timer;
      const throttledFn = (...args) => {
          const remainingDelay = Math.max(0, lastCalled + delay - Date.now());
          if (remainingDelay) {
              clearTimeout(timer);
              timer = setTimeout(() => {
                  lastCalled = Date.now();
                  fn(...args);
              }, remainingDelay);
          }
          else {
              lastCalled = Date.now();
              fn(...args);
          }
      };
      return Object.assign(throttledFn, {
          cancel: () => {
              lastCalled = 0;
              clearTimeout(timer);
          },
      });
  }

  // Public: Create a new SelectorSet.
  function SelectorSet() {
    // Construct new SelectorSet if called as a function.
    if (!(this instanceof SelectorSet)) {
      return new SelectorSet();
    }

    // Public: Number of selectors added to the set
    this.size = 0;

    // Internal: Incrementing ID counter
    this.uid = 0;

    // Internal: Array of String selectors in the set
    this.selectors = [];

    // Internal: Map of selector ids to objects
    this.selectorObjects = {};

    // Internal: All Object index String names mapping to Index objects.
    this.indexes = Object.create(this.indexes);

    // Internal: Used Object index String names mapping to Index objects.
    this.activeIndexes = [];
  }

  // Detect prefixed Element#matches function.
  var docElem = window.document.documentElement;
  var matches =
    docElem.matches ||
    docElem.webkitMatchesSelector ||
    docElem.mozMatchesSelector ||
    docElem.oMatchesSelector ||
    docElem.msMatchesSelector;

  // Public: Check if element matches selector.
  //
  // Maybe overridden with custom Element.matches function.
  //
  // el       - An Element
  // selector - String CSS selector
  //
  // Returns true or false.
  SelectorSet.prototype.matchesSelector = function(el, selector) {
    return matches.call(el, selector);
  };

  // Public: Find all elements in the context that match the selector.
  //
  // Maybe overridden with custom querySelectorAll function.
  //
  // selectors - String CSS selectors.
  // context   - Element context
  //
  // Returns non-live list of Elements.
  SelectorSet.prototype.querySelectorAll = function(selectors, context) {
    return context.querySelectorAll(selectors);
  };

  // Public: Array of indexes.
  //
  // name     - Unique String name
  // selector - Function that takes a String selector and returns a String key
  //            or undefined if it can't be used by the index.
  // element  - Function that takes an Element and returns an Array of String
  //            keys that point to indexed values.
  //
  SelectorSet.prototype.indexes = [];

  // Index by element id
  var idRe = /^#((?:[\w\u00c0-\uFFFF\-]|\\.)+)/g;
  SelectorSet.prototype.indexes.push({
    name: 'ID',
    selector: function matchIdSelector(sel) {
      var m;
      if ((m = sel.match(idRe))) {
        return m[0].slice(1);
      }
    },
    element: function getElementId(el) {
      if (el.id) {
        return [el.id];
      }
    }
  });

  // Index by all of its class names
  var classRe = /^\.((?:[\w\u00c0-\uFFFF\-]|\\.)+)/g;
  SelectorSet.prototype.indexes.push({
    name: 'CLASS',
    selector: function matchClassSelector(sel) {
      var m;
      if ((m = sel.match(classRe))) {
        return m[0].slice(1);
      }
    },
    element: function getElementClassNames(el) {
      var className = el.className;
      if (className) {
        if (typeof className === 'string') {
          return className.split(/\s/);
        } else if (typeof className === 'object' && 'baseVal' in className) {
          // className is a SVGAnimatedString
          // global SVGAnimatedString is not an exposed global in Opera 12
          return className.baseVal.split(/\s/);
        }
      }
    }
  });

  // Index by tag/node name: `DIV`, `FORM`, `A`
  var tagRe = /^((?:[\w\u00c0-\uFFFF\-]|\\.)+)/g;
  SelectorSet.prototype.indexes.push({
    name: 'TAG',
    selector: function matchTagSelector(sel) {
      var m;
      if ((m = sel.match(tagRe))) {
        return m[0].toUpperCase();
      }
    },
    element: function getElementTagName(el) {
      return [el.nodeName.toUpperCase()];
    }
  });

  // Default index just contains a single array of elements.
  SelectorSet.prototype.indexes['default'] = {
    name: 'UNIVERSAL',
    selector: function() {
      return true;
    },
    element: function() {
      return [true];
    }
  };

  // Use ES Maps when supported
  var Map;
  if (typeof window.Map === 'function') {
    Map = window.Map;
  } else {
    Map = (function() {
      function Map() {
        this.map = {};
      }
      Map.prototype.get = function(key) {
        return this.map[key + ' '];
      };
      Map.prototype.set = function(key, value) {
        this.map[key + ' '] = value;
      };
      return Map;
    })();
  }

  // Regexps adopted from Sizzle
  //   https://github.com/jquery/sizzle/blob/1.7/sizzle.js
  //
  var chunker = /((?:\((?:\([^()]+\)|[^()]+)+\)|\[(?:\[[^\[\]]*\]|['"][^'"]*['"]|[^\[\]'"]+)+\]|\\.|[^ >+~,(\[\\]+)+|[>+~])(\s*,\s*)?((?:.|\r|\n)*)/g;

  // Internal: Get indexes for selector.
  //
  // selector - String CSS selector
  //
  // Returns Array of {index, key}.
  function parseSelectorIndexes(allIndexes, selector) {
    allIndexes = allIndexes.slice(0).concat(allIndexes['default']);

    var allIndexesLen = allIndexes.length,
      i,
      j,
      m,
      dup,
      rest = selector,
      key,
      index,
      indexes = [];

    do {
      chunker.exec('');
      if ((m = chunker.exec(rest))) {
        rest = m[3];
        if (m[2] || !rest) {
          for (i = 0; i < allIndexesLen; i++) {
            index = allIndexes[i];
            if ((key = index.selector(m[1]))) {
              j = indexes.length;
              dup = false;
              while (j--) {
                if (indexes[j].index === index && indexes[j].key === key) {
                  dup = true;
                  break;
                }
              }
              if (!dup) {
                indexes.push({ index: index, key: key });
              }
              break;
            }
          }
        }
      }
    } while (m);

    return indexes;
  }

  // Internal: Find first item in Array that is a prototype of `proto`.
  //
  // ary   - Array of objects
  // proto - Prototype of expected item in `ary`
  //
  // Returns object from `ary` if found. Otherwise returns undefined.
  function findByPrototype(ary, proto) {
    var i, len, item;
    for (i = 0, len = ary.length; i < len; i++) {
      item = ary[i];
      if (proto.isPrototypeOf(item)) {
        return item;
      }
    }
  }

  // Public: Log when added selector falls under the default index.
  //
  // This API should not be considered stable. May change between
  // minor versions.
  //
  // obj - {selector, data} Object
  //
  //   SelectorSet.prototype.logDefaultIndexUsed = function(obj) {
  //     console.warn(obj.selector, "could not be indexed");
  //   };
  //
  // Returns nothing.
  SelectorSet.prototype.logDefaultIndexUsed = function() {};

  // Public: Add selector to set.
  //
  // selector - String CSS selector
  // data     - Optional data Object (default: undefined)
  //
  // Returns nothing.
  SelectorSet.prototype.add = function(selector, data) {
    var obj,
      i,
      indexProto,
      key,
      index,
      objs,
      selectorIndexes,
      selectorIndex,
      indexes = this.activeIndexes,
      selectors = this.selectors,
      selectorObjects = this.selectorObjects;

    if (typeof selector !== 'string') {
      return;
    }

    obj = {
      id: this.uid++,
      selector: selector,
      data: data
    };
    selectorObjects[obj.id] = obj;

    selectorIndexes = parseSelectorIndexes(this.indexes, selector);
    for (i = 0; i < selectorIndexes.length; i++) {
      selectorIndex = selectorIndexes[i];
      key = selectorIndex.key;
      indexProto = selectorIndex.index;

      index = findByPrototype(indexes, indexProto);
      if (!index) {
        index = Object.create(indexProto);
        index.map = new Map();
        indexes.push(index);
      }

      if (indexProto === this.indexes['default']) {
        this.logDefaultIndexUsed(obj);
      }
      objs = index.map.get(key);
      if (!objs) {
        objs = [];
        index.map.set(key, objs);
      }
      objs.push(obj);
    }

    this.size++;
    selectors.push(selector);
  };

  // Public: Remove selector from set.
  //
  // selector - String CSS selector
  // data     - Optional data Object (default: undefined)
  //
  // Returns nothing.
  SelectorSet.prototype.remove = function(selector, data) {
    if (typeof selector !== 'string') {
      return;
    }

    var selectorIndexes,
      selectorIndex,
      i,
      j,
      k,
      selIndex,
      objs,
      obj,
      indexes = this.activeIndexes,
      selectors = (this.selectors = []),
      selectorObjects = this.selectorObjects,
      removedIds = {},
      removeAll = arguments.length === 1;

    selectorIndexes = parseSelectorIndexes(this.indexes, selector);
    for (i = 0; i < selectorIndexes.length; i++) {
      selectorIndex = selectorIndexes[i];

      j = indexes.length;
      while (j--) {
        selIndex = indexes[j];
        if (selectorIndex.index.isPrototypeOf(selIndex)) {
          objs = selIndex.map.get(selectorIndex.key);
          if (objs) {
            k = objs.length;
            while (k--) {
              obj = objs[k];
              if (obj.selector === selector && (removeAll || obj.data === data)) {
                objs.splice(k, 1);
                removedIds[obj.id] = true;
              }
            }
          }
          break;
        }
      }
    }

    for (i in removedIds) {
      delete selectorObjects[i];
      this.size--;
    }

    for (i in selectorObjects) {
      selectors.push(selectorObjects[i].selector);
    }
  };

  // Sort by id property handler.
  //
  // a - Selector obj.
  // b - Selector obj.
  //
  // Returns Number.
  function sortById(a, b) {
    return a.id - b.id;
  }

  // Public: Find all matching decendants of the context element.
  //
  // context - An Element
  //
  // Returns Array of {selector, data, elements} matches.
  SelectorSet.prototype.queryAll = function(context) {
    if (!this.selectors.length) {
      return [];
    }

    var matches = {},
      results = [];
    var els = this.querySelectorAll(this.selectors.join(', '), context);

    var i, j, len, len2, el, m, match, obj;
    for (i = 0, len = els.length; i < len; i++) {
      el = els[i];
      m = this.matches(el);
      for (j = 0, len2 = m.length; j < len2; j++) {
        obj = m[j];
        if (!matches[obj.id]) {
          match = {
            id: obj.id,
            selector: obj.selector,
            data: obj.data,
            elements: []
          };
          matches[obj.id] = match;
          results.push(match);
        } else {
          match = matches[obj.id];
        }
        match.elements.push(el);
      }
    }

    return results.sort(sortById);
  };

  // Public: Match element against all selectors in set.
  //
  // el - An Element
  //
  // Returns Array of {selector, data} matches.
  SelectorSet.prototype.matches = function(el) {
    if (!el) {
      return [];
    }

    var i, j, k, len, len2, len3, index, keys, objs, obj, id;
    var indexes = this.activeIndexes,
      matchedIds = {},
      matches = [];

    for (i = 0, len = indexes.length; i < len; i++) {
      index = indexes[i];
      keys = index.element(el);
      if (keys) {
        for (j = 0, len2 = keys.length; j < len2; j++) {
          if ((objs = index.map.get(keys[j]))) {
            for (k = 0, len3 = objs.length; k < len3; k++) {
              obj = objs[k];
              id = obj.id;
              if (!matchedIds[id] && this.matchesSelector(el, obj.selector)) {
                matchedIds[id] = true;
                matches.push(obj);
              }
            }
          }
        }
      }
    }

    return matches.sort(sortById);
  };

  var bubbleEvents = {};
  var captureEvents = {};
  var propagationStopped = new WeakMap();
  var immediatePropagationStopped = new WeakMap();
  var currentTargets = new WeakMap();
  var currentTargetDesc = Object.getOwnPropertyDescriptor(Event.prototype, 'currentTarget');

  function before(subject, verb, fn) {
    var source = subject[verb];

    subject[verb] = function () {
      fn.apply(subject, arguments);
      return source.apply(subject, arguments);
    };

    return subject;
  }

  function matches$1(selectors, target, reverse) {
    var queue = [];
    var node = target;

    do {
      if (node.nodeType !== 1) break;

      var _matches = selectors.matches(node);

      if (_matches.length) {
        var matched = {
          node: node,
          observers: _matches
        };

        if (reverse) {
          queue.unshift(matched);
        } else {
          queue.push(matched);
        }
      }
    } while (node = node.parentElement);

    return queue;
  }

  function trackPropagation() {
    propagationStopped.set(this, true);
  }

  function trackImmediate() {
    propagationStopped.set(this, true);
    immediatePropagationStopped.set(this, true);
  }

  function getCurrentTarget() {
    return currentTargets.get(this) || null;
  }

  function defineCurrentTarget(event, getter) {
    if (!currentTargetDesc) return;
    Object.defineProperty(event, 'currentTarget', {
      configurable: true,
      enumerable: true,
      get: getter || currentTargetDesc.get
    });
  }

  function canDispatch(event) {
    try {
      event.eventPhase;
      return true;
    } catch (_) {
      return false;
    }
  }

  function dispatch(event) {
    if (!canDispatch(event)) return;
    var events = event.eventPhase === 1 ? captureEvents : bubbleEvents;
    var selectors = events[event.type];
    if (!selectors) return;
    var queue = matches$1(selectors, event.target, event.eventPhase === 1);
    if (!queue.length) return;
    before(event, 'stopPropagation', trackPropagation);
    before(event, 'stopImmediatePropagation', trackImmediate);
    defineCurrentTarget(event, getCurrentTarget);

    for (var i = 0, len1 = queue.length; i < len1; i++) {
      if (propagationStopped.get(event)) break;
      var matched = queue[i];
      currentTargets.set(event, matched.node);

      for (var j = 0, len2 = matched.observers.length; j < len2; j++) {
        if (immediatePropagationStopped.get(event)) break;
        matched.observers[j].data.call(matched.node, event);
      }
    }

    currentTargets["delete"](event);
    defineCurrentTarget(event);
  }

  function on(name, selector, fn) {
    var options = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : {};
    var capture = options.capture ? true : false;
    var events = capture ? captureEvents : bubbleEvents;
    var selectors = events[name];

    if (!selectors) {
      selectors = new SelectorSet();
      events[name] = selectors;
      document.addEventListener(name, dispatch, capture);
    }

    selectors.add(selector, fn);
  }
  function off(name, selector, fn) {
    var options = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : {};
    var capture = options.capture ? true : false;
    var events = capture ? captureEvents : bubbleEvents;
    var selectors = events[name];
    if (!selectors) return;
    selectors.remove(selector, fn);
    if (selectors.size) return;
    delete events[name];
    document.removeEventListener(name, dispatch, capture);
  }

  var r=function(){return (r=Object.assign||function(t){for(var e,n=1,o=arguments.length;n<o;n++)for(var i in e=arguments[n])Object.prototype.hasOwnProperty.call(e,i)&&(t[i]=e[i]);return t}).apply(this,arguments)};function a(e){var n=parseFloat(getStyle(e,"marginLeft")),o=e.offsetWidth-n,i=e.offsetHeight,r=e.getBoundingClientRect(),a=r.left+o/2,s=r.top+i/2;return {top:s,bottom:window.innerHeight-s,leftRelative:a/window.innerWidth}}function s(e,n){var o=2*parseInt(getStyle(e,"marginTop"),10)+e.offsetHeight;return n.bottom<o&&n.bottom<n.top}function c(t){var e;null===(e=t.parentNode)||void 0===e||e.removeChild(t);}var l={activateDelay:100,activateOnHover:!1,allowDuplicates:!0,allowMultiple:!1,anchorParentSelector:"sup",anchorPattern:/(fn|footnote|note)[:\-_\d]/gi,dismissDelay:100,dismissOnUnhover:!1,footnoteSelector:"li",hoverDelay:250,numberResetSelector:"",scope:"",contentTemplate:'<aside class="littlefoot-footnote" id="fncontent:<% id %>"><div class="littlefoot-footnote__wrapper"><div class="littlefoot-footnote__content"><% content %></div></div><div class="littlefoot-footnote__tooltip"></div></aside>',buttonTemplate:'<button class="littlefoot-footnote__button littlefoot-footnote__button__ellipsis" id="<% reference %>" title="See Footnote <% number %>" aria-expanded="false" aria-label="Footnote <% number %>"><svg viewbox="0 0 31 6" preserveAspectRatio="xMidYMid"><circle r="3" cx="3" cy="3" fill="white"></circle><circle r="3" cx="15" cy="3" fill="white"></circle><circle r="3" cx="27" cy="3" fill="white"></circle></svg></button>'};function u(t,e){t.isReady()&&(t.dismiss(),setTimeout((function(){t.remove();}),e));}function f(n){var o=n.id,i=n.button,r=n.content,l=n.host,u=n.popover,f=n.wrapper,d=!1,v=0;return {id:o,activate:function(n){i.classList.add("is-changing"),i.setAttribute("aria-expanded","true"),i.classList.add("is-active"),i.insertAdjacentElement("afterend",u),u.style.maxWidth=document.body.clientWidth+"px";var o=getStyle(r,"maxHeight");v=Math.round(pixels(o,r)),"function"==typeof n&&n(u,i);},dismiss:function(){i.classList.add("is-changing"),i.setAttribute("aria-expanded","false"),i.classList.remove("is-active");},isActive:function(){return i.classList.contains("is-active")},isReady:function(){return !i.classList.contains("is-changing")},isHovered:function(){return d},ready:function(){u.classList.add("is-active"),i.classList.remove("is-changing");},remove:function(){c(u),i.classList.remove("is-changing");},reposition:function(){if(u.parentElement){var e=a(i),n=Math.min(v,function(e,n){var o=s(e,n),i=parseInt(getStyle(e,"marginTop"),10);return n[o?"top":"bottom"]-i-15}(u,e));r.style.maxHeight=n+"px",function(t,e){var n=s(t,e),o=t.dataset.footnotePosition,i=n?"top":"bottom";o!==i&&(t.dataset.footnotePosition=i,t.classList.remove("is-positioned-"+o),t.classList.add("is-positioned-"+i),t.style.transformOrigin=100*e.leftRelative+"% "+(n?"100%":"0"));}(u,e),u.offsetHeight<r.scrollHeight?(u.classList.add("is-scrollable"),r.setAttribute("tabindex","0")):(u.classList.remove("is-scrollable"),r.removeAttribute("tabindex"));}},resize:function(){if(u.parentElement){var e=a(i),n=r.offsetWidth,o=parseInt(getStyle(i,"marginLeft"),10),s=-e.leftRelative*n+o+i.offsetWidth/2;u.style.left=s+"px",f.style.maxWidth=n+"px",function(t,e){var n=t.querySelector(".littlefoot-footnote__tooltip");n&&(n.style.left=100*e+"%");}(u,e.leftRelative);}},startHovering:function(){d=!0;},stopHovering:function(){d=!1;},destroy:function(){c(l);}}}function d(t){return t.target}function v(t){return null==t?void 0:t.dataset.footnoteId}function m(t,e){return function(n){n.preventDefault();var o=v(d(n).closest("[data-footnote-id]")),i=o&&t(o);i&&e(i);}}function p(t){var e,r=t.dismissAll,a=t.lookup,s=t.hover,c=t.repositionAll,l=t.resizeAll,u=t.toggle,f=t.unhover,p=function(t,e,n){return function(o){var i=v(d(o).closest("[data-footnote-button]")),r=i&&t(i);r?e(r):d(o).closest("[data-footnote-popover]")||n();}}(a,u,r),h=(e=r,function(t){27===t.keyCode&&e();}),g=throttle(c,16),y=throttle(l,16),b=m(a,s),L=m(a,f);return document.addEventListener("touchend",p),document.addEventListener("click",p),document.addEventListener("keyup",h),document.addEventListener("gestureend",g),window.addEventListener("scroll",g),window.addEventListener("resize",y),on("mouseover","[data-footnote-id]",b),on("mouseout","[data-footnote-id]",L),function(){document.removeEventListener("touchend",p),document.removeEventListener("click",p),document.removeEventListener("keyup",h),document.removeEventListener("gestureend",g),window.removeEventListener("scroll",g),window.removeEventListener("resize",y),off("mouseover","[data-footnote-id]",b),off("mouseout","[data-footnote-id]",L);}}var h=function(t){return t.classList.add("footnote-print-only")};function g(t,e){return Array.from(t.querySelectorAll(e))}function y(t,e){return t.querySelector("."+e)||t.firstElementChild||t}function b(t){var e=document.createElement("div");return e.innerHTML=t,e.firstElementChild}function L(t){return void 0!==t}function E(t){var e,n,o=(e=t,n=":not(.footnote-print-only)",Array.from(e.children).filter((function(t){return 8!==t.nodeType&&(!n||t.matches(n))}))),i=o.filter((function(t){return "HR"===t.tagName}));o.length===i.length&&(i.concat(t).forEach(h),E(t.parentElement));}function w(t){var e=t.parentElement;c(t);var n=null==e?void 0:e.innerHTML.replace("[]","").replace("&nbsp;"," ").trim();e&&!n&&w(e);}function H(t,e){var n=b(t.body.outerHTML);g(n,'[href$="#'+t.referenceId+'"]').forEach(w);var o=n.innerHTML.trim();return {original:t,data:{id:""+(e+1),number:e+1,reference:"lf-"+t.referenceId,content:o.startsWith("<")?o:"<p>"+o+"</p>"}}}function x(t){var e=/<%=?\s*(\w+?)\s*%>/g;return function(n){return t.replace(e,(function(t,e){var o;return String(null!==(o=n[e])&&void 0!==o?o:"")}))}}function _(t,e){var o=x(t),i=x(e);return function(t){var e=t.original,r=t.data,a=r.id,s=b('<span class="littlefoot-footnote__host">'+o(r)+"</span>"),c=s.firstElementChild;c.dataset.footnoteButton="",c.dataset.footnoteId=a,c.dataset.footnoteNumber=""+r.number;var l=b(i(r));l.dataset.footnotePopover="",l.dataset.footnoteId=a;var u=y(l,"littlefoot-footnote__wrapper"),f=y(l,"littlefoot-footnote__content");return function(t,e){t.addEventListener("wheel",throttle(function(t){return function(e){var n=e.currentTarget,o=-e.deltaY;o>0&&t.classList.remove("is-fully-scrolled"),n&&o<=0&&o<n.clientHeight+n.scrollTop-n.scrollHeight&&t.classList.add("is-fully-scrolled");}}(e),16));}(f,l),{original:e,data:r,id:a,button:c,host:s,popover:l,content:f,wrapper:u}}}function A(t){var e,n,o,i=t.allowDuplicates,a=t.anchorParentSelector,s=t.anchorPattern,c=t.buttonTemplate,l=t.contentTemplate,u=t.footnoteSelector,d=t.numberResetSelector,v=t.scope,m=function(t,e,n){return g(t,n+' a[href*="#"]').filter((function(t){return (t.href+t.rel).match(e)}))}(document,s,v).map(function(t,e,n,o){var i=[];return function(r){var a="#"+r.href.split("#")[1].replace(/[:.+~*[\]]/g,"\\$&"),s=g(t,a).find((function(t){return e||!i.includes(t)})),c=null==s?void 0:s.closest(o);if(c){i.push(c);var l=r.closest(n)||r;return {reference:l,referenceId:l.id||r.id,body:c}}}}(document,i,a,u)).filter(L).map(H).map(d?(e=d,n=0,o=null,function(t){var i=t.original,a=t.data,s=i.reference.closest(e);return n=o===s?n+1:1,o=s,{original:i,data:r(r({},a),{number:n})}}):function(t){return t}).map(_(c,l));return m.forEach((function(t){var e=t.original,n=t.host;h(e.reference),h(e.body),E(e.body.parentElement),function(t,e){t.insertAdjacentElement("beforebegin",e);}(e.reference,n);})),m.map(f)}function T(t){t.forEach((function(t){return t.destroy()})),g(document,".footnote-print-only").forEach((function(t){return t.classList.remove("footnote-print-only")}));}function D(t){void 0===t&&(t={});var e=r(r({},l),t),n=function(t,e){var n=t.setup(e);function o(t,o){e.allowMultiple||n.filter((function(e){return e.id!==t.id})).forEach((function(t){return u(t,e.dismissDelay)})),t.isReady()&&(t.activate(e.activateCallback),t.reposition(),t.resize(),setTimeout((function(){t.ready();}),o));}var i={activate:o,dismiss:u,lookup:function(t){return n.find((function(e){return e.id===t}))},dismissAll:function(){n.forEach((function(t){return u(t,e.dismissDelay)}));},repositionAll:function(){n.forEach((function(t){return t.reposition()}));},resizeAll:function(){n.forEach((function(t){return t.resize()}));},toggle:function(t){t.isActive()?u(t,e.dismissDelay):o(t,e.activateDelay);},hover:function(t){t.startHovering(),e.activateOnHover&&!t.isActive()&&o(t,e.hoverDelay);},unhover:function(t){t.stopHovering(),e.dismissOnUnhover&&setTimeout((function(){n.filter((function(t){return !t.isHovered()})).forEach((function(t){return u(t,e.dismissDelay)}));}),e.hoverDelay);}},a=t.addListeners(i);return r(r({},i),{unmount:function(){a(),t.cleanup(n);}})}({setup:A,addListeners:p,cleanup:T},e);return {activate:function(t,o){void 0===o&&(o=e.activateDelay);var i=n.lookup(t);i&&n.activate(i,o);},dismiss:function(t,o){void 0===o&&(o=e.dismissDelay);var i=t&&n.lookup(t);i?n.dismiss(i,o):n.dismissAll();},unmount:n.unmount,getSetting:function(t){return e[t]},updateSetting:function(t,n){e[t]=n;}}}

  function set_greetings() {
    let greetings = document.querySelectorAll(".greeting");
    let today = new Date();
    let current_hour = today.getHours();
    let day_region;
    if (current_hour < 12) {
      day_region = "morning";
    } else if (current_hour < 17) {
      day_region = "afternoon";
    } else {
      day_region = "evening";
    }
    greetings.forEach((greet) => {
      greet.innerHTML = `Good ${day_region}`;
    });
  }

  // This script should have the 'defer' attribute set, so that the
  // 'DOMContentLoaded' event will not yet have fired when it is run.
  document.addEventListener("DOMContentLoaded", function(_event) {
    setUpControl("colour-scheme", "change", setColourScheme);
    set_greetings();
    D();
  });

}());
