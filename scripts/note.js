import {LitElement, html, css, unsafeCSS} from 'lit-element';
import baseStyles from '../bontent/base.css';

class Note extends LitElement {
  static get properties() {
    return {
      open: { type: Boolean },
      id: { type: String }
    };
  }
  constructor() {
    super();
    this.open = false;
    this.id = Math.random().toString(); // a bit hacky
  }
  static get styles() {
    return css`
      ${unsafeCSS(baseStyles)}
      .note-toggle {
        font-size: calc(var(--base-font-size)*0.9375);
        color: var(--text-colour-highlight);
      }
      .note-content {
        color: var(--text-colour-lighter);
      }
    `;
  }
  render() {
    return html`
      <span
        class="note-toggle"
        role="button"
        tabindex="0"
        title="Show/hide note"
        aria-label="Show/hide note"
        aria-expanded="${ this.open }"
        aria-pressed="${ this.open }"
        aria-controls="${ this.id }"
        @click="${ (_) => { this.open = !this.open } }"
      >[${ this.open ? '−' : '+' }]</span>
      <small
        ?hidden=${ !this.open } id="${ this.id }"
        role="note"
        class="smaller note-content"
      ><slot></slot></small>
    `;
  }
}

customElements.define('inline-note', Note);
