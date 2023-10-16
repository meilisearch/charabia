# kvariants

A Rust crate wrapping https://github.com/hfhchan/irg/blob/master/kVariants.md made by @hfhchan.
If you want to participate in improving this dictionary,
don't hesitate to create an issue or submit a PR directly on the dictionary repository.

## Usage

```rs
use kvariants::KVARIANTS;

let c = '澚';

let kvariant = match KVARIANTS.get(&c) {
    Some(kvariant) => kvariant.destination_ideograph,
    None => c,
};

assert_eq!(kvariant, '澳');
```

## Fetch latest dictionary from upstream

The dictionary file is vendored into `dictionaries/source/` and can be updated with `bin/sync_dictionaries`.
