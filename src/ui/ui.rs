use crate::ui::widget::path_indicator::PathIndicator;
use crate::ui::widget::tab::Tab;
use crate::ui::Mrc;

struct UI {
    tab: Mrc<Tab>,
    path: Mrc<PathIndicator>,
}
