function permalinkFor(page) {
  const stem = page.filePathStem.replace(/^\/book\//, "");

  if (stem === "getting_started") {
    return "/book/index.html";
  }

  if (stem.endsWith("/index")) {
    return `/book/${stem.replace(/\/index$/, "")}/index.html`;
  }

  return `/book/${stem}/index.html`;
}

function titleFor(page) {
  const slug = page.fileSlug === "getting_started" ? "Getting started" : page.fileSlug;
  return slug
    .split(/[-_]/)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

export default {
  layout: "docs.njk",
  styles: ["/assets/css/prose.css", "/assets/css/docs.css", "/book/pagefind/pagefind-component-ui.css"],
  eleventyComputed: {
    permalink: (data) => permalinkFor(data.page),
    title: (data) => titleFor(data.page),
  },
};
