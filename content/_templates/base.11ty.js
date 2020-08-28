module.exports = function({title, description, collections, content, url}) {
  let title_str = title ? title + " … " : "";
  let descr_str = description ? `<meta name="description" content="${description}">` : ""

  let compare_weights = function(a, b) {
    let aw = a.data.weight;
    let ab = b.data.weight;
    return aw <= ab ? -1 : 1;
  };
  let nav_pages = collections.nav.slice().reverse().sort(compare_weights).map(page => this.link(page));
  let footer_nav_pages = collections.footer_nav.slice().reverse().sort(compare_weights);
  let build_footer_nav_link = (nav_page) => {
    return `<p>${this.link(nav_page)}</p>`
  };
  let nav_list = nav_pages.join(" • ");
  let footer_nav_list = footer_nav_pages.map(build_footer_nav_link).join("\n");

  return `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta content="text/html; charset=utf-8" http-equiv="ContentType">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <meta name="google-site-verification" content="DhZUgJjUNSRFdHhycAzNuCiTKprn-1Csb49PU1lsABo" >
    <link href="http://threedots.ca${this.page.url}" rel="canonical">
    <title>${title_str}‹three dots›</title>
    ${descr_str}
    <link rel="alternate" type="application/atom+xml" href="/posts/feed.xml">
    <link rel="stylesheet" href="/styles/littlefoot.css">
    <link href="/styles/styles.css" rel="stylesheet">
    <script defer src="/bundle.js"></script>
  </head>
  <body class="colour-scheme-auto">
    <div class="container">
      <header class="top-header">
        <a class="bold" href="/">three dots …</a>
        <nav>${nav_list}</nav>
      </header>
      <main class="mt-2-3 mb-1-1">
        ${content}
      </main>
      <footer class="mb-1-1 pt-1-1 border-top">
        <section class="mb-3-4 mr-1-1">
          <nav aria-label="Site navigation" class="link-plain">
          ${footer_nav_list}
          </nav>
        </section>
        <section>
          <p class="fs-1-1 lh-3-4 mt-1-3">
            <label for="colour-scheme">Colour scheme:</label>
            <select required id="colour-scheme">
              <option value="auto" selected>System setting</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </p>
        </section>
      </footer>
    </div>
    <script data-goatcounter="https://threedots_ca.goatcounter.com/count" async src="//gc.zgo.at/count.js"></script>
  </body>
</html>`;
};
