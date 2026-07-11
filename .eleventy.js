import markdownIt from "markdown-it";
import markdownItAnchor from "markdown-it-anchor";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const Prism = require("prismjs");
const loadPrismLanguage = require("prismjs/components/index.js");
loadPrismLanguage.silent = true;

const pathPrefix = process.env.ELEVENTY_PATH_PREFIX || "/yolk/";

const languageAliases = new Map([
  ["console", "bash"],
  ["elvish", "bash"],
  ["fish", "bash"],
  ["handlebars", "toml"],
  ["kdl", "javascript"],
  ["rhai", "rust"],
  ["shell", "bash"],
  ["sh", "bash"],
  ["ps1", "powershell"],
  ["pwsh", "powershell"],
  ["rs", "rust"],
  ["zsh", "bash"],
]);

function escapeHtml(input = "") {
  return input
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function normalizeLanguage(language = "") {
  const normalized = String(language).trim().split(/[,\s/]+/)[0].toLowerCase();
  return languageAliases.get(normalized) ?? normalized;
}

function highlightedCodeBlock(content = "", language = "text") {
  const normalizedLanguage = normalizeLanguage(language) || "text";
  let html = escapeHtml(content.trim());

  if (normalizedLanguage !== "text") {
    try {
      if (!Prism.languages[normalizedLanguage]) {
        loadPrismLanguage(normalizedLanguage);
      }
      if (Prism.languages[normalizedLanguage]) {
        html = Prism.highlight(content.trim(), Prism.languages[normalizedLanguage], normalizedLanguage);
      }
    } catch {
      html = escapeHtml(content.trim());
    }
  }

  return `<pre class="language-${escapeHtml(normalizedLanguage)}"><code class="language-${escapeHtml(normalizedLanguage)}">${html}</code></pre>`;
}

// A single markdown-it instance, used both to render the docs pages and by the
// `markdown` filter for the changelog release notes.
const md = markdownIt({
  html: true,
  highlight: (content, language) => highlightedCodeBlock(content, language),
}).use(markdownItAnchor, {
  permalink: markdownItAnchor.permalink.headerLink(),
});

// Rewrite relative `.md` links (used in the source docs) to clean URLs.
const defaultLinkOpen =
  md.renderer.rules.link_open ??
  ((tokens, idx, options, _env, self) => self.renderToken(tokens, idx, options));

md.renderer.rules.link_open = (tokens, idx, options, env, self) => {
  const hrefIndex = tokens[idx].attrIndex("href");

  if (hrefIndex >= 0) {
    const href = tokens[idx].attrs[hrefIndex][1];
    if (/\.md(#.*)?$/.test(href)) {
      tokens[idx].attrs[hrefIndex][1] = href
        .replace(/\/index\.md(?=#|$)/, "/")
        .replace(/\.md(?=#|$)/, "/");
    }
  }

  return defaultLinkOpen(tokens, idx, options, env, self);
};

export default function (eleventyConfig) {
  // The generated CLI reference (site/book/cli_reference.md) is gitignored, but
  // we still want Eleventy to render it. Everything else under the input dir is
  // tracked, so it's safe to stop honoring .gitignore here.
  eleventyConfig.setUseGitIgnore(false);
  eleventyConfig.setLibrary("md", md);
  eleventyConfig.addFilter("markdown", (input = "") => md.render(input));
  eleventyConfig.addFilter("codeblock", highlightedCodeBlock);

  eleventyConfig.addPassthroughCopy({
    ".github/images": "assets/images",
    "site/assets": "assets",
  });

  eleventyConfig.addFilter("releaseUrl", (version = "") => {
    return `https://github.com/elkowar/yolk/releases/tag/v${String(version).replace(/^v/, "")}`;
  });

  return {
    pathPrefix,
    markdownTemplateEngine: false,
    htmlTemplateEngine: "njk",
    dir: {
      input: "site",
      output: "public",
      includes: "_includes",
      data: "_data",
    },
  };
}
