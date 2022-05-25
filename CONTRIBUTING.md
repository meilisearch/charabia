# Contributing

First, thank you for contributing to Meilisearch! The goal of this document is to provide everything you need to start contributing to the Meilisearch tokenizer.

Remember that there are many ways to contribute other than writing code: writing [tutorials or blog posts](https://github.com/meilisearch/awesome-meilisearch), improving [the documentation](https://github.com/meilisearch/documentation), submitting [bug reports](https://github.com/meilisearch/charabia/issues/new) and [feature requests](https://github.com/meilisearch/product/discussions/categories/feedback-feature-proposal)...

## Table of Contents
- [Assumptions](#assumptions)
- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Git Guidelines](#git-guidelines)
- [Release Process (for internal team only)](#release-process-for-internal-team-only)

## Assumptions

1. **You're familiar with [GitHub](https://github.com) and the [Pull Requests](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/about-pull-requests)(PR) workflow.**
2. **You know about the [Meilisearch community](https://docs.meilisearch.com/learn/what_is_meilisearch/contact.html).
   Please use this for help.**

## How to Contribute

1. Ensure your change has an issue! Find an
   [existing issue](https://github.com/meilisearch/charabia/issues/) or [open a new issue](https://github.com/meilisearch/charabia/issues/new).
   * This is where you can get a feel if the change will be accepted or not.
2. Once approved, [fork the Tokenizer repository](https://help.github.com/en/github/getting-started-with-github/fork-a-repo) in your own GitHub account.
3. [Create a new Git branch](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-and-deleting-branches-within-your-repository)
4. Review the [Development Workflow](#development-workflow) section that describes the steps to maintain the repository.
5. Make your changes on your branch.
6. [Submit the branch as a Pull Request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request-from-a-fork) pointing to the `main` branch of the origin repository. A maintainer should comment and/or review your Pull Request within a few days. Although depending on the circumstances, it may take longer.

## Development Workflow

### Test

```bash
cargo test
```

### Benchmark

```bash
cargo bench
```

### Implement a `Segmenter`
A `Segmenter` is a Script or Language specialized struct that segment a text in several [lemmas](https://en.wikipedia.org/wiki/Lemma_(morphology)) that will be classified as a separator or a word later in the tokenization pipeline.
A Segmenter will never change, add, or skip a lemma, that means that concatenating all lemmas must be equal to the original text.
All Segmenters implementation are stored in `src/segmenter`.

#### Start the implementation
We highly recommend to start the implementation by copy-pasting the dummy example (`src/segmenter/dummy_example.rs`) and follow the instructions in comments.

#### Add a Benchmark
The only thing needed is 2 texts detected as the `Segmenter`'s Script or Language by the tokenizer.
One that has a size of around 130 bytes and an other that has a size of around 365 bytes.
These 2 texts must be added in the `static DATA_SET` global located `benches/bench.rs`:

```rust
static DATA_SET: &[((usize, Script, Language), &str)] = &[
    // short texts (~130 bytes)
    [...]
    ((<size in bytes>, Script, Language), "<Text of around 130 bytes>"),

    // long texts (~365 bytes)
    [...]
    ((<size in bytes>, Script, Language), "<Text of around 365 bytes>"),
```

### Implement a `Normalizer`
A `Normalizer` is a struct used to alterate the lemma contained in a Token in order to remove features that doesn't sygnificantly impact the sens like lowecasing, removing accents, or converting Traditionnal Chinese characteres into Simplified Chinese characteres.

#### Start the implementation
We highly recommend to start the implementation by copy-pasting the dummy example (`src/normalizer/dummy_example.rs`) and follow the instructions in comments.

## Git Guidelines

### Git Branches

All changes must be made in a branch and submitted as PR.

We do not enforce any branch naming style, but please use something descriptive of your changes.

### Git Commits

As minimal requirements, your commit message should:
- be capitalized
- not finish by a dot or any other punctuation character (!,?)
- start with a verb so that we can read your commit message this way: "This commit will ...", where "..." is the commit message.
  e.g.: "Fix the home page button" or "Add more tests for create_index method"

We don't follow any other convention, but if you want to use one, we recommend [the Chris Beams one](https://chris.beams.io/posts/git-commit/).

### GitHub Pull Requests

Some notes on GitHub PRs:

- All PRs must be reviewed and approved by at least one maintainer.
- The PR title should be accurate and descriptive of the changes. The title of the PR will be indeed automatically added to the next [release changelogs](https://github.com/meilisearch/charabia/releases/).
- [Convert your PR as a draft](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/changing-the-stage-of-a-pull-request) if your changes are a work in progress: no one will review it until you pass your PR as ready for review.<br>
  The draft PRs are recommended when you want to show that you are working on something and make your work visible.
- The branch related to the PR must be **up-to-date with `main`** before merging. Fortunately, this project uses [Bors](https://github.com/bors-ng/bors-ng) to automatically enforce this requirement without the PR author having to rebase manually.

## Release Process (for internal team only)

Meilisearch tools follow the [Semantic Versioning Convention](https://semver.org/).

### Automation to rebase and Merge the PRs <!-- omit in toc -->

This project integrates a bot that helps us manage pull requests merging.<br>
_[Read more about this](https://github.com/meilisearch/integration-guides/blob/main/resources/bors.md)._

### Automated changelogs <!-- omit in toc -->

This project integrates a tool to create automated changelogs: the [release-drafter](https://github.com/release-drafter/release-drafter/).

### How to Publish the Release <!-- omit in toc -->

Make a PR modifying the file [`Cargo.toml`](/Cargo.toml) with the right version.

```toml
version = "X.X.X"
```

Once the changes are merged on `main`, you can publish the current draft release via the [GitHub interface](https://github.com/meilisearch/charabia/releases): on this page, click on `Edit` (related to the draft release) > update the description if needed > when you are ready, click on `Publish release`.

<hr>

Thank you again for reading this through, we can not wait to begin to work with you if you made your way through this contributing guide ❤️
