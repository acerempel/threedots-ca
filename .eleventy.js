const { DateTime } = require("luxon");
const Nunjucks = require("nunjucks");
const Toml = require("toml");
const RssPlugin = require("@11ty/eleventy-plugin-rss");

module.exports = function(config) {
  config.addPassthroughCopy("styles");
  config.addPassthroughCopy("bundle.js");

  config.addPlugin(RssPlugin);

  config.addLayoutAlias('base', 'base.njk');
  config.addLayoutAlias('article', 'page-basic.njk');
  config.addLayoutAlias('post', 'page-post.njk');

  let dateFromJSDate = function(date) {
    return DateTime.fromJSDate(date, {zone: 'utc'});
  };
  let niceDate = function(dateObj) {
    return dateFromJSDate(dateObj).toLocaleString(DateTime.DATE_FULL);
  };
  let isoDate = function(dateObj) {
    return dateFromJSDate(dateObj).toISODate();
  };
  config.addFilter("nice_date", niceDate);
  config.addFilter("iso_date", isoDate);
  config.addShortcode("date", function(date) {
    let theDate = dateFromJSDate(date);
    return `<time datetime="${theDate.toISODate()}">${theDate.toLocaleString(DateTime.DATE_FULL)}</time>`;
  });

  config.addShortcode("note", function (index) {
    return `<sup id="fnref${index}"><a href="#fn${index}" aria-label="Footnote ${index}">${index}</a></sup>`;
  });

  let njkOpts = { autoescape: true, throwOnUndefined: true };
  let njkEnv = new Nunjucks.Environment(new Nunjucks.FileSystemLoader('content/_templates'), njkOpts);
  config.setLibrary("njk", njkEnv);

  config.addShortcode("link", function(page) {
    let titleAttr = page.data.description ? ` title="${page.data.description}"` : "";
    let typeAttr = page.data.type ? ` type="${page.data.type}"` : "";
    let linkContent = page.data.link_text || page.data.title;
    return `<a href="${page.url}"${titleAttr}${typeAttr}>${linkContent}</a>`;
  });

  let compare_weights = function(a, b) {
    let aw = a.data.weight;
    let ab = b.data.weight;
    return aw <= ab ? -1 : 1;
  };

  config.addCollection("top_nav", function(coll) {
    return coll.getFilteredByTag("nav").reverse().sort(compare_weights);
  });

  config.addCollection("lists", function(coll) {
    return coll.getFilteredByTag("misc_list").sort((a, b) => a.data.date_revised >= b.data.date_revised ? -1 : 1);
  });

  const excerptMarker = "<!-- FOLD -->";
  config.addNunjucksFilter("has_excerpt", function(page) {
    return page.templateContent.includes(excerptMarker);
  });
  config.addNunjucksFilter("excerpt", function(page) {
    return page.templateContent.split(excerptMarker)[0];
  });
  config.addFilter("groupByYear", function(arr) {
    let postsByYear = {};
    arr.forEach(function(post) {
      let yearString = Number(post.date.getFullYear()).toString();
      if (!(postsByYear[yearString])) { postsByYear[yearString] = [post]; } else { postsByYear[yearString].push(post); };
    });
    return Object.entries(postsByYear).sort((pair1, pair2) => pair2[0].localeCompare(pair1[0]));
  });

  config.setFrontMatterParsingOptions({
    engines: { toml: Toml.parse.bind(Toml) }
  });

  return {
    templateFormats: ["md", "html", "njk"],
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",
    dataTemplateEngine: "njk",
    dir: {
      input: "content",
      output: "_site",
      includes: "_templates",
      data: "data"
    },
  }
}
