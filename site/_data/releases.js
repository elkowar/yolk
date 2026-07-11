import { readFileSync } from "node:fs";

const changelog = readFileSync("CHANGELOG.md", "utf8");
const releaseHeading =
  /^## \[(?<version>[^\]]+)](?:\((?<url>[^)]+)\))?(?: - (?<date>\d{4}-\d{2}-\d{2}))?/gm;
const releases = [];
const matches = [...changelog.matchAll(releaseHeading)];

for (let index = 0; index < matches.length; index += 1) {
  const match = matches[index];
  const next = matches[index + 1];
  const body = changelog
    .slice(match.index + match[0].length, next?.index ?? changelog.length)
    .trim();

  if (match.groups.version === "Unreleased") {
    continue;
  }

  releases.push({
    version: match.groups.version,
    url: match.groups.url,
    date: match.groups.date,
    body,
  });
}

export default releases;
