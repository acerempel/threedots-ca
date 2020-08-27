module.exports = function({title, description, collections, content, url}) {
  let title_str = title ? title + " … " : "";
  let descr_str = description ? `<meta name="description" content="${description}">` : ""

  let compare_weights = function(a, b) {
    let aw = a.data.weight;
    let ab = b.data.weight;
    return aw <= ab ? -1 : 1;
  };
  let nav_pages = collections.nav.slice().reverse().sort(compare_weights);
  let build_nav_link = (nav_page) => {
    return `<p class="fs-1-1 lh-3-4 mt-1-3">${this.link(nav_page)}</p>`
  };
  let nav_list = nav_pages.map(build_nav_link).join("\n");

  let google_analytics = `
    <!-- Global site tag (gtag.js) - Google Analytics -->
    <script async src="https://www.googletagmanager.com/gtag/js?id=UA-172347531-1"></script>
    <script>
      window.dataLayer = window.dataLayer || [];
      function gtag(){dataLayer.push(arguments);}
      gtag('js', new Date());

      gtag('config', 'UA-172347531-1');
    </script>`;

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
    <link rel="me" href="https://x0r.be/@alan">
  </head>
  <body class="colour-scheme-auto">
    <div class="container">
      <header>
        <a class="semibold heading-like" href="/">three dots …</a>
      </header>
      <main class="mt-2-3 mb-1-1">
        ${content}
      </main>
      <footer class="mb-1-1 pt-1-1 border-top">
        <section class="mb-3-4 mr-1-1">
          <nav aria-label="Site navigation" class="link-plain">
          ${nav_list}
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
          <p>
            <label for="font-size">Font size:</label>
            <input type="range" tabindex=0 id="font-size" min="0.8" max="1.6" step="0.1" value="1.0">
          </p>
        </section>
      </footer>
    </div>
    <script data-goatcounter="https://threedots_ca.goatcounter.com/count" async src="//gc.zgo.at/count.js"></script>
  </body>
</html>`;
};
