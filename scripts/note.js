import {LitElement, html, css, unsafeCSS} from 'lit-element';
import baseStyles from '../bontent/base.css';
import { createPopper } from '@popperjs/core/lib/popper-lite';
import flip from '@popperjs/core/lib/modifiers/flip';
import preventOverflow from '@popperjs/core/lib/modifiers/preventOverflow';
import offset from '@popperjs/core/lib/modifiers/offset';

class Note extends LitElement {
  static get properties() {
    return {
      open: { type: Boolean, reflect: true },
      id: { type: String },
      popup: { type: Object, attribute: false }
    };
  }
  constructor() {
    super();
    this.open = false;
    this.id = Math.random().toString(); // a bit hacky
    this.popup = null;
  }
  static get styles() {
    return css`
      ${unsafeCSS(baseStyles)}
      .note-toggle {
        font-size: calc(var(--base-font-size)*0.9375);
        color: var(--text-colour-highlight);
        display: inline-block;
        border: none;
        background-color: transparent;
      }
      .note-content {
        background-color: var(--background-colour-lighter);
        border-radius: calc(var(--base-line-height)*0.125);
        border: calc(var(--base-line-height)*0.0625) solid var(--text-colour-normal);
        padding: calc(var(--base-line-height)*0.25) calc(var(--base-line-height)*0.375);
        width: 40ch;
        max-width: 80vw;
      }
    `;
  }
  render() {
    return html`
      <button
        class="note-toggle"
        tabindex="0"
        title="Show/hide note"
        aria-label="Show/hide note"
        aria-expanded="${ this.open }"
        aria-pressed="${ this.open }"
        aria-controls="${ this.id }"
        @click="${ this._toggle }"
      >✱</button>
      <small
        ?hidden=${ !this.open } id="${ this.id }"
        role="note"
        class="smaller note-content"
      ><slot></slot></small>
    `;
  }
  _toggle(_ev) {
    if (this.open) {
      this.open = false;
      this.popup.destroy();
    } else {
      this.open = true;
      let toggle = this.shadowRoot.querySelector(".note-toggle");
      let note = this.shadowRoot.querySelector(".note-content");
      let main = document.querySelector("main");
      offset.options = { offset: [0, 5] };
      preventOverflow.options = { boundary: main };
      this.popup = createPopper(toggle, note, {
        modifiers: [flip, preventOverflow, offset],
        placement: "bottom"
      });
    };
  }
}

customElements.define('inline-note', Note);
