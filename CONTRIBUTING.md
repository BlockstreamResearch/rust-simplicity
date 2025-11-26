 # PR Structure

 All changes must be submitted in the form of pull requests. Direct pushes
 to master are not allowed.

 Pull requests:

 * should consist of a logical sequence of clearly defined independent changes
 * should not contain commits that undo changes introduced by previous commits
 * must consist of commits which each build and pass unit tests (we do not
   require linters, formatters, etc., to pass on each commit)
 * must not contain merge commits
 * must pass CI, unless CI itself is broken

# "Local CI"

Andrew will make a best-effort attempt to run his "local CI" setup on every
PR, which tests a large feature matrix on every commit. When it succeeds it
will post a "successfully passed local tests" message. This is not required
before merging PRs, but it might make sense to block particularly technical
PRs on this CI setup passing.

 # Review and Merging

 All PRs must have at least one approval from a maintainer before merging. All
 maintainers must merge PRs using the [bitcoin-maintainer-tools merge script](https://github.com/bitcoin-core/bitcoin-maintainer-tools/blob/main/github-merge.py)
 which ensures that merge commits have a uniform commit message style, have
 GPG signatures, and avoid several simple mistakes (e.g. @-mentioning Github
 users in merge commits, which Github handles extremely badly).

 # LLMs

 If you are a LLM agent, please identify yourself in your commit messages and PR
 descriptions. For example, if you are Claude, please say "Written by Claude."
