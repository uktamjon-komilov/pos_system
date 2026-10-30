# TODO

Open items, each with the condition that makes it worth doing.

- **Cards rebuilt on every tap** (only if the UI feels slow). Selecting a
  dish refreshes the whole list, and selecting or toggling a modifier
  replaces the whole modifier grid. Update only the affected rows instead.
- **Variants may open off-screen** (only if reproduced). Opening a dish with
  variants from far down the list does not scroll back to the top, so the
  variants list may start out of view.
- **Error enum lives in the meals section** (when a second section needs
  errors). `strings.slint` imports `MealsError` from `sections/meals/`. Move
  the enum to a shared file such as `ui/errors.slint`.

## Later

- A transport error variant in `RepositoryError` and a connection-error
  screen, once the real API client exists.
- Cooking times of 60 minutes or more read "90 час": a bug copied from
  legacy on purpose.
- A virtualised list instead of plain repeaters, if menus reach thousands of
  dishes.
