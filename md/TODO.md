# Afmt
To-Do:

- resolve idempotent issue (it's in battle_test.sh and also added use-cases into the three tests)
- add test folders into idempotent list
- to-do folder
- create benchmark to run locally
- binary exp comments
- remove all the "pub" properties in struct/enum

## Big items:

- as I don't use precise group(), challenge: how to avoid some line-comment new Doc
  variant so line-comment doesn't need to calculate a newline anymore?

Other:
- should bodymember not handle 1-2 newline(), rather let the code or comment to
  handle it? (one place to handle it all, it's better?)

- check Dang's logic
- chain method comment, tests in to-do folder
- design newline handling that's not coupled?


## ToDo
- Doc::Text needs to check precedding space, fits() doesn't check the same.
  alternatives? change `b.txt(" ")` to `b.try_space()` to tell the conditional
  adding space
- field_access with super or this
- Check what Enum size too big?
