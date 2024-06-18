# sjupdate

This is a simple script to update the sheetjs with the latest version.

## Why?

The Sheetjs team has decided not to release the latest version on the NPM repository.  
As a result, automatic updates through Dependabot or similar tools are not possible, and you will need to find the latest release in the RSS feed they provide.  
They also host the "latest" version on their [CDN server](https://cdn.sheetjs.com/), but we have no control over when it gets updated.
To address this, I have developed a program that checks the RSS feed, compares it to the package manifest, and updates it if needed.

## Usage

```bash
Usage: sjupdate [OPTIONS] <COMMAND>

Arguments:
  <COMMAND>  The command to run. Supported values are "release" and "update". The "release" command prints the latest version number to stdout. The "update" command updates the xlsx package to the latest version [possible values: release, update]

Options:
  -u, --url <URL>
          RSS feed URL for the sheetjs releases [default: https://git.sheetjs.com/sheetjs/sheetjs/tags.rss]
  -d, --directory <DIRECTORY>
          The path to the directory containing the package.json file. Must be relative to the project root [default: .]
  -p, --package-manager <PACKAGE_MANAGER>
          The package manager to use. Supported values are "npm", "yarn", and "pnpm" [default: npm] [possible values: npm, yarn, pnpm]
      --debug
          Enable debug mode. Prints debug information to stdout
  -h, --help
          Print help
  -V, --version
          Print version
```

## Example

### GitHub Actions

This is an example of how you can use this tool in a GitHub Actions workflow to automatically update the xlsx package.  
This workflow runs on the first day of every month and creates a pull request if the package needs to be updated.

```yaml
name: Update xlsx

on:
  schedule:
    - cron: '0 0 1 * *'

jobs:
  update:
    name: Update xlsx
    runs-on: ubuntu-latest

    steps:
      # Install latest version of sjupdate
      - name: Get latest release of sjupdate
        id: sjupdate_latest_release
        run: echo DOWNLOAD_URL=$(curl -s https://api.github.com/repos/miyamotota/sjupdate/releases/latest | jq -r '.assets[] | select(.name | contains("linux")) | .browser_download_url') >> $GITHUB_OUTPUT

      - name: Install sjupdate
        env:
          DOWNLOAD_URL: '${{ steps.sjupdate_latest_release.outputs.DOWNLOAD_URL }}'
        run: |
          sudo curl -fL -o sjupdate.tar.gz $DOWNLOAD_URL
          sudo tar -C /usr/bin -xzf ./sjupdate.tar.gz
          sudo rm sjupdate.tar.gz

      # Checkout your repository and set up Node.js
      - name: Checkout
        uses: actions/checkout@v4

      - name: Set up Node.js
        uses: actions/setup-node@v4
        with:
          node-version-file: '.tool-versions'

      # Run sjupdate and it will update the package manifest if needed
      - name: Run sjupdate
        run: sjupdate update

      # Make a pull request if the package manifest has changed
      # Make sure to set the workflow permissions to allow to make pull requests
      - name: Check for diff
        id: diff
        run: git diff --exit-code --quiet
        continue-on-error: true

      - name: No changes
        if: success() && steps.diff.outcome == 'success'
        run: echo 'No changes'

      - name: Latest version
        id: latest_version
        run: echo LATEST_VERSION=$(sjupdate release) >> $GITHUB_OUTPUT

      - name: Make pull request title
        id: pull_request_title
        run: |
          VERSION=${{ steps.latest_version.outputs.LATEST_VERSION }}
          TITLE="[sjupdate] Update xlsx to v${VERSION}"
          echo PULL_REQUEST_TITLE=$TITLE >> $GITHUB_OUTPUT

      - name: Find pull request
        if: steps.diff.conclusion == 'success' && steps.diff.outcome == 'failure'
        id: find_pull_request
        uses: actions/github-script@v7
        continue-on-error: true
        env:
          PULL_REQUEST_TITLE: '${{ steps.pull_request_title.outputs.PULL_REQUEST_TITLE }}'
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          script: |
            const title = process.env.PULL_REQUEST_TITLE
            console.log(`title: ${title}`)

            const pulls = await github.rest.pulls.list({
              owner: context.repo.owner,
              repo: context.repo.repo,
              state: 'all',
              labels: 'sjupdate',
              sort: 'created',
              direction: 'desc'
            })

            const data = pulls['data']
            const exists = data.some(pr => pr.title === title)
            console.log(exists)
            if (exists) {
              core.setFailed('A pull request already exists')
            }

      - name: Create pull request
        if: steps.find_pull_request.conclusion == 'success' && steps.find_pull_request.outcome == 'success'
        id: create_pull_request
        uses: peter-evans/create-pull-request@v5
        with:
          branch: 'sjupdate/xlsx/${{ steps.latest_version.outputs.LATEST_VERSION }}'
          title: '${{ steps.pull_request_title.outputs.PULL_REQUEST_TITLE }}'
          delete-branch: true
          labels: |
            sjupdate

      - name: Check outputs
        run: |
          echo "Pull Request Number - ${{ steps.create_pull_request.outputs.pull-request-number }}"
          echo "Pull Request URL - ${{ steps.create_pull_request.outputs.pull-request-url }}"
```