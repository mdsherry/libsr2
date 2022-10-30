
# Building blocks

Saves are built of five major components: primitive values, like `i32s` or timestamps; strings; arrays; maps; and objects.

## Primitives

The two most commmon primitive values are `i32`s and `f32s`. All values are encoded [little-endian](https://en.wikipedia.org/wiki/Endianness). In addition to these, there are also some 64-bit values: 

* In-game times are recorded as `f64`s, and measure the number of in-game seconds since midnight of day 1. (Note that the game starts at 9 AM, or 32400 seconds.) One in-game hour equals one real-world minute, and dying or sleeping in your house will also advance time.
* Save files include a summary, to show when choosing which save to load. This summary also includes the time when the save file was written. This 64-bit integer timestamp uses a unit of tenths of microsecond (100s of nanoseconds), and an epoch of midnight January 1, 1 CE under the [proleptic Gregorian Calendar](https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar) (i.e. the date you would get if you counted back to 1/1/1 using current calendar rules for leap years, etc.)
* The game records when various events first happened, and most recently happened. These events include things like entering or leaving areas, dying, and collecting items. The timestamps are recorded both as in-game time, using the aforementioned format, as well as real-world time. Like the format used to store when a save was made, this time stamp is also a 64-bit integer measuring 10ths of microseconds. The epoch used for this one, however, is midnight of January 1, 1601. This same format is used by [Microsoft Windows](https://devblogs.microsoft.com/oldnewthing/20090306-00/?p=18913). I have no idea why they didn't use this format for save files as well.

On top of that, booleans are encoded as a single byte with a value of `0x00` (false) or `0x01` (true).

## Strings

Strings are serialized by first writing a byte containing their length, followed by the bytes of the string itself. One consequence is that no string can be longer than 255 characters, but it did make deciphering the format very straight-forward. (This format is often known as a Pascal string, and as someone who once wrote a lot of Turbo Pascal, the 255 byte limit is very familiar.)

I don't know what encoding the strings use: UTF-8? [ISO 8859-1](https://en.wikipedia.org/wiki/ISO/IEC_8859-1)? Strict ASCII? Since there's no user-provided text in the save file, I can't experiment to find out.

## Arrays

Arrays are encoded by first writing the length of the array as a 32-bit integer, followed by the array elements in sequence. Decoding is thus straight-forward, provided one can figure out the type of value in the array.

## Maps

Like arrays, maps are preceded by a 32-bit integer encoding its length. In this case, the length is the total number of key/value pairs, rather than values written. Unlike arrays, after each key/value pair, the byte sequence `0x00 0x20` is written, making maps easier to recognize in a hex dump.

## Objects

Objects open with a string containing the object's name, using the string encoding described above. This is followed by a 32-bit integer containing, I presume, a version number, followed by the byte sequence `0x00 0x30`, and then the object's fields. The end of the object is indicated by the byte sequence `0x00 0x40`.

Most objects have a version of number of 1. Two have a version number of 2, and one has a version number of 0. (It also has an empty name.)

Although the `0x00 0x30` and `0x00 0x40` markers make it easy to guess where an object starts and ends, an object could contain the either sequence as e.g. part of an integer, so one can't use the markers to do simple bracket-matching style parsing. On the other hand, if you've finished parsing the fields of an object and there *isn't* a close marker immediately following, you know that you've made a mistake.

Relatedly, the save file format is not self-describing: there is nothing encoding the number of fields an object has, or their types. Writing a decoder involves a lot of guess work and testing hypotheses. Even then, it's impossible to distinguish between an integer (or float) that's always 0 from an array or map that's empty in every sample file you've looked at.

Object names are always written in capital letters, and with two exceptions their names begin with `SR`. Some names have obvious meanings (`SRGAME`, `SRRANCH`). Others are more opaque (`SRPL` stores player data; `SRV3` is a 3-float vector). Others are still a mystery to me (e.g. `SRRCD`, which seems to relate to vegetable and plant growth and decay.)

# The format itself

Here are the formats of the objects, to the best of my ability to decode them so far. One limiting factor is that I only have access to my nearly complete save file, and two recently just-started ones. As such, I don't have any saves representing mid-game.

Slime Rancher 1 appears to have a very similar save format, and reverse engineering that would probably provide some insight into the Slime Rancher 2 one, but this has sucked up enough of my time already.

The entire save is an `SRGAME` object.

## `SRGAME`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Identifiable Type Index | `SRITI` |
| Garden patch index | `SRRGI` | R-something Garden Index? |
| Game icon index | `SRGAMEICONINDEX` | Sometimes the object names do make things easy |
| Zone index | `SRZONEINDEX` | |
| Game name | String | This is the base name of the save, and is comprised of the time (down to seconds) that the game was started, followed by an underscore and then which save slot this corresponds to. E.g. `20221020134518_2` is the game I started on October 20, 2022, in the second game slot.|
| Game slot? | String | This appears to just be the game slot again |
| ??? | i32 | Possibly the slot number again, but this time encoded as an integer and 0-indexed? |
| Game summary | `SRGSUMM` | Data shown to the user on the load screen |
| Game settings | `SRGAMESETTINGS` |
| Scene group index | `SRSGI` | 
| World | `SRW` | Data concerning the world at large |
| Player | `SRPL` | Player data, including the refinery inventory |
| Ranch | `SRRANCH` | Data about the ranch and its plots |
| ??? | Two bytes | Or maybe two bools? `0x00 0x10` in both of the games I've looked at. It might also be a control code (similar to `0x00 0x20`, `0x00 0x30`, and `0x00 0x40`) that I haven't deciphered yet.
| Actors | `SRAD`[] | State data for all 'actors': slimes, hens, botanicals, containers, and plorts |
| Discoveries | `SRPED` | Player E-something Discoveries? PEDagological opportunities? |
| ??? | `SRAPP` | |
| Secret style discs | `SRSECRETSTYLEDISC` | I don't think SR2 has these yet? |
| Upgrade component index | `SRUCI` | |
| ??? index | `SRSEI` | Just contains a pair of hex strings |

`SRGAME` is large, but there are very few mysterious byte sequences.

## `SRV3`: Vector (3D)
| Name        |Type    | Notes |
|-------------------------|---------|----|
| x | `f32` | |
| y | `f32` | |
| z | `f32` | |

Axes might not actually be in that order.

## Indexes

By embedding an index into the save file mapping numerical IDs to strings, it makes it easier for Monomi Park to reorder, etc. IDs used by the game itself.

It also made my life reverse-engineering the file format a bit easier since I can look up values in these indices and see if the results make sense. Usually you can guess which index to use based on the range of the values: there are around 480 entries in `SRITI`, and the other indices are much smaller.

### `SRITI`: Identifiable Type Index

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Identifiable types | `String[]` |

 
Type names are all in the format of `Category.Specifier`, e.g. `SlimeDefinition.Pink` or `IdentifiableType.Hen`.

### `SRRGI`: Ranch growable index?

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Garden patch type | `String[]` |

All names start with `patch` (for vegetables) or `tree` (for fruit), e.g. `patchbeet01` or `treepear01`.

Curiously, there are some plants in the index that appeared in SR1 but not yet in SR2.

### `SRGAMEICONINDEX`: Game icon index

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Game icon | `String[]` |

Maps which game icon is used to represent the save. All icons are slimes.

### `SRZONEINDEX`: Zone index

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Zone name | `String[]` |

I haven't found yet where these are being used

### `SRSGI`: Scene group index

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Scene group | `String[]` |

All scene group names start with `SceneGroup.`.

### `SRUCI`: Upgrade component index

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Upgrade components | `String[]` |

All upgrade component names start with `UpgradeComponent.`.

### `SRSEI`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| ??? | `String[]` |

I'm not even sure this is an index, and if so what of. There are only two entries, and both are just opaque hexadecimal strings.

## `SRGSUMM`: Game summary

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Build | `String` | The build number matching the one on the title screen |
| In-game time | In-game time (`f64`) | |
| Save file timestamp | Save file time (`i64`) | The one place with an epoch of 1/1/1 |
| Money | `i32` | Current player wealth |
| Discoveries | `i32` | Number of slimepedia entries |
| ??? | 23 bytes | Probably various game settings, e.g. tarrs enabled, ferals enabled, but I haven't deciphered them all yet
| Game icon | `i32` | References `SRGAMEICONINDEX` |

Summarized save info so that browsing the load-game screen doesn't require fully parsing each save.

## `SRGAMESETTINGS`: Game settings

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Game settings | `StringPair[]` | "StringPair" being my name for the anonymous object this actually uses |
| Game icon | `i32` | References `SRGAMEICONINDEX` |

This is the only place where the anonymous object is used: it has a 0-length name, a version number of 0, and contains two strings. I don't know why a map wasn't used instead here. Legacy reasons?

The game settings are all stored as strings, e.g a key of `setting.GameIcon` mapping a string `14`, or `setting.FeralEnabled` mapping to `on`. Curiously, this means that game icon is stored twice, first as a string and then later as an integer.

## `SRW`: World

| Name        |Type    | Notes |
|-------------------------|---------|----|
| World time | In-game time | Current in-game clock |
| ??? | `i32`/`f32`? | Number varies betweens save slots, but seems to remain steady in any given game. As an integer, it always seems to be a 10 digit number starting with `122`. As a float, values I've seen range from to 467392.46875 to 670484.0|
| ??? | `f64`? | Always seems to be `inf` |
| ??? | `f64`? | Always seems to be 0 |
| ??? | In-game time? | If so, it always seems to point to a time on the first day around or shortly after noon. |
| ??? | `i32`? | Always 0 |
| Plort market saturation | `f32[i32]` | Keys are references to the item index; values are the number of plorts the market has left unsold: unsold plorts suppress plort prices. |
| Hen spawn timers? | `In-game time[SRV3]` | The locations indicated by the keys seem to correspond to nest locations, although some are also floating in the air, e.g. in one of the ranch extensions. This extension also has a bunch of hens wandering around. I think the values are when the spawner last ran. In a new game, they're all 0. |
| Liquid sources | `i32[String]` | Keys all start with `LiquidSource` then a number. Values are all zero. |
| Slime spawn timers? | `In-game time[SRV3]` | Similar to hen-spawn timers, but for slimes |
| Gordos | `SRG[String]` | Gordo state data. Keys are `gordo` followed by a 10-digit number |
| ??? | `SRRW[SRV3]` | ??? |
| Player-placed gadgets | `SRPG[String]` | Keys are `site` followed by a 10-digit number |
| Pods | `SRTP[String]` | Keys are of the form `pod` followed by a 10-digit number |
| Switches | `i32[String]` | Keys are of the form `pod` followed by a 10-digit number. Value is current state? |
| Puzzles | `bool[String]` | Keys are `puz` followed by a 10-digit number. Value is whether the puzzle is solved? |
| ??? | `i32` | Always 0, so could be an empty map or array |
| Research drones | `SRREDRONE[String]` | Keys are descriptive, e.g. `ResearchDroneGully`. |
| ??? | `i32` | Again, always 0 |
| Resource nodes | `SRRESNODE[String]` | Keys are `resource_node` followed by a 10-digit number |

There are still a lot of mysteries here. Some I'm unravelling by mapping the SRV3s alongside the various actors, visualizing it, and using that to find the world coordinates they refer to.

### `SRG`: Gordo

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Fed | `i32` | Units of food fed to the Gordo. Favourite foods count double. When the Gordo is fully fed and bursts, this value is -1. |
| ??? | `i32` | Always 0? |
| Found | `bool` | Has the player discovered this gordo? |
| Position | `SRV3` | Position in world coordinates |
| Facing | `SRV3` | [Rotation around the x, y and z axes in degrees](https://en.wikipedia.org/wiki/Gimbal_lock) |
| Gordo type | `i32` | References item index |

Gordos only appear on this list when the player has discovered the zone they live in.

### `SRRW`: ???

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Time stamp? | In-game time | |
| ??? | `i32`? | |

This is still a mystery. Timestamps are 'recent', so it's probably another spawn time, but with some additional state attached. The value is usually (but not always) 0, which makes it harder to decypher.

### `SRPG`: Player gadget
| Name        |Type    | Notes |
|-------------------------|---------|----|
| Item ID | `i32` | References item index |
| Actor ID | `i32` | References actor list in `SRGAME` |
| ??? | `i32` | Location ID? |
| Angle | `f32` | Placement angle in degrees |
| ??? | 32 bytes | Not yet decoded; could contain, e.g. inventory of warp depots, or which gadget is this one's sibling |
| Scene group ID | `i32` | References scene group index; where in the world is this gadget? |
| ??? | 5 bytes | |
| Position? | `SRV3` | |

Still more work to go to figure this one out.

### `SRTP`: Treasure pod

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Opened? | `i32` | 0 or 1 |
| ??? | `i32` | Always 0 |

Data on treasure pods

### `SRREDRONE`: Research drone

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Discovered | `bool` | |
| Name | `String` | |

### `SRRESNODE`: Resource node

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Name | `String` | |
| ??? | `i32` | |
| Timestamp | In-game time | A future time. When the node will despawn? |
| Resource type | `String` | Often blank |
| ??? | `i32` | |
| Contents | `i32[]` | References item index. |

## `SRPL`: Player

| Name        |Type    | Notes |
|-------------------------|---------|----|
| HP | `i32` | Current HP, not max |
| Energy | `i32` | Current energy, not max |
| ??? | `i32` | |
| Money | `i32` | |
| ??? | `i32` | |
| Total money earned | `i32` | |
| Build | `String` | |
| Position | `SRV3` | |
| Facing | `SRV3` | |
| Upgrades | `SRPU` | |
| Upgrade components | `SRUPGRADECOMPONENTS` | |
| Inventory | `SRAD[]` | All simple-variant `SRAD`s |
| Gadgets unlocked | `i32[]` | References item index |
| ??? | 4 bytes | |
| Refinery contents | `i32[i32]` | Keys are item indices, values are counts. Refinery contents is not just inputs (plorts) but also unbuilt gadgets and decorations |
| ??? | `bool` | |
| ??? | 8 bytes | |
| ??? | `bool` | |
| Decorizer | `SRDZR` | Not yet in SR2, but purpose discovered from looking at SR1 saves |

### `SRPU`: Player upgrades

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Purchased upgrades? | `i32[i32]` | Keys are all multiples of 16. Values start at -1 for unpurchased, and then increase by 1 for each level purchased. |

Yes, at this point I could have just cheated my way past the bug and stopped.

### `SRUPGRADECOMPONENTS`: Upgrade components

| Name        |Type    | Notes |
|-------------------------|---------|----|
| ??? | 4 bytes | |

In my most advanced game, I've spent all my components. In my other games, I haven't collected any yet. As such, I don't know what this looks like. (Is it 4 bools?)

### `SRDZR`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| ??? | 8 bytes | Always seem to be 0s |

## `SRRANCH`: Ranch

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Plots | `SRLP[]` | All plots, not just accessible ones |
| Doors | `i32[String]` | Has the player unlocked the door to the ranch extension yet? Keys are of the form `door` followed by a 10-digit number |
| ??? | `i32[i32]` | Keys are all distinct integers from 0 to 9. Values are 0, 21, 1000, 1001, and 1002, and don't seem to vary. |
| ??? | `In-game Time[String]` | Keys are all `ranch` followed by a 10-digit number. Values are timestamps: in a new game, there are 4 ranches, and they're all 0d 9:00:00. In a more established game, the timestamps update, and a 5th ranch is added. Last visited? |

### `SRLP`: Plot

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Ran feeder | In-game time | When was the auto-feeder (if any) last run? 0 if there is no autofeeder. |
| ??? | 8 bytes | |
| Ran vaccuum | In-game time | When was the plort/hen vaccuum last run? |
| ??? | 8 bytes | Always 0s |
| Plot type | `i32` | 1 = empty, 2 = slime corral, 3 = hen coop, 4 = garden, 5 = silo, 6 = pond?, 7 = incinerator |
| Plant type | `i32` | References garden patch index |
| Name | `String` | `plot` followed by, you guessed it, a 10 digit number |
| Upgrades | `i32[]` | Which upgrades have been purchased for the plot, e.g. sun-shield (13)? |
| Plot inventories | `SRAD[][String]` | A map of strings to SRAD arrays. SRADs are the simple variant. Strings draw from a fixed pool, and indicate the *type* of inventory: silo, plort, food, or aged fowl. |
| ??? | `i32[]` | The length is always 4, and values default to 0, but sometimes are 1 or 2 instead. |
| Incinerator pit fullness? | f32 | |
| ??? | `bool` | |
| Tracked actor list | `TRACKEDACTORLIST` | |

### `TRACKEDACTORLIST`: Tracked actor list

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Actor IDs | `f64[]` | Complex SRAD variants, as used in the Actors field of `SRGAME`, have an index field. Although that field is an `i32`, we store them here as an `f64` and I have no idea why. |

## `SRAD`: Actor

There are two variants of `SRAD`: a simple one for inactive actors, stored in inventories or silos; and a more complex one for actors that can interact with the world.

#### Simple `SRAD`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Item ID | `i32` | References item index |
| Count | `i32` | How many there are |
| ??? | `i32` | |
| ??? | `SRSED` | |

#### Complex `SRAD`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Position | `SRV3` | |
| Facing | `SRV3` | |
| Index | i32 | Monotonically incrementing ID used to reference the actor, independent of its position in this list. Counting starts at 100. |
| ??? | `SRSED` | |
| Chick timer | In-game time | Possibly counts time until chick matures? NaN for grown fowl, and 0 for non-fowl. |
| Hen timer | In-game timer | Possibly counts time until the hen tries to lay a new egg? NaN for chicks and roosters, and 0 for non-fowl. |
| Plort despawn | In-game time | When will a plort despawn from age. 0 for non-plorts |
| Plant growth data? | `SRRCD` | |
| Has extra timestamp | `bool` | Is there an extra timestamp immediately after this, growing this object by an extra 8 bytes? Only set on slimes, but not all slimes. |
| ??? | In-game time | Only set on some slimes, and references a past time. |
| Feral | `bool` | Is this a feral slime? |
| ??? | 5 bytes | All zero |
| Scene group ID | `i32` | Where in the world is this actor? References scene group index |
| Petrified | `bool` | If this is a ringtail or ringtail largo, is it currently petrified by the sun? If so, it's not rendered, and a statue is put in its place. |
| Unpetrified actor for statue | `i32` | For ringtail statues, when night comes, which actor should it unpetrify to? |
| ??? | 4 bytes | |
| ??? | In-game time | Set on non-slimes, 0 otherwise. Spawning date? |
| ??? | `SRSE[]` |

### `SRSED`
| Name        |Type    | Notes |
|-------------------------|---------|----|
| ??? | `f32[i32]` | Keys are 0, 1 and 2 but rarely saved in that order. Hunger, and some other metrics? |

This seems to be slime-specific.

### `SRRCD`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Condition | `i32` | 0, 1 or 2. I think this indicates growing, ripe and decayed, not neccesarily in that order. |
| ??? | In-game time | Future time. When to make the next state transition? |

### `SRSE`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| ??? | 4 bytes | Always 0|
| Timestamp | ??? | Sometimes future, sometimes past |

I don't even see these in recently started games.

## `SRPED`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| ??? | 4 bytes | always 0; could be an empty array? |
| Discoveries | `String[]` | Array of discovery names |

## `SRAPP`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| ??? | `(i32, i32)[i32]` | Keys are references to the item index. Values might actually be single-element arrays, instead, all containing just the number 1. |
| ??? | `i32[i32]` | Keys are references to the item index. Values are all 1. |

In both fields, the keys enumerate the different non-largo slimes.

## `SRSECRETSTYLEDISC`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| ??? | 4 bytes | Empty array? |


## `SREVENTRECORD`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Events | `SREVENTENTRY[]` | |

## `SREVENTENTRY`

| Name        |Type    | Notes |
|-------------------------|---------|----|
| Category | `String` | Category of event |
| Subcategory | `String` | |
| Count | `i32` | How many times has this event occurred? |
| Entry created (IGT) | In-game time | When did this event first occur |
| Last updated (IGT) | In-game time | When did this event last occur |
| Entry created (wall time) | Real-world time | 1601 epoch |
| Last updated (wall time) | Real-world time | 1601 epoch |
