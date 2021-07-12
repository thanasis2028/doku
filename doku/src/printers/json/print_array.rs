mod expand_variants;

use super::*;

impl<'ty> Ctxt<'ty, '_> {
    pub fn print_array(&mut self, ty: &'ty Type) {
        if !self.mode.allows(ty.serializable, ty.deserializable) {
            return;
        }

        if let Some(example) = self.example() {
            // Using the `#[doku(example = ...)]` attribute, user can provide
            // hint either:
            //
            // ... for a single array's element:
            //   - example = "foo"
            //
            // ... or for the entire array:
            //   - example = "[foo, bar]"
            //
            // In the first case we automatically add square brackets and
            // indentation, whereas in the second one we just print whatever
            // user's provided - and that happens right here.
            if example.starts_with('[') {
                self.out.text(example);

                // Manually-provided examples always take priority over our
                // automatically inferred ones - so if someone has already
                // provided an example, there's nothing more to do here
                return;
            }
        }

        if self.inline {
            self.out.text("[ ");
        } else {
            self.out.line("[");
            self.out.inc_indent();
        }

        if !self.expand_variants(ty) {
            self.with_ty(ty).print();

            if self.inline {
                self.out.text(", /* ... */");
            } else {
                self.out.line(",");
                self.out.line("/* ... */");
            }
        }

        if self.inline {
            self.out.text(" ]");
        } else {
            self.out.dec_indent();
            self.out.text("]");
        }
    }
}
