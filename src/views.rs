use cursive::views::Button;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::Cursive;
use cursive::View;

pub struct CreateWorldMenu {
    pub size: WorldSize,
}

impl Default for CreateWorldMenu {
    fn default() -> Self {
        CreateWorldMenu {
            size: WorldSize::Normal,
        }
    }
}

impl CreateWorldMenu {
    pub fn menu(&mut self, siv: &mut Cursive) {
        siv.add_layer(
            Dialog::around(LinearLayout::vertical().child()))
                .button("Cancel", |s| {
                    s.pop_layer();
                })
                .button("Continue", |_s| {}),
        )
    }
}

pub enum WorldSize {
    Tiny,
    Small,
    Normal,
    Large,
    Huge,
}
