# named-id

Utilities for making logs more readable by shortening and renaming long identifiers.

## How it works

Any frequently occurring long ID can be replaced with a friendlier version, consisting of a short prefix of the ID, or a user-chosen name, or both.

This is done by associating the `Debug` output of such an ID with a name.
Then, *any* type which is known to contain instances of that ID can change its debug representation to use names instead of the full ID wherever it occurs.

Pretty-printing (`format!("{:#?}", x)`) is supported.

In order to be minimally invasive:
- implement `Nameable` on your ID types
- any container type whose Debug output you want to modify, you can implement `Nameables` on it, specifying the list of IDs which should be interpolated
- when actually producing Debug output, you have to specify that you want interpolation by wrapping your `Nameables` type in the `Renamed<T: Nameables>` wrapper struct. `Renamed` implements Debug which will interpolate any IDs with their names.
- Any type with a derived `Debug` impl which contains a `Renamed` value will always print that value using names. If that outer type is again wrapped in `Renamed`, it should include the named ids from any inner Renamed types along with any new named ids

Ideally there would be a proc macro to derive Nameables, so that every type in a tree would be able to be wrapped in `Renamed`. For now, pick and choose strategically.