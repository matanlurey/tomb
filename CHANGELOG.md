# Changelog

## 0.2.0

- Removed `Copy` trait from structs with mutable state (i.e. `*Die` structs).
- `Roll.roll` trait now takes a `&Rotate` instead of taking ownership:

  ```diff
  let d6 = D6::new();
  - let rd = roller.roll(d6);
  + let rd = roller.roll(&d6);
  assert_eq!(rd.value(), 3);
  ```

  _This was always the intended behavior, we just made a mistake :)_

- `SliceDie::with_position(position, element)` changed to `(element, position`):

  ```diff
  type GradeDie<'a> = SliceDie<'a, char, 5>;
  const GRADES: [char; 5] = ['A', 'B', 'C', 'D', 'F'];

  - let d = GradeDie::with_position(1, &GRADES);
  + let d = GradeDie::with_position(&GRADES, 1);
  assert_eq!(d.value(), &'B');
  ```

- Fixed a bug where negative rotations on `SliceDie` were treated as positive.

## 0.1.0

- Initial release.
