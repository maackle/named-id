# aliased-id

Utilities for making logs more readable by shortening and aliasing long identifiers.

## How it works

Any frequently occurring long ID can be replaced with a friendlier version, consisting of a short prefix of the ID, or a user-chosen alias, or both.

This is done by associating the `Debug` output of such an ID with an alias.
Then, *any* type which is known to contain instances of that ID can change its debug representation to use aliases instead of the full ID wherever it occurs.

Pretty-printing (`format!("{:#?}", x)`) is supported.

In order to be minimally invasive:
- implement `AliasedId` on your ID types
- any container type whose Debug output you want to modify, you can implement `ContainsAliases` on it, specifying the list of IDs which should be interpolated
- when actually producing Debug output, you have to specify that you want interpolation by wrapping your `ContainsAliases` type in the `Aliased<T: ContainsAliases>` wrapper struct. `Aliased` implements Debug which will interpolate any IDs with their aliases.
- Any type with a derived `Debug` impl which contains an `Aliased` value will always print that value using aliases. If that outer type is again wrapped in `Aliased`, it should include the aliased ids from any inner Aliased types along with any new aliased ids

Ideally there would be a proc macro to derive ContainsAliases, so that every type in a tree would be able to be wrapped in `Aliased`. For now, pick and choose strategically.