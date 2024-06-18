# sjupdate

This is a simple script to update the sheetjs with the latest version.

## Why?

The Sheetjs team has decided not to release the latest version on the NPM repository.  
As a result, automatic updates through Dependabot or similar tools are not possible, and you will need to find the latest release in the RSS feed they provide.  
They also host the "latest" version on their [CDN server](https://cdn.sheetjs.com/), but we have no control over when it gets updated.
To address this, I have developed a program that checks the RSS feed, compares it to the package manifest, and updates it if needed.

## Usage

```bash
$ sjupdate -h
Usage: sjupdate [OPTIONS]

Options:
  -u, --url <URL>
          RSS feed URL for the sheetjs releases [default: https://git.sheetjs.com/sheetjs/sheetjs/tags.rss]
  -d, --directory <DIRECTORY>
          The path to the directory containing the package.json file. Must be relative to the project root [default: .]
  -p, --package-manager <PACKAGE_MANAGER>
          The package manager to use. Supported values are "npm", "yarn", and "pnpm" [default: npm] [possible values: npm, yarn, pnpm]
  -h, --help
          Print help
  -V, --version
          Print version
```