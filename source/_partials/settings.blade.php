          <details class="dropdown">
            <summary class="dropdown-toggle">Settings</summary>
            <div class="dropdown-inner">
              <p>
                <label for="colour-scheme">Colour scheme:</label>
                <select required id="colour-scheme">
                  <option value="auto" selected>System setting</option>
                  <option value="light">Light</option>
                  <option value="dark">Dark</option>
                </select>
              </p>
              <p>
                <label for="line-height">Spacing:</label>
                <input id="line-height" type="range" min="1.2" max="1.8" step="0.1" value="1.5">
              </p>
              <p>
                <input type="checkbox" value="fancy" id="fonts">
                <label for="fonts">Fancy fonts</label>
              </p>
            </div>
          </details>
