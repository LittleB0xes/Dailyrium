
# An Unexpected Journey

## Day 1
First step for my roguelike attempt. I choosed to try Bracketlib, instead of my favourite game engine, Tetra.

Today was the day of the project setup and the first steps of my Hero... He's alive (in a dark world)

## Day 2 
After a file structure refactoring, I created some usefull struct like `Element` to populate the world with some static element, then,
I started the world generation (first a unstructured random world) for my hero.

So, the map is ugly, with awfull color... But it's here.

Tomorrow, I would try to change the format of the terminal. The size and, over all, I will try to change the font !!

## Day 3
### Big changes...
Only 3 day and i decide to go back to tetra game engine (and left bracket-lib, which is also amazing but not fit to me)

Better sooner rather than later

### Other minor update
- Basic turn system
- Event based input manager
- Random map

I also try a new way to manage the element's propertie. I use a `Vec<Propertie>` where `Propertie` is an enum. And I use the `any` iterator to check what i want to check. Perhaps a bad idea... Wait and see (usually i use a bunch of properties in struct)

My hero can now hit the wall... Outch!

Some monsters are born ... they are alcoholic zombies (random walk with in-map checking)

... and a last minute bonus : adding an extended `put`function in `rltetra` (for glyph and color in one call)

## Day 4
- Now, player and living entities are not separated. The hero is treated the same as everyone else. no favoritism: everyone in the same `Vec <LivingEntity>`
- The question of switching to a `HashMap` rather than a `Vec` may be posed.
- Add a check propertie function for elements.

## Day 5
- Spawn some gold here
- Move from `Vec<Property>` to `HashMap<Property, PropertyValue> (need to be improved).It's pretty ugly to to write but seems to be more useful and powerful
- Pick (object) function... but, yet, no impact with inventory
- Refactoring with a `Level` struct, that embeds all level data. (why not a `Vec<Level>` later ?)
- Big cleanup : 21 warnings erased

## Day 6
A little module refactoring for the future... To let the game growing without constraint, and less friction

## Day 7
- Some project structure change... psychological instability.
- Create a `puppet_master` function for living entities management.
- Improve the move action management. Now, entity can't go on cell that is non crossable **AND** when the cell is occupied by another entity.
- Create a FOV. but i think it needs to be improved... Perhaps i will put it in the engine puppet master function (and add an seen property for living entity). Actually it's in main. At least, i can improve it with `any()` iterartor.

## Day 8
- Some optimization with `puppet_master` function, to avoid multiple iteration over `entities` Vec
- Fix turn 0 bug (black view)

Big question. My first approach, that consist to have one unique element struct and hold several specific properties in a hashmap seems to be a little source of headache for me...
I need to take another way...

I try to use a classical `HashMap<Property, i32>` and I will translate this i32 value in various ways (depends of use case).
Or a multipurpose integer data.

- Add basical inventory...

Too many question about structure this days, but nothing concrete...

# Day 9
I decided to keep a very simple and multipurpose technic : an classical `u64` data variable(like a bit field). The use depends of the item.
No progress today... just questions, always questions...

- Add `Item` struct (to differentiate with `Element`)