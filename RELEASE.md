# How to make a new binstall release

1. update [CHANGELOG.md](CHANGELOG.md) and commit to `main`
2. replace all occurrences of `0.6.1` with the new version and commit to `main`
3. tag the new version in the codebase:

       git tag v0.6.1
4. push the new tag:

       git push --tags
5. verify that the release notes match CHANGELOG.md
6. publish the release
